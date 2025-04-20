#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

mod attr;
mod context;
#[cfg(test)]
mod test;

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
/// use enum_ids::enum_ids;
///
/// #[enum_ids]
/// #[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
/// pub enum Kind {
///     A(i32),
///     B { value: String },
///     C,
/// }
/// ```
/// or with attributes
///
/// ```rust
/// use enum_ids::enum_ids;
/// use serde::{Deserialize, Serialize};
///
/// #[enum_ids(getter = "get_id", derive = "Deserialize, Serialize", public)]
/// #[derive(Debug, Clone)]
/// pub enum Kind {
///     A(i32),
///     B { value: String },
///     C,
/// }
/// ```
#[proc_macro_attribute]
pub fn enum_ids(args: TokenStream, item: TokenStream) -> TokenStream {
    let context: Context = parse_macro_input!(args as Context);
    let input: ItemEnum = parse_macro_input!(item as ItemEnum);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let src = &input.ident;
    let visibility = context.visibility(&input.vis);
    let dest_ident = context.enum_name(src);
    let getter_ident = context.getter_name(src);

    let variants = input.variants.iter().map(|v| &v.ident);

    let derive_attrs: Vec<Attribute> = context.derive(&input.attrs);

    let match_arms = input.variants.iter().map(|v| get_arm(v, src, &dest_ident));

    let iter_values = input.variants.iter().map(|v| {
        let variant = &v.ident;
        quote! {
            #dest_ident::#variant
        }
    });

    let disaply_impl = get_display_impl(&context, &input, &dest_ident, src);

    let disaply_variant_impl = get_display_variant_impl(&context, &input, &dest_ident);

    let disaply_from_value_impl = get_display_from_value_required(&context, &input, src);

    let self_itarator_impl = get_iterator(&context, &input, src);

    let expanded = quote! {
        #input

        impl #impl_generics #src #ty_generics #where_clause {
            /// Returns the corresponding ID variant for the enum instance.
            ///
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

        #self_itarator_impl

        impl #dest_ident {
            pub fn as_vec() -> Vec<#dest_ident> {
                vec![#(#iter_values),*]
            }
        }

        #disaply_impl

        #disaply_variant_impl

        #disaply_from_value_impl
    };

    TokenStream::from(expanded)
}

fn get_arm(
    variant: &syn::Variant,
    src: &proc_macro2::Ident,
    dest_ident: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => {
            quote! {
                #src::#variant_ident => #dest_ident::#variant_ident,
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                #src::#variant_ident(..) => #dest_ident::#variant_ident,
            }
        }
        Fields::Named(_) => {
            quote! {
                #src::#variant_ident{..} => #dest_ident::#variant_ident,
            }
        }
    }
}

fn get_display_impl(
    cx: &Context,
    input: &ItemEnum,
    dest_ident: &proc_macro2::Ident,
    src: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    if cx.display_required() {
        let arms = input.variants.iter().map(|v| {
            let variant = &v.ident;
            quote! {
                #dest_ident::#variant => stringify!(#src::#variant),
            }
        });
        quote! {
            impl std::fmt::Display for #dest_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        match self {
                            #(#arms)*
                        }
                    )
                }
            }
        }
    } else {
        quote! {}
    }
}

fn get_display_variant_impl(
    cx: &Context,
    input: &ItemEnum,
    dest_ident: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    fn to_snake_case<S: AsRef<str>>(name: S) -> String {
        let mut result = String::new();

        for (i, c) in name.as_ref().chars().enumerate() {
            if c.is_uppercase() {
                if i != 0 {
                    result.push('_');
                }
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }

        result
    }
    if cx.display_variant() || cx.display_variant_snake() {
        let arms = input.variants.iter().map(|v| {
            let variant = &v.ident;
            if cx.display_variant() {
                quote! {
                    #dest_ident::#variant => stringify!(#variant),
                }
            } else {
                let variant_str = to_snake_case(variant.to_string());
                quote! {
                    #dest_ident::#variant => #variant_str,
                }
            }
        });
        quote! {
            impl std::fmt::Display for #dest_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        match self {
                            #(#arms)*
                        }
                    )
                }
            }
        }
    } else {
        quote! {}
    }
}

fn get_display_from_value_required(
    cx: &Context,
    input: &ItemEnum,
    src: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    if cx.display_from_value_required() {
        let arms = input.variants.iter().map(|v| {
            let variant = &v.ident;
            quote! {
                #src::#variant(v) => v.to_string(),
            }
        });
        quote! {
            impl std::fmt::Display for #src {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        match self {
                            #(#arms)*
                        }
                    )
                }
            }
        }
    } else {
        quote! {}
    }
}

fn get_iterator(
    cx: &Context,
    input: &ItemEnum,
    src: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    if cx.iterator() {
        let iter_values = input.variants.iter().map(|v| {
            let variant = &v.ident;
            quote! {
                #src::#variant
            }
        });
        quote! {
            impl #src {
                pub fn as_vec() -> Vec<#src> {
                    vec![#(#iter_values),*]
                }
            }
        }
    } else {
        quote! {}
    }
}
