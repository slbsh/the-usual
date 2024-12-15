use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn the_usual(_: TokenStream, i: TokenStream) -> TokenStream {
	format!("#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]\n{i}").parse().unwrap()
}

