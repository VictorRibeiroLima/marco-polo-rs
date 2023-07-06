use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

mod utils;

#[proc_macro_derive(Paginate, attributes(order_field_name))]
pub fn ordination_enum(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Get the visibility and identifier of the struct
    let (visibility, ident, generics) =
        (derive_input.vis, derive_input.ident, derive_input.generics);

    let (_, ty_generics, where_clause) = generics.split_for_impl();

    // Create the enum name
    let enum_name = ident.to_string() + "OrderFields";
    let enum_ident = Ident::new(&enum_name, Span::call_site());

    // Create the derives
    let derives = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    };

    // Get the fields of the struct
    let struct_fields = match derive_input.data {
        syn::Data::Struct(data_enum) => utils::filter_fields(&data_enum.fields),
        _ => panic!("Pagination can only be derived for structs"),
    };
    if struct_fields.is_empty() {
        panic!("Pagination can only be derived for non-empty structures");
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
    let field_name_variants = struct_fields.iter().map(|(_, variant_ident, _)| {
        quote! {
            #variant_ident
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

        impl #ty_generics crate::database::queries::pagination::Paginator for #ident #ty_generics #where_clause {
            type E = #enum_ident;
        }
    };

    return tokens.into();
}
