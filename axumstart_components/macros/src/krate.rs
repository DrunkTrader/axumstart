//! Resolves the token path generated code should use to reach `axumstart_components`'
//! public API — through the `axumstart` meta-crate re-export (`axumstart::components`) if
//! the consumer depends on that, or directly (`axumstart_components`) otherwise. Checked at
//! macro-expansion time against the *consuming* crate's `Cargo.toml`, so the same derive
//! works whether someone depends on `axumstart` or `axumstart_components` directly.

use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn path() -> TokenStream {
  if let Ok(FoundCrate::Name(name)) = crate_name("axumstart") {
    let meta = Ident::new(&name, Span::call_site());
    return quote!(::#meta::components);
  }

  match crate_name("axumstart_components") {
    Ok(FoundCrate::Itself) => quote!(crate),
    Ok(FoundCrate::Name(name)) => {
      let ident = Ident::new(&name, Span::call_site());
      quote!(::#ident)
    }
    Err(_) => quote!(::axumstart_components),
  }
}
