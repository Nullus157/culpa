// This module implements the Throws folder.
//
// The Throws folder actually visits the item being processed and performs two
// processes:
// - It ok wraps return expressions and inserts terminal Ok(())s.
// - It delegates return type rewriting to the Args type.

use proc_macro::*;
use syn::fold::Fold;

use crate::Args;

pub struct Throws {
    args: Args,
    outer_fn: bool,
    return_type: Box<syn::Type>,
}

impl Throws {
    pub fn new(args: Args) -> Throws {
        Throws {
            args,
            outer_fn: true,
            return_type: Box::new(syn::parse_quote!(())),
        }
    }

    pub fn fold(&mut self, input: TokenStream) -> TokenStream {
        if let Ok(item_fn) = syn::parse(input.clone()) {
            let item_fn = self.fold_item_fn(item_fn);
            quote::quote!(#item_fn).into()
        } else if let Ok(impl_item_fn) = syn::parse(input.clone()) {
            let impl_item_fn = self.fold_impl_item_fn(impl_item_fn);
            quote::quote!(#impl_item_fn).into()
        } else if let Ok(trait_item_fn) = syn::parse(input) {
            let trait_item_fn = self.fold_trait_item_fn(trait_item_fn);
            quote::quote!(#trait_item_fn).into()
        } else {
            panic!("#[throws] attribute can only be applied to functions and methods")
        }
    }
}

impl Fold for Throws {
    fn fold_item_fn(&mut self, i: syn::ItemFn) -> syn::ItemFn {
        if !self.outer_fn { return i; }

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
        if !self.outer_fn { return i; }

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
        if !self.outer_fn { return i; }

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
        i // TODO
    }

    fn fold_expr_async(&mut self, i: syn::ExprAsync) -> syn::ExprAsync {
        i // TODO
    }

    fn fold_return_type(&mut self, i: syn::ReturnType) -> syn::ReturnType {
        if !self.outer_fn { return i; }
        let return_type = self.args.ret(i);
        let syn::ReturnType::Type(_, ty) = &return_type else { unreachable!() };
        self.return_type = ty.clone();
        return_type
    }

    fn fold_expr_return(&mut self, i: syn::ExprReturn) -> syn::ExprReturn {
        let ok = match &i.expr {
            Some(expr)  => ok(&self.return_type, expr),
            None        => ok_unit(&self.return_type),
        };
        syn::ExprReturn { expr: Some(Box::new(ok)), ..i }
    }
}

fn make_fn_block(ty: &syn::Type, inner: &syn::Block) -> syn::Block {
    syn::parse2(quote::quote! {{
        let __ret = #inner;

        #[allow(unreachable_code)]
        <#ty as ::fehler::__internal::_Succeed>::from_ok(__ret)
    }}).unwrap()
}

fn ok(ty: &syn::Type, expr: &syn::Expr) -> syn::Expr {
    syn::parse2(quote::quote!(<#ty as ::fehler::__internal::_Succeed>::from_ok(#expr))).unwrap()
}

fn ok_unit(ty: &syn::Type) -> syn::Expr {
    syn::parse2(quote::quote!(<#ty as ::fehler::__internal::_Succeed>::from_ok(()))).unwrap()
}
