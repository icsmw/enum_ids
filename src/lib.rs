#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

mod attr;
mod context;

use context::Context;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Fields, ItemEnum};

/// Procedural macro to generate a companion ID enum and an associated getter method for the annotated enum.
///
/// # Attributes
///
/// - `derive = "Trait1, Trait2, ..."`: Specifies traits to derive for the generated ID enum.
/// - `getter = "method_name"`: Sets a custom name for the getter method instead of the default `id`.
/// - `name = "CustomName"`: Sets a custom name for the generated ID enum instead of the default `ParentNameId`.
/// - `public`: Makes the generated ID enum public.
/// - `not_public`: Makes the generated ID enum private.
/// - `no_derive`: Disables deriving traits for the generated ID enum.
///
/// # Example
///
/// ```rust
/// #[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Copy)]
/// #[enum_ids]
/// pub enum Kind {
///     A(i32),
///     B { value: String },
///     C,
/// }
/// ```
/// or with attributes
///
/// ```rust
/// #[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Copy)]
/// #[enum_ids(getter = "get_id", attr = "#[derive(serde::Serialize)]", public)]
/// pub enum Kind {
///     A(i32),
///     B { value: String },
///     C,
/// }
/// ```
#[proc_macro_attribute]
pub fn enum_ids(args: TokenStream, item: TokenStream) -> TokenStream {
    let context: Context = parse_macro_input!(args as Context);
    let input = parse_macro_input!(item as ItemEnum);

    let src = &input.ident;
    let visibility = context.visibility(&input.vis);
    let dest_ident = context.enum_name(src);
    let getter_ident = context.getter_name(src);

    let variants = input.variants.iter().map(|v| &v.ident);

    let derive_attrs: Vec<Attribute> = context.derive(&input.attrs);

    let match_arms = input.variants.iter().map(|v| {
        let variant = &v.ident;
        match &v.fields {
            Fields::Unit => {
                quote! {
                    #src::#variant => #dest_ident::#variant,
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    #src::#variant(..) => #dest_ident::#variant,
                }
            }
            Fields::Named(_) => {
                quote! {
                    #src::#variant{..} => #dest_ident::#variant,
                }
            }
        }
    });

    let expanded = quote! {
        #input

        impl #src {
            /// Returns the corresponding ID variant for the enum instance.
            ///
            /// # Examples
            ///
            /// ```rust
            /// let kind = Kind::A(10);
            /// assert_eq!(kind.id(), KindId::A);
            /// ```
            pub fn #getter_ident(&self) -> #dest_ident {
                match self {
                    #(#match_arms)*
                }
            }
        }

        #(#derive_attrs)*
        #visibility enum #dest_ident {
            #(#variants),*
        }
    };

    TokenStream::from(expanded)
}
