use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};
fn is_ty(wrapper: String, ty: &syn::Type) -> bool {
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
fn is_numeric(ty: &syn::Type) -> bool {
    for t in vec![
        "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
    ] {
        if is_ty(t.to_string(), ty) {
            return true;
        }
    }
    return false;
}
// fn get_wheres_for_numerics(ident: &syn::Ident, ty: &syn::Type) -> TokenStream {}
#[proc_macro_derive(QueryBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let select_builder_name = format!("{}SelectBuilder", &ast.ident);
    let model_ident = name;
    let table_name = format!("\"{}s\"", model_ident.to_string());
    let select_builder_ident = syn::Ident::new(&select_builder_name, name.span());
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
    let eqs = fields.iter().map(|f| {
        let eq_ident = syn::Ident::new(
            format!("where_{}_eq", (&f.ident).as_ref().unwrap()).as_str(),
            f.span(),
        );
        let ty = &f.ty;
        let cond = format!("{}={{}}", (&f.ident).as_ref().unwrap());
        quote! {
            pub fn #eq_ident(&mut self, arg: #ty) -> &mut Self {
                self.builder._where(format!(#cond, arg));
                self
            }
        }
    });

    let query_builder = quote! {
        impl #model_ident {
            pub fn select(&mut self) -> #select_builder_ident {
                return #select_builder_ident::new();
            }
        }
        struct #select_builder_ident {
            builder: query_builder_engine::SelectBuilder
        }

        impl #select_builder_ident {
            #(#eqs)*
            pub fn build(&mut self) -> String {
                self.builder.build()
            }
            pub fn table(&mut self, t: String) -> &mut Self {
                self.builder.table(t);
                self
            }
            pub fn new() -> Self {
                #select_builder_ident {
                    builder: query_builder_engine::SelectBuilder::new()
                }
            }
        }
    };

    query_builder.into()
}
