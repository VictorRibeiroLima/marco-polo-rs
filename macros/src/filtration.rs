use deluxe::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Type};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(filtrate))]
struct FiltrationFieldAttributes {
    name: Option<String>,
    #[deluxe(default = false)]
    skip: bool,
}

pub fn gen_filtration_block(input: TokenStream) -> Result<TokenStream> {
    //parse
    let mut derive_input: DeriveInput = syn::parse2(input)?;

    let struct_fields = extract_metadata_field_attrs(&mut derive_input);
    if struct_fields.is_empty() {
        panic!("Pagination can only be derived for non-empty structures");
    };

    // Get the visibility and identifier of the struct
    let (visibility, ident, generics) =
        (derive_input.vis, derive_input.ident, derive_input.generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Create the struct name
    let struct_name = ident.to_string() + "Filters";
    let struct_ident = Ident::new(&struct_name, Span::call_site());

    // Create the derives
    let derives = quote! {
        #[derive(Debug, Clone, PartialEq, Default, serde::Deserialize, serde::Serialize)]
    };

    let fields = create_struct_fields(&struct_fields);

    let filter_fields_block = create_filter_fields_block(&struct_fields);

    let apply_block = create_apply_block(&struct_fields);

    let where_block = create_where_block(&struct_fields);

    /*

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct VideoFilters {
        pub id: Option<Uuid>,
        pub title: Option<String>,
        pub description: Option<String>,
        pub user_id: Option<i32>,
        pub channel_id: Option<i32>,
        pub url: Option<String>,
        pub language: Option<String>,
        pub stage: Option<VideoStage>,
        pub created_at: Option<NaiveDateTime>,
        pub updated_at: Option<NaiveDateTime>,
        pub deleted_at: Option<NaiveDateTime>,
        pub uploaded_at: Option<NaiveDateTime>,
    }

    impl VideoFilters {
        pub fn new() -> Self {
            Self {
                id: None,
                title: None,
                description: None,
                user_id: None,
                channel_id: None,
                url: None,
                language: None,
                stage: None,
                created_at: None,
                updated_at: None,
                deleted_at: None,
                uploaded_at: None,
            }
        }
    }

    impl FilterableOptions for VideoFilters {
        fn filter_fields(&self) -> Vec<&'static str> {
            let mut fields: Vec<&'static str> = vec![];
            if self.id.is_some() {
                fields.push("id");
            };
            return fields;
        }

        fn apply<O>(
            self,
            query: sqlx::query::QueryAs<'_, Postgres, O, PgArguments>,
        ) -> sqlx::query::QueryAs<'_, Postgres, O, PgArguments> {
            let mut query = query;
            if self.id.is_some() {
                query = query.bind(self.id.unwrap());
            };
            return query;
        }

        fn apply_raw(
            self,
            query: sqlx::query::Query<'_, Postgres, PgArguments>,
        ) -> sqlx::query::Query<'_, Postgres, PgArguments> {
            let mut query = query;
            if self.id.is_some() {
                query = query.bind(self.id.unwrap());
            };
            return query;
        }
    }
     */

    let tokens = quote! {
        #derives
        #visibility struct #struct_ident{
            #(#fields,)*
        }

        impl #impl_generics crate::database::queries::filter::FilterableOptions for #struct_ident #ty_generics #where_clause {
            fn filter_fields(&self) -> Vec<&'static str> {
                let mut fields: Vec<&'static str> = vec![];
                #(#filter_fields_block)*
                return fields;
            }

            fn apply<O>(self, mut query: sqlx::query::QueryAs<'_, sqlx::Postgres,O,sqlx::postgres::PgArguments>) -> sqlx::query::QueryAs<'_, sqlx::Postgres,O,sqlx::postgres::PgArguments> {
                #(#apply_block)*
                return query;
            }

            fn apply_raw(self, mut query: sqlx::query::Query<'_, sqlx::Postgres,sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres,sqlx::postgres::PgArguments> {
                #(#apply_block)*
                return query;
            }

            fn gen_where_statements(&self, param_count: Option<usize>) -> (String, usize) {
                let mut sql = String::new();

                let mut param_count = match param_count {
                    Some(param_count) => param_count,
                    None => 0,
                };

                #(#where_block)*

                return (sql, param_count);

            }
        }
    };
    Ok(tokens)
}

fn extract_metadata_field_attrs(derive_input: &mut DeriveInput) -> Vec<(Ident, Type, String)> {
    let mut vec = Vec::new();

    if let syn::Data::Struct(s) = &mut derive_input.data {
        for field in s.fields.iter_mut() {
            match field.vis {
                syn::Visibility::Public(_) => {
                    let attrs: FiltrationFieldAttributes = deluxe::extract_attributes(field)
                        .unwrap_or(FiltrationFieldAttributes {
                            name: None,
                            skip: false,
                        });

                    let field_type = field.ty.clone();
                    let field_ident = field.ident.as_ref().unwrap().clone();

                    let meta_name = match attrs.name {
                        Some(name) => name,
                        None => field_ident.to_string(),
                    };

                    if attrs.skip {
                        continue;
                    }
                    vec.push((field_ident, field_type, meta_name))
                }
                _ => continue,
            }
        }
    } else {
        panic!("Filtration can only be derived for structs");
    }

    return vec;
}

/* Create the struct fields

Example:
    struct Foo {
        bar: i32,
        baz: String,
    }

    struct FooFilters {
        this lines--> pub bar: Option<i32>,
        this lines--> pub baz: Option<String>,
    }
*/
fn create_struct_fields(struct_fields: &Vec<(Ident, Type, String)>) -> Vec<TokenStream> {
    struct_fields
        .iter()
        .map(|(field_ident, field_type, _)| {
            quote! {
                pub #field_ident: Option<#field_type>
            }
        })
        .collect()
}

/* Create the struct filter_fields block

Example:
     fn filter_fields(&self) -> Vec<&'static str> {
        let mut fields: Vec<&'static str> = vec![];
                  ---> if self.id.is_some() {
    THIS BLOCK    --->   fields.push("id");
                  ---> };
        return fields;
    }
*/
fn create_filter_fields_block(struct_fields: &Vec<(Ident, Type, String)>) -> Vec<TokenStream> {
    struct_fields
        .iter()
        .map(|(field_ident, _, meta_name)| {
            quote! {
                if self.#field_ident.is_some() {
                    fields.push(#meta_name);
                }
            }
        })
        .collect()
}

/* Create the struct filter_fields block

Example:
     fn filter_fields(&self) -> Vec<&'static str> {
        let mut fields: Vec<&'static str> = vec![];
                  ---> if self.id.is_some() {
    THIS BLOCK    --->   query = query.bind(self.id.unwrap());
                  ---> };
        return fields;
    }
*/
fn create_apply_block(struct_fields: &Vec<(Ident, Type, String)>) -> Vec<TokenStream> {
    struct_fields
        .iter()
        .map(|(field_ident, _, _)| {
            quote! {
                if self.#field_ident.is_some() {
                  query = query.bind(self.#field_ident.unwrap());
                }
            }
        })
        .collect()
}

fn create_where_block(struct_fields: &Vec<(Ident, Type, String)>) -> Vec<TokenStream> {
    struct_fields
        .iter()
        .map(|(field_ident, field_type, meta_name)| {
            let is_option = check_if_type_is_option(field_type);

            if is_option {
                return quote! {
                    param_count = param_count + 1;
                    if self.#field_ident.is_some() {
                        let value = self.#field_ident.as_ref().unwrap();
                        match value {
                            Some(_) => {
                                if param_count == 1 {
                                    sql.push_str(&format!("{} = ${}", #meta_name, param_count));
                                } else {
                                    sql.push_str(&format!(" AND {} = ${}", #meta_name, param_count));
                                }
                            }
                            None => {
                                if param_count == 1 {
                                    sql.push_str(&format!("{} IS NULL", #meta_name));
                                } else {
                                    sql.push_str(&format!(" AND {} IS NULL", #meta_name));
                                }
                            }
                        }
                    }
                };
            }
            return quote! {
                param_count = param_count + 1;
                if self.#field_ident.is_some() {
                    if param_count == 1 {
                        sql.push_str(&format!("{} = ${}", #meta_name, param_count));
                    } else {
                        sql.push_str(&format!(" AND {} = ${}", #meta_name, param_count));
                    }
                }
            };
        })
        .collect()
}

fn check_if_type_is_option(field_type: &Type) -> bool {
    if let Type::Path(ref type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident.to_string() == "Option" {
                return true;
            }
        }
    }
    return false;
}
