// This module implements the Throws folder.
//
// The Throws folder actually visits the item being processed and performs two
// processes:
// - It ok wraps return expressions and inserts terminal Ok(())s.
// - It delegates return type rewriting to the Args type.

use proc_macro::TokenStream;
use syn::fold::Fold;

use crate::Args;

pub struct Throws {
    args: Option<Args>,
    outer_fn: bool,
    return_type: syn::Type,
}

impl Throws {
    pub fn new(args: Option<Args>) -> Throws {
        Throws {
            args,
            outer_fn: true,
            return_type: syn::parse_quote!(()),
        }
    }

    pub fn fold(&mut self, input: TokenStream) -> TokenStream {
        if let Ok(item_fn) = syn::parse(input.clone()) {
            let item_fn = self.fold_item_fn(item_fn);
            quote::quote!(#item_fn).into()
        } else if let Ok(impl_item_fn) = syn::parse(input.clone()) {
            let impl_item_fn = self.fold_impl_item_fn(impl_item_fn);
            quote::quote!(#impl_item_fn).into()
        } else if let Ok(trait_item_fn) = syn::parse(input.clone()) {
            let trait_item_fn = self.fold_trait_item_fn(trait_item_fn);
            quote::quote!(#trait_item_fn).into()
        } else if let Ok(expr_closure) = syn::parse(input.clone()) {
            let expr_closure = self.fold_expr_closure(expr_closure);
            quote::quote!(#expr_closure).into()
        } else if let Ok(expr_async) = syn::parse(input) {
            let expr_async = self.fold_expr_async(expr_async);
            quote::quote!(#expr_async).into()
        } else {
            panic!("#[throws] attribute can only be applied to functions, methods, closures or async blocks")
        }
    }
}

impl Fold for Throws {
    fn fold_item_fn(&mut self, i: syn::ItemFn) -> syn::ItemFn {
        if !self.outer_fn {
            return i;
        }

        let sig = syn::Signature {
            output: self.fold_return_type(i.sig.output),
            ..i.sig
        };

        self.outer_fn = false;

        let inner = self.fold_block(*i.block);
        let block = Box::new(make_fn_block(&self.return_type, &inner));

        syn::ItemFn { sig, block, ..i }
    }

    fn fold_impl_item_fn(&mut self, i: syn::ImplItemFn) -> syn::ImplItemFn {
        if !self.outer_fn {
            return i;
        }

        let sig = syn::Signature {
            output: self.fold_return_type(i.sig.output),
            ..i.sig
        };

        self.outer_fn = false;

        let inner = self.fold_block(i.block);
        let block = make_fn_block(&self.return_type, &inner);

        syn::ImplItemFn { sig, block, ..i }
    }

    fn fold_trait_item_fn(&mut self, mut i: syn::TraitItemFn) -> syn::TraitItemFn {
        if !self.outer_fn {
            return i;
        }

        let sig = syn::Signature {
            output: self.fold_return_type(i.sig.output),
            ..i.sig
        };

        self.outer_fn = false;

        let default = i.default.take().map(|block| {
            let inner = self.fold_block(block);
            make_fn_block(&self.return_type, &inner)
        });

        syn::TraitItemFn { sig, default, ..i }
    }

    fn fold_expr_closure(&mut self, i: syn::ExprClosure) -> syn::ExprClosure {
        if !self.outer_fn {
            return i;
        }

        let output = match i.output {
            syn::ReturnType::Default => syn::parse_quote!(-> _),
            output => output,
        };
        let output = self.fold_return_type(output);

        self.outer_fn = false;

        let inner = self.fold_expr(*i.body);
        let body = Box::new(make_fn_expr(&self.return_type, &inner));

        syn::ExprClosure { output, body, ..i }
    }

    fn fold_expr_async(&mut self, i: syn::ExprAsync) -> syn::ExprAsync {
        if !self.outer_fn {
            return i;
        }

        // update self.return_type
        let _ = self.fold_return_type(syn::parse_quote!(-> _));
        self.outer_fn = false;

        let inner = self.fold_block(i.block);
        let block = make_fn_block(&self.return_type, &inner);

        syn::ExprAsync { block, ..i }
    }

    fn fold_return_type(&mut self, i: syn::ReturnType) -> syn::ReturnType {
        if !self.outer_fn {
            return i;
        }
        let return_type = match &mut self.args {
            Some(args) => args.ret(i),
            None => i,
        };
        let ty = match &return_type {
            syn::ReturnType::Type(_, ty) => (**ty).clone(),
            syn::ReturnType::Default => syn::Type::Infer(syn::parse_quote!(_)),
        };
        struct ImplTraitToInfer;
        impl Fold for ImplTraitToInfer {
            fn fold_type(&mut self, i: syn::Type) -> syn::Type {
                match i {
                    syn::Type::ImplTrait(_) => syn::Type::Infer(syn::parse_quote!(_)),
                    i => syn::fold::fold_type(self, i),
                }
            }
        }
        self.return_type = ImplTraitToInfer.fold_type(ty);
        return_type
    }

    fn fold_expr_return(&mut self, i: syn::ExprReturn) -> syn::ExprReturn {
        let ok = match &i.expr {
            Some(expr) => ok(&self.return_type, expr),
            None => ok_unit(&self.return_type),
        };
        syn::ExprReturn {
            expr: Some(Box::new(ok)),
            ..i
        }
    }
}

fn make_fn_block(ty: &syn::Type, inner: &syn::Block) -> syn::Block {
    let mut block: syn::Block = syn::parse2(quote::quote! {{
        #[allow(clippy::diverging_sub_expression)]
        {
            let __ret = #inner;

            #[allow(unreachable_code)]
            <#ty as ::culpa::__internal::_Succeed>::from_ok(__ret)
        }
    }})
    .unwrap();
    block.brace_token = inner.brace_token;
    block
}

fn make_fn_expr(ty: &syn::Type, inner: &syn::Expr) -> syn::Expr {
    syn::parse2(quote::quote! {{
        #[allow(clippy::diverging_sub_expression)]
        {
            let __ret = { #inner };

            #[allow(unreachable_code)]
            <#ty as ::culpa::__internal::_Succeed>::from_ok(__ret)
        }
    }})
    .unwrap()
}

fn ok(ty: &syn::Type, expr: &syn::Expr) -> syn::Expr {
    syn::parse2(quote::quote!(<#ty as ::culpa::__internal::_Succeed>::from_ok(#expr))).unwrap()
}

fn ok_unit(ty: &syn::Type) -> syn::Expr {
    syn::parse2(quote::quote!(<#ty as ::culpa::__internal::_Succeed>::from_ok(()))).unwrap()
}
