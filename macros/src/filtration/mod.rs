use deluxe::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, GenericArgument, PathArguments, Type};

mod date;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(filtrate))]
struct FiltrationFieldAttributes {
    name: Option<String>,
    #[deluxe(default = false)]
    skip: bool,
}

enum FieldType {
    Option,
    String,
    Date,
    NonSpecial,
}

pub fn gen_filtration_block(input: TokenStream) -> Result<TokenStream> {
    //parse
    let mut derive_input: DeriveInput = syn::parse2(input)?;

    let struct_fields = extract_metadata_field_attrs(&mut derive_input);
    if struct_fields.is_empty() {
        panic!("Filtration can only be derived for non-empty structures");
    };

    // Get the visibility and identifier of the struct
    let (visibility, ident, generics) =
        (derive_input.vis, derive_input.ident, derive_input.generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ident_string = ident.to_string();
    // Create the struct name
    let struct_name = format!("InternalFiltration{}Filters", ident_string);
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
        #visibility struct #struct_ident #impl_generics #where_clause {
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

                let mut param_count:usize = match param_count {
                    Some(param_count) => param_count,
                    None => 0,
                };

                let mut and = false;

                #(#where_block)*

                return (sql, param_count);

            }
        }

        impl #impl_generics crate::database::queries::filter::Filterable for #ident #ty_generics #where_clause {
            type F = #struct_ident #ty_generics;
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
            let f_type = check_type(field_type);

            let deserialize_with = match f_type {
                FieldType::Option => "crate::database::queries::filter::filtration_from_str_option",
                _ => "crate::database::queries::filter::filtration_from_str",
            };

            /*
               String -> Option<String>
               Option<String> -> Option<Option<String>
            */
            let primary_field: TokenStream = quote! {
                #[serde(
                    default,                                    // <- important for deserialization
                    skip_serializing_if = "Option::is_none",    // <- important for serialization
                    deserialize_with = #deserialize_with       // <- important for deserialization
                )]
                pub #field_ident: Option<#field_type>
            };

            match f_type {
                FieldType::Date => {
                    let between_fields =
                        create_date_between_fields(field_ident, field_type, deserialize_with);
                    quote! {
                        #primary_field,
                        #between_fields
                    }
                }
                FieldType::Option => {
                    let inner_type = get_option_inner_type(field_type);
                    match inner_type {
                        FieldType::Date => {
                            let between_fields = create_date_between_fields(
                                field_ident,
                                field_type,
                                deserialize_with,
                            );
                            quote! {
                                #primary_field,
                                #between_fields
                            }
                        }
                        FieldType::Option => {
                            panic!("Nested options are not supported")
                        }
                        _ => primary_field,
                    }
                }
                _ => primary_field,
            }
        })
        .collect()
}

fn create_date_between_fields(
    field_ident: &Ident,
    field_type: &Type,
    deserialize_with: &str,
) -> TokenStream {
    let string_ident = field_ident.to_string();
    let start_ident = Ident::new(&format!("{}_start", string_ident), Span::call_site());
    let end_ident = Ident::new(&format!("{}_end", string_ident), Span::call_site());
    quote! {
        #[serde(
            default,                                    // <- important for deserialization
            skip_serializing_if = "Option::is_none",    // <- important for serialization
            deserialize_with = #deserialize_with       // <- important for deserialization
        )]
        pub #start_ident: Option<#field_type>,

        #[serde(
            default,                                    // <- important for deserialization
            skip_serializing_if = "Option::is_none",    // <- important for serialization
            deserialize_with = #deserialize_with        // <- important for deserialization
        )]
        pub #end_ident: Option<#field_type>
    }
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
        .map(|(field_ident, field_type, _)| {
            let inner_type = check_type(field_type);
            match inner_type {
                FieldType::Option => {
                    let inner_type = get_option_inner_type(field_type);
                    match inner_type {
                        FieldType::Date => date::write_option_date_apply_block(field_ident),
                        FieldType::Option => panic!("Nested options are not supported"),
                        _ => {
                            quote! {
                                if self.#field_ident.is_some() {
                                    let value = self.#field_ident.unwrap();
                                    if value.is_some() {
                                        query = query.bind(value.unwrap());
                                    }
                                }
                            }
                        }
                    }
                }
                FieldType::Date => date::write_date_apply_block(field_ident),
                _ => {
                    quote! {
                        if self.#field_ident.is_some() {
                          query = query.bind(self.#field_ident.unwrap());
                        }
                    }
                }
            }
        })
        .collect()
}

fn create_where_block(struct_fields: &Vec<(Ident, Type, String)>) -> Vec<TokenStream> {
    struct_fields
        .iter()
        .map(|(field_ident, field_type, meta_name)| {
            write_where_block(field_ident, field_type, meta_name)
        })
        .collect()
}

fn write_where_block(field_ident: &Ident, field_type: &Type, meta_name: &String) -> TokenStream {
    let where_type = check_type(field_type);

    match where_type {
        FieldType::Option => create_where_block_option(field_ident, field_type, meta_name),
        FieldType::String => create_where_block_string(field_ident, meta_name),
        FieldType::Date => create_where_block_date(field_ident, meta_name),
        FieldType::NonSpecial => create_where_block_non_special(field_ident, meta_name),
    }
}

fn create_where_block_option(
    field_ident: &Ident,
    field_type: &Type,
    meta_name: &String,
) -> TokenStream {
    let inner_type = get_option_inner_type(field_type);
    match inner_type {
        FieldType::String => {
            let statement = write_string_statement(meta_name);
            quote! {

                if self.#field_ident.is_some() {
                    let value = self.#field_ident.as_ref().unwrap();
                    match value {
                        Some(_) => {
                            param_count = param_count + 1;
                           #statement
                        }
                        None => {
                            if !and {
                                sql.push_str(&format!("{} IS NULL", #meta_name));
                            } else {
                                sql.push_str(&format!(" AND {} IS NULL", #meta_name));
                            }
                            and = true;
                        }
                    }
                }
            }
        }
        FieldType::NonSpecial => {
            let statement = write_non_special_statement(meta_name);
            quote! {

                if self.#field_ident.is_some() {
                    let value = self.#field_ident.as_ref().unwrap();
                    match value {
                        Some(_) => {
                            param_count = param_count + 1;
                           #statement
                        }
                        None => {
                            if !and {
                                sql.push_str(&format!("{} IS NULL", #meta_name));
                            } else {
                                sql.push_str(&format!(" AND {} IS NULL", #meta_name));
                            }
                            and = true;
                        }
                    }
                }
            }
        }
        FieldType::Date => date::write_option_date_statement(field_ident, meta_name),
        FieldType::Option => {
            panic!("Nested option types are not supported")
        }
    }
}

fn create_where_block_string(field_ident: &Ident, meta_name: &String) -> TokenStream {
    let string_statement = write_string_statement(meta_name);
    return quote! {

        if self.#field_ident.is_some() {
            param_count = param_count + 1;
            #string_statement
        }
    };
}

fn create_where_block_date(field_ident: &Ident, meta_name: &String) -> TokenStream {
    let date_statement = date::write_date_statement(field_ident, meta_name);
    return quote! {

        #date_statement
    };
}

fn write_string_statement(meta_name: &String) -> TokenStream {
    return quote! {
        if !and {
            sql.push_str(&format!("{} LIKE ${}", #meta_name, param_count));
        } else {
            sql.push_str(&format!(" AND {} LIKE ${}", #meta_name, param_count));
        }
        and = true;
    };
}

fn create_where_block_non_special(field_ident: &Ident, meta_name: &String) -> TokenStream {
    let non_special_statement = write_non_special_statement(meta_name);
    return quote! {
        if self.#field_ident.is_some() {
            param_count = param_count + 1;
            #non_special_statement
        }
    };
}

fn write_non_special_statement(meta_name: &String) -> TokenStream {
    return quote! {
        if !and  {
            sql.push_str(&format!("{} = ${}", #meta_name, param_count));
        } else {
            sql.push_str(&format!(" AND {} = ${}", #meta_name, param_count));
        }
        and = true;
    };
}

fn check_type(field_type: &Type) -> FieldType {
    if let Type::Path(ref type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            let ident_string = segment.ident.to_string();
            if ident_string == "Option" {
                return FieldType::Option;
            } else if ident_string == "String" {
                return FieldType::String;
            } else if ident_string.contains("Date") {
                return FieldType::Date;
            }
        }
    }
    return FieldType::NonSpecial;
}

fn get_option_inner_type(field_type: &Type) -> FieldType {
    match field_type {
        Type::Path(ref type_path) => {
            let path_segment = type_path.path.segments.last().unwrap();
            match path_segment.arguments {
                PathArguments::AngleBracketed(ref args) => {
                    let generic_arg = args.args.first().unwrap();
                    match generic_arg {
                        GenericArgument::Type(ref inner_type) => {
                            return check_type(inner_type);
                        }
                        _ => {
                            panic!("Only option types are supported")
                        }
                    }
                }
                _ => {
                    panic!("Only option types are supported")
                }
            };
        }
        _ => {
            panic!("Only option types are supported")
        }
    }
}
