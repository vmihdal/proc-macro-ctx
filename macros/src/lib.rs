mod act;

#[proc_macro_attribute]
pub fn attr1(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match act::attr1_impl(attr.into(), item.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn list(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match act::list(item.into()) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
