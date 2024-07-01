use proc_macro::{TokenStream, TokenTree};
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

fn partition_at_first_ident(input: TokenStream) -> (TokenStream, TokenStream) {
    let mut left = TokenStream::new();
    let mut right = TokenStream::new();
    let mut input = input.into_iter();
    for token_tree in input.by_ref() {
        match token_tree {
            TokenTree::Ident(_) => {
                right.extend(TokenStream::from(token_tree));
                break;
            }
            _ => {
                left.extend(TokenStream::from(token_tree));
            }
        }
    }
    right.extend(input);
    (left, right)
}

#[proc_macro_attribute]
pub fn tokio_unit_rstest(_args: TokenStream, input: TokenStream) -> TokenStream {
    let (left, right) = partition_at_first_ident(input);
    let mut output: TokenStream = quote! {
        #[rstest]
    }
    .into();
    let center: TokenStream = quote! {
        #[tokio::test]
        #[cfg_attr(not(feature = "unit-test"), ignore)]
    }
    .into();
    output.extend(left);
    output.extend(center);
    output.extend(right);
    output
}

#[proc_macro_attribute]
pub fn unit_rstest(_args: TokenStream, input: TokenStream) -> TokenStream {
    let (left, right) = partition_at_first_ident(input);
    let mut output: TokenStream = quote! {
        #[rstest]
    }
    .into();
    let center: TokenStream = quote! {
        #[cfg_attr(not(feature = "unit-test"), ignore)]
    }
    .into();
    output.extend(left);
    output.extend(center);
    output.extend(right);
    output
}
