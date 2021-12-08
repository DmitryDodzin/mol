extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(ToBox)]
pub fn derive_as_box(tokens: TokenStream) -> TokenStream {
  let mut target_name = None;
  let mut token_iter = tokens.into_iter();

  while let Some(token) = token_iter.next() {
    if let TokenTree::Ident(ident) = token {
      let ident_str = ident.to_string();
      if ident_str == "struct" || ident_str == "enum" {
        target_name = token_iter.next().map(|token| token.to_string());

        break;
      }
    }
  }

  if let Some(name) = target_name {
    format!("impl ToBox for {} {{}}", name).parse().unwrap()
  } else {
    panic!("Must be derived on enum or struct")
  }
}
