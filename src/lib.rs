use proc_macro::{bridge::server::Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
// const (
// 	ClauseType_Where         = "WHERE"
// 	ClauseType_Limit         = "LIMIT"
// 	ClauseType_Offset        = "OFFSET"
// 	ClauseType_OrderBy       = "ORDER BY"
// 	ClauseType_GroupBy       = "GROUP BY"
// 	ClauseType_InnerJoin     = "INNER JOIN"
// 	ClauseType_LeftJoin      = "LEFT JOIN"
// 	ClauseType_RightJoin     = "RIGHT JOIN"
// 	ClauseType_FullOuterJoin = "FULL OUTER JOIN"
// 	ClauseType_Select        = "SELECT"
// 	ClauseType_Having        = "HAVING"
// )
enum ClauseType {
    Where,
    Limit,
    Offset,
    OrderBy,
    GroupBy,
    InnerJoin,
    RightJoin,
    LeftJoin,
    FullOuterJoin,
    Select,
    Having,
}

impl std::fmt::Display for ClauseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClauseType::Where => write!(f, "WHERE"),
            ClauseType::Limit => write!(f, "LIMIT"),
            ClauseType::Offset => write!(f, "OFFSET"),
            ClauseType::OrderBy => write!(f, "ORDER BY"),
            ClauseType::GroupBy => write!(f, "GROUP BY"),
            ClauseType::InnerJoin => write!(f, "INNER JOIN"),
            ClauseType::RightJoin => write!(f, "RIGHT JOIN"),
            ClauseType::LeftJoin => write!(f, "LEFT JOIN"),
            ClauseType::FullOuterJoin => write!(f, "FULL OUTER JOIN"),
            ClauseType::Select => write!(f, "SELECT"),
            ClauseType::Having => write!(f, "HAVING"),
        }
    }
}
struct Clause {
    ty: ClauseType,
    arg: Vec<String>,
    delimiter: String,
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.delimiter == "" {
            self.delimiter = " ".to_string();
        }
        write!(f, "{} {}", self.ty, self.arg.join(&self.delimiter))
    }
}
struct BaseQueryBuilder {}

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
