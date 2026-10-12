//! Resolves the token path generated code should use to reach `axumstart_router`'s public
//! API — through the `axumstart` meta-crate re-export (`axumstart::router`) if the consumer
//! depends on that, or directly (`axumstart_router`) otherwise. Checked at macro-expansion time
//! against the *consuming* crate's `Cargo.toml`, so the same attribute works whether someone
//! depends on `axumstart` or `axumstart_router` directly.

use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn path() -> TokenStream {
  if let Ok(FoundCrate::Name(name)) = crate_name("axumstart") {
    let meta = Ident::new(&name, Span::call_site());
    return quote!(::#meta::router);
  }

  match crate_name("axumstart_router") {
    Ok(FoundCrate::Itself) => quote!(crate),
    Ok(FoundCrate::Name(name)) => {
      let ident = Ident::new(&name, Span::call_site());
      quote!(::#ident)
    }
    Err(_) => quote!(::axumstart_router),
  }
}
