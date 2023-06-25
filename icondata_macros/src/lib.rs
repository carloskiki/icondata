extern crate proc_macro;

#[proc_macro]
pub fn icon(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = input.into_iter();

    let icon_ident = match input.next() {
        Some(token) => match token {
            proc_macro::TokenTree::Ident(i) => i,
            _ => panic!("Expected an identifier, but received a different token type."),
        },
        None => panic!("Expected an identifier, but received an empty token stream."),
    };
    // Ensure that there are no more tokens in the token stream
    if input.next().is_some() {
        panic!("Expected only one identifier, but received multiple tokens.");
    }
    let icon_string = icon_ident.to_string();

    let (lib_short_name, icon_name) = icon_string.split_at(2);

    format!(
        "Icon::from({}Icon::{}{})",
        lib_short_name, lib_short_name, icon_name
    )
    .parse()
    .unwrap()
}
