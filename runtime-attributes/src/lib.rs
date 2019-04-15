//! Proc Macro attributes for the [Runtime](https://github.com/rustasync/runtime) crate. See the
//! [Runtime](https://docs.rs/runtime) documentation for more details.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![feature(async_await, await_macro, futures_api)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

/// Defines the async main function.
///
/// # Examples
///
/// ```
/// #![feature(async_await, futures_api)]
///
/// #[runtime::main]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let rt = if attr.is_empty() {
        syn::parse_str("runtime_native::Native").unwrap()
    } else {
        syn::parse_macro_input!(attr as syn::Expr)
    };
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.decl.output;
    let name = &input.ident;
    let body = &input.block;

    if name != "main" {
        let tokens = quote_spanned! { name.span() =>
          compile_error!("only the main function can be tagged with #[runtime::main]");
        };
        return TokenStream::from(tokens);
    }

    if input.asyncness.is_none() {
        let tokens = quote_spanned! { input.span() =>
          compile_error!("the async keyword is missing from the function declaration");
        };
        return TokenStream::from(tokens);
    }

    let result = quote! {
      fn #name() #ret {
        runtime_raw::enter(#rt, async { #body })
      }
    };

    result.into()
}

/// Creates an async unit test.
///
/// # Examples
///
/// ```
/// #![feature(async_await, futures_api)]
///
/// #[runtime::test]
/// async fn main() -> std::io::Result<()> {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let rt = if attr.is_empty() {
        syn::parse_str("runtime_native::Native").unwrap()
    } else {
        syn::parse_macro_input!(attr as syn::Expr)
    };
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.decl.output;
    let name = &input.ident;
    let body = &input.block;

    if input.asyncness.is_none() {
        let tokens = quote_spanned! { input.span() =>
          compile_error!("the async keyword is missing from the function declaration");
        };
        return TokenStream::from(tokens);
    }

    let result = quote! {
      #[test]
      fn #name() #ret {
        runtime_raw::enter(#rt, async { #body })
      }
    };

    result.into()
}

/// Creates an async benchmark.
///
/// # Examples
///
/// ```
/// #![feature(async_await, await_macro, futures_api, test)]
///
/// extern crate test;
///
/// #[runtime::test]
/// async fn spawn_and_await() {
///   await!(runtime::spawn(async {}));
/// }
/// ```
#[proc_macro_attribute]
pub fn bench(attr: TokenStream, item: TokenStream) -> TokenStream {
    let rt = if attr.is_empty() {
        syn::parse_str("runtime_native::Native").unwrap()
    } else {
        syn::parse_macro_input!(attr as syn::Expr)
    };
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let args = &input.decl.inputs;
    let name = &input.ident;
    let body = &input.block;

    if input.asyncness.is_none() {
        let tokens = quote_spanned! { input.span() =>
          compile_error!("the async keyword is missing from the function declaration");
        };
        return TokenStream::from(tokens);
    }

    if !args.is_empty() {
        let tokens = quote_spanned! { args.span() =>
          compile_error!("async benchmarks don't take any arguments");
        };
        return TokenStream::from(tokens);
    }

    let result = quote! {
      #[bench]
      fn #name(b: &mut test::Bencher) {
        b.iter(|| {
          let _ = runtime_raw::enter(#rt, async { #body });
        });
      }
    };

    result.into()
}
