use quote::quote;
use syn::spanned::Spanned;

pub fn attr1_impl(
    _attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let struct_item: syn::ItemStruct =
        syn::parse2(input).map_err(|err| syn::Error::new(err.span(), "Expected struct."))?;

    let stream = quote!(
        #struct_item
    );

    Ok(stream)
}

pub fn list(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let metadata = ctx::load().map_err(|err| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to get ctx.\n{err}"),
        )
    })?;

    println!("macros::list!() using metadata: {:#?}", metadata);

    let values = metadata
        .structs
        .iter()
        .map(|value| syn::LitStr::new(&value, input.span()));

    let stream = quote!(
        fn list() -> Vec<&'static str> {
            vec![#(#values),*]
        }
    );

    Ok(stream)
}
