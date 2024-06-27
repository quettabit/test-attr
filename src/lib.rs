use proc_macro::TokenStream;
use quote::quote;

// TODO: try factoring out common stuffs from all these fns.

#[proc_macro_attribute]
pub fn unit_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output: TokenStream = quote! {
        #[test]
        #[cfg_attr(not(feature = "unit-test"), ignore)]
    }
    .into();
    output.extend(input.into_iter());
    output
}

#[proc_macro_attribute]
pub fn integration_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output: TokenStream = quote! {
        #[test]
        #[cfg_attr(not(feature = "integration-test"), ignore)]
    }
    .into();
    output.extend(input.into_iter());
    output
}

#[proc_macro_attribute]
pub fn system_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output: TokenStream = quote! {
        #[test]
        #[cfg_attr(not(feature = "system-test"), ignore)]
    }
    .into();
    output.extend(input.into_iter());
    output
}

#[proc_macro_attribute]
pub fn tokio_unit_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output: TokenStream = quote! {
        #[tokio::test]
        #[cfg_attr(not(feature = "unit-test"), ignore)]
    }
    .into();
    output.extend(input.into_iter());
    output
}

#[proc_macro_attribute]
pub fn tokio_integration_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output: TokenStream = quote! {
        #[tokio::test]
        #[cfg_attr(not(feature = "integration-test"), ignore)]
    }
    .into();
    output.extend(input.into_iter());
    output
}

#[proc_macro_attribute]
pub fn tokio_system_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output: TokenStream = quote! {
        #[tokio::test]
        #[cfg_attr(not(feature = "system-test"), ignore)]
    }
    .into();
    output.extend(input.into_iter());
    output
}
