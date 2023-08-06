use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn write_date_statement(field_ident: &Ident, meta_name: &String) -> TokenStream {
    let string_ident = field_ident.to_string();
    let start_ident = Ident::new(&format!("{}_start", string_ident), Span::call_site());
    let end_ident = Ident::new(&format!("{}_end", string_ident), Span::call_site());

    let inner_field = inner_field_statement(meta_name);
    let inner_start = inner_start_statement(meta_name);
    let inner_end = inner_end_statement(meta_name);
    let inner_start_end_statement = inner_start_end_statement(meta_name);

    return quote! {
        if self.#field_ident.is_some() {
            #inner_field
        }

        if self.#start_ident.is_some() && self.#end_ident.is_some() {
            #inner_start_end_statement
        } else if self.#start_ident.is_some() {
            #inner_start
        } else if self.#end_ident.is_some() {
            #inner_end
        }
    };
}
pub fn write_option_date_statement(field_ident: &Ident, meta_name: &String) -> TokenStream {
    let string_ident = field_ident.to_string();
    let start_ident = Ident::new(&format!("{}_start", string_ident), Span::call_site());
    let end_ident = Ident::new(&format!("{}_end", string_ident), Span::call_site());

    let inner_field = inner_field_statement(meta_name);
    let inner_start = inner_start_statement(meta_name);
    let inner_end = inner_end_statement(meta_name);
    let inner_start_end_statement = inner_start_end_statement(meta_name);
    let inner_else_statement = inner_else_null_statement(meta_name);

    return quote! {
        if self.#field_ident.is_some() {
            let value = self.#field_ident.as_ref().unwrap();
            if value.is_some() {
                #inner_field
            }#inner_else_statement
        }
        if self.#start_ident.is_some() && self.#end_ident.is_some() {
            let start = self.#start_ident.as_ref().unwrap();
            let end = self.#end_ident.as_ref().unwrap();

            if start.is_some() && end.is_some() {
                #inner_start_end_statement
            } else if start.is_some() {
                #inner_start
            } else if end.is_some() {
                #inner_end
            }
        }else if self.#start_ident.is_some() {
            let start = self.#start_ident.as_ref().unwrap();
            if start.is_some() {
                #inner_start
            }#inner_else_statement
        }else if self.#end_ident.is_some() {
            let end = self.#end_ident.as_ref().unwrap();
            if end.is_some() {
                #inner_end
            }#inner_else_statement
        }
    };
}

pub fn write_date_apply_block(field_ident: &Ident) -> TokenStream {
    let string_ident = field_ident.to_string();
    let start_ident = Ident::new(&format!("{}_start", string_ident), Span::call_site());
    let end_ident = Ident::new(&format!("{}_end", string_ident), Span::call_site());
    quote! {
        if self.#field_ident.is_some() {
            let value = self.#field_ident.unwrap();
            query = query.bind(value);
        }
        if self.#start_ident.is_some() && self.#end_ident.is_some() {
            let start = self.#start_ident.unwrap();
            let end = self.#end_ident.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        }
        else if self.#start_ident.is_some() {
            query = query.bind(self.#start_ident.unwrap());
        }
        else if self.#end_ident.is_some() {
            query = query.bind(self.#end_ident.unwrap());
        }
    }
}

pub fn write_option_date_apply_block(field_ident: &Ident) -> TokenStream {
    let string_ident = field_ident.to_string();
    let start_ident = Ident::new(&format!("{}_start", string_ident), Span::call_site());
    let end_ident = Ident::new(&format!("{}_end", string_ident), Span::call_site());
    quote! {
        if self.#field_ident.is_some() {
            let value = self.#field_ident.as_ref().unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.#start_ident.is_some() && self.#end_ident.is_some() {
            let start = self.#start_ident.as_ref().unwrap();
            let end = self.#end_ident.as_ref().unwrap();
            if start.is_some() && end.is_some() {
                query = query.bind(start.unwrap());
                query = query.bind(end.unwrap());
            } else if start.is_some() {
                query = query.bind(start.unwrap());
            } else if end.is_some() {
                query = query.bind(end.unwrap());
            }
        }else if self.#start_ident.is_some() {
            let start = self.#start_ident.as_ref().unwrap();
            if start.is_some() {
                query = query.bind(start.unwrap());
            }
        }else if self.#end_ident.is_some() {
            let end = self.#end_ident.as_ref().unwrap();
            if end.is_some() {
                query = query.bind(end.unwrap());
            }
        }
    }
}

fn inner_field_statement(meta_name: &String) -> TokenStream {
    return quote! {
      param_count = param_count + 1;
      if and{
          sql.push_str(" AND ");
      }
      and = true;
      sql.push_str(&format!("{} = ${}", #meta_name, param_count));
    };
}

fn inner_start_statement(meta_name: &String) -> TokenStream {
    return quote! {
      param_count = param_count + 1;
      if and{
          sql.push_str(" AND ");
      }
      and = true;
      sql.push_str(&format!("{} >= ${}", #meta_name, param_count));
    };
}

fn inner_end_statement(meta_name: &String) -> TokenStream {
    return quote! {
      param_count = param_count + 1;
      if and{
          sql.push_str(" AND ");
      }
      and = true;
      sql.push_str(&format!("{} <= ${}", #meta_name, param_count));
    };
}

fn inner_start_end_statement(meta_name: &String) -> TokenStream {
    return quote! {
      param_count = param_count + 2;
      if and{
          sql.push_str(" AND ");
      }
      and = true;
      sql.push_str(&format!("{} BETWEEN ${} AND ${}", #meta_name, param_count - 1, param_count));
    };
}

fn inner_else_null_statement(meta_name: &String) -> TokenStream {
    return quote! {else{
        if !and {
          sql.push_str(&format!("{} IS NULL", #meta_name));
        } else {
          sql.push_str(&format!(" AND {} IS NULL", #meta_name));
        }
        and = true;
      }
    };
}
