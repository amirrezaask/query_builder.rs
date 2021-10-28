use proc_macro::{bridge::server::Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
mod base;
fn is_option(wrapper: String, ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = &ty
    {
        let last_seg = segments.first().unwrap();
        if last_seg.ident == wrapper {
            true
        } else {
            false
        }
    } else {
        false
    }
}
fn extract_inner(ty: syn::Type) -> syn::Type {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = &ty
    {
        let last_seg = segments.first().unwrap();
        if last_seg.ident == "Option" {
            if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) = &last_seg.arguments
            {
                if let syn::GenericArgument::Type(t) = args.first().unwrap() {
                    return t.to_owned();
                }
                ty
            } else {
                ty
            }
        } else {
            ty
        }
    } else {
        ty
    }
}
#[proc_macro_derive(QueryBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}QueryBuilder", &ast.ident);
    let ident = name;
    let bident = syn::Ident::new(&bname, name.span());
    let fields = {
        if let syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) = ast.data
        {
            named
        } else {
            unimplemented!()
        }
    };
    let query_builder = quote! {
        struct #bident {

        }
    };

    query_builder.into()
}
