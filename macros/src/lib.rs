use proc_macro::TokenStream;
mod filtration;
mod pagination;

#[proc_macro_derive(Paginate, attributes(paginate))]
pub fn paginate(input: TokenStream) -> TokenStream {
    let pagination_block = pagination::gen_pagination_block(input.into()).unwrap();
    pagination_block.into()
}

#[proc_macro_derive(Filtrate, attributes(filtrate))]
pub fn filter(input: TokenStream) -> TokenStream {
    let filtration_block = filtration::gen_filtration_block(input.into()).unwrap();
    filtration_block.into()
}
