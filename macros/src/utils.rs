use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span};
use syn::Fields;

pub fn filter_fields(fields: &Fields) -> Vec<(Ident, Ident)> {
    fields
        .iter()
        .filter_map(|field| {
            if field.ident.is_some() {
                let field_ident = field.ident.as_ref().unwrap().clone();

                let field_name = field.ident.as_ref().unwrap().to_string();
                let variant_ident =
                    Ident::new(&field_name.to_upper_camel_case(), Span::call_site());

                Some((field_ident, variant_ident))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
