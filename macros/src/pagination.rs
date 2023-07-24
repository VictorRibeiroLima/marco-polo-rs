use deluxe::Result;
use heck::ToUpperCamelCase;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(paginate))]
struct PaginationFieldAttributes {
    name: Option<String>,
}

pub fn gen_pagination_block(input: TokenStream) -> Result<TokenStream> {
    // Parse the input tokens into a syntax tree
    let mut derive_input: DeriveInput = syn::parse2(input)?;

    // Get the fields of the struct
    let struct_fields = extract_metadata_field_attrs(&mut derive_input);
    if struct_fields.is_empty() {
        panic!("Pagination can only be derived for non-empty structures");
    };

    // Get the visibility and identifier of the struct
    let (visibility, ident, generics) =
        (derive_input.vis, derive_input.ident, derive_input.generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Create the enum name
    let enum_name = ident.to_string() + "OrderFields";
    let enum_ident = Ident::new(&enum_name, Span::call_site());

    // Create the derives
    let derives = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Default, Eq, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
    };

    /* Create the enum variants

    Example:
        struct Foo {
            bar: i32,
            baz: String,
        }

        enum FooOrderFields {
            this lines--> Bar,
            this lines--> Baz,
        }
    */
    let field_name_variants = struct_fields
        .iter()
        .enumerate()
        .map(|(i, (_, variant_ident, _))| {
            if i == 0 {
                quote! {
                    #[default]
                    #variant_ident
                }
            } else {
                quote! {
                    #variant_ident
                }
            }
        });

    /* Create the match arms

    Example:
        struct Foo {
            bar: i32,
            baz: String,
        }

        enum FooOrderFields {
            Bar,
            Baz,
        }

        impl FooOrderFields {
            fn name(&self) -> &'static str {
                match *self {
                this lines-->    FooPagination::Bar => "bar",
                this lines-->    FooPagination::Baz => "baz",
                }
            }
        }
    */
    let field_name_to_strs = struct_fields
        .iter()
        .map(|(field_ident, variant_ident, meta_name)| {
            // If the attribute #[order_field_name = "field_name"] is not present, the field name will be used
            let field_name = if let Some(meta_name) = meta_name {
                meta_name.to_string()
            } else {
                field_ident.to_string()
            };

            quote! {
                #enum_ident::#variant_ident => #field_name
            }
        });

    let tokens = quote! {
        #derives
        #visibility enum #enum_ident{
            #(#field_name_variants,)*
        }

        impl #enum_ident {
            #visibility fn name(&self) -> &'static str {
                match *self {
                    #(#field_name_to_strs),*
                }
            }
        }

        impl #impl_generics crate::database::queries::pagination::Paginator for #ident #ty_generics #where_clause {
            type E = #enum_ident;
        }
    };

    return Ok(tokens);
}

fn extract_metadata_field_attrs(
    derive_input: &mut DeriveInput,
) -> Vec<(Ident, Ident, Option<String>)> {
    let mut vec = Vec::new();

    if let syn::Data::Struct(s) = &mut derive_input.data {
        for field in s.fields.iter_mut() {
            let attrs: PaginationFieldAttributes = deluxe::extract_attributes(field)
                .unwrap_or(PaginationFieldAttributes { name: None });

            let meta_name = attrs.name;
            let field_ident = field.ident.as_ref().unwrap().clone();
            let field_name = field.ident.as_ref().unwrap().to_string();
            let variant_ident = Ident::new(&field_name.to_upper_camel_case(), Span::call_site());
            vec.push((field_ident, variant_ident, meta_name))
        }
    } else {
        panic!("Pagination can only be derived for structs");
    }

    return vec;
}
