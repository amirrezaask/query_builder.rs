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
    let table_name = format!("{}s", model_ident.to_string().to_lowercase());
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
        let ty = extract_inner(f.ty.clone());
        let cond = format!("{}={{}}", (&f.ident).as_ref().unwrap());
        quote! {
            pub fn #eq_ident(&mut self, arg: #ty) -> &mut Self {
                self.builder._where(format!(#cond, arg));
                self
            }
        }
    });

    let comparisons = fields.iter().filter_map(|f| {
        let inner_ty = extract_inner(f.ty.clone());
        if !is_numeric(&inner_ty) {
            return None;
        }
        let le_ident = syn::Ident::new(
            format!("where_{}_le", (&f.ident).as_ref().unwrap()).as_str(),
            f.span(),
        );

        let ge_ident = syn::Ident::new(
            format!("where_{}_ge", (&f.ident).as_ref().unwrap()).as_str(),
            f.span(),
        );

        let gt_ident = syn::Ident::new(
            format!("where_{}_gt", (&f.ident).as_ref().unwrap()).as_str(),
            f.span(),
        );

        let lt_ident = syn::Ident::new(
            format!("where_{}_lt", (&f.ident).as_ref().unwrap()).as_str(),
            f.span(),
        );
        let ty = extract_inner(f.ty.clone());
        let le_cond = format!("{}<={{}}", (&f.ident).as_ref().unwrap());
        let ge_cond = format!("{}>={{}}", (&f.ident).as_ref().unwrap());
        let gt_cond = format!("{}>{{}}", (&f.ident).as_ref().unwrap());
        let lt_cond = format!("{}<{{}}", (&f.ident).as_ref().unwrap());

        Some(quote! {
            pub fn #le_ident(&mut self, arg: #ty) -> &mut Self {
                self.builder._where(format!(#le_cond, arg));
                self
            }
            pub fn #ge_ident(&mut self, arg: #ty) -> &mut Self {
                self.builder._where(format!(#ge_cond, arg));
                self
            }
            pub fn #gt_ident(&mut self, arg: #ty) -> &mut Self {
                self.builder._where(format!(#gt_cond, arg));
                self
            }
            pub fn #lt_ident(&mut self, arg: #ty) -> &mut Self {
                self.builder._where(format!(#lt_cond, arg));
                self
            }
        })
    });

    let null_checks = fields.iter().filter_map(|f| {
        if !is_ty("Option".to_string(), &f.ty) {
            return None;
        }
        let inner_ty = extract_inner(f.ty.clone());
        let null_check_name = syn::Ident::new(
            format!("where_{}_null", (&f.ident).as_ref().unwrap()).as_str(),
            f.span(),
        );
        let not_null_check_name = syn::Ident::new(
            format!("where_{}_not_null", (&f.ident).as_ref().unwrap()).as_str(),
            f.span(),
        );
        let null_cond = format!("{} is NULL", (&f.ident).as_ref().unwrap());
        let not_null_cond = format!("{} is NOT NULL", (&f.ident).as_ref().unwrap());

        Some(quote! {
            pub fn #null_check_name(&mut self) -> &mut Self{
                self.builder._where(format!(#null_cond));
                self
            }
            pub fn #not_null_check_name(&mut self) -> &mut Self{
                self.builder._where(format!(#not_null_cond));
                self
            }
        })
    });
    let query_builder = quote! {
        impl #model_ident {
            pub fn select() -> #select_builder_ident {
                return #select_builder_ident::new();
            }
        }
        struct #select_builder_ident {
            builder: query_builder_engine::SelectBuilder
        }

        impl #select_builder_ident {
            #(#eqs)*
            #(#comparisons)*
            #(#null_checks)*
            pub fn build(&mut self) -> String {
                self.builder.build()
            }
            pub fn table(&mut self, t: String) -> &mut Self {
                self.builder.table(t);
                self
            }
            pub fn new() -> Self {
                let mut builder = query_builder_engine::SelectBuilder::new();
                builder.table(#table_name.to_string());
                #select_builder_ident {
                    builder: builder
                }
            }
        }
    };

    query_builder.into()
}
