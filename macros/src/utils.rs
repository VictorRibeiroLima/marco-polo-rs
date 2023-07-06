use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span};
use syn::{Expr, Field, Fields, Meta};

pub fn filter_fields(fields: &Fields) -> Vec<(Ident, Ident, Option<String>)> {
    fields
        .iter()
        .filter_map(|field| {
            if field.ident.is_some() {
                let meta_name = meta_name(field);

                let field_ident = field.ident.as_ref().unwrap().clone();

                let field_name = field.ident.as_ref().unwrap().to_string();

                let variant_ident =
                    Ident::new(&field_name.to_upper_camel_case(), Span::call_site());

                Some((field_ident, variant_ident, meta_name))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

// This function returns the name of the field if it has the attribute #[order_field_name = "field_name"]
pub fn meta_name(field: &Field) -> Option<String> {
    //Creating the expected attribute ident for comparison
    let lib_ident = Some(Ident::new("order_field_name", Span::call_site()));

    let attrs = &field.attrs;
    for attr in attrs {
        let meta = &attr.meta;

        //Getting the ident of the attribute
        let ident = meta.path().get_ident();

        //Comparing the attribute ident with the expected one
        if lib_ident.as_ref() == ident {
            //Getting the expr of the attribute and erroring if it's wrongfully used
            let value = match meta {
                Meta::NameValue(ref name_value) => &name_value.value,

                _ => {
                    panic!("Wrong use of attribute, expected #[order_field_name = \"field_name\"]")
                }
            };

            //Getting the literal of the attribute and erroring if it's wrongfully used
            let lit_str = match value {
                Expr::Lit(ref lit_str) => &lit_str.lit,
                _ => {
                    panic!("Wrong use of attribute, expected #[order_field_name = \"field_name\"]")
                }
            };

            //Getting the string of the attribute and erroring if it's wrongfully used
            let meta_name = match lit_str {
                syn::Lit::Str(ref lit_str) => lit_str.value(),
                _ => {
                    panic!("Wrong use of attribute, expected #[order_field_name = \"field_name\"]")
                }
            };

            return Some(meta_name);
        }
    }
    return None;
}
