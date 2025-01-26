use crate::attr;
use proc_macro2::Span;
use std::convert::TryFrom;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Attribute, Expr, Ident, Lit, Token, Visibility,
};

/// Represents the context for generating enum IDs, holding relevant attributes.
#[derive(Clone, Debug, Default)]
pub struct Context {
    /// A list of attributes applied to the enum.
    pub attrs: Vec<attr::Attr>,
}

impl Context {
    /// Creates a new `Context` with the provided attributes.
    ///
    /// # Arguments
    ///
    /// * `attrs` - A vector of `Attr` attributes.
    pub(self) fn new(attrs: Vec<attr::Attr>) -> Self {
        Self { attrs }
    }

    /// Determines `display` is required
    pub fn display_required(&self) -> bool {
        self.attrs
            .iter()
            .any(|at| matches!(at, attr::Attr::Display))
    }

    /// Determines `display_variant` is required
    pub fn display_variant(&self) -> bool {
        self.attrs
            .iter()
            .any(|at| matches!(at, attr::Attr::DisplayVariant))
    }

    /// Determines `display_variant_snake` is required
    pub fn display_variant_snake(&self) -> bool {
        self.attrs
            .iter()
            .any(|at| matches!(at, attr::Attr::DisplayVariantSnake))
    }

    /// Determines `display_from_value` is required
    pub fn display_from_value_required(&self) -> bool {
        self.attrs
            .iter()
            .any(|at| matches!(at, attr::Attr::DisplayFromValue))
    }

    /// Determines the name of the generated ID enum.
    ///
    /// If an `EnumName` attribute is present, its value is used.
    /// Otherwise, the default naming convention (`ParentNameId`) is applied.
    ///
    /// # Arguments
    ///
    /// * `src` - The identifier of the source enum.
    ///
    /// # Returns
    ///
    /// * An `Ident` representing the name of the generated ID enum.
    pub fn enum_name(&self, src: &Ident) -> Ident {
        let name = self
            .attrs
            .iter()
            .find_map(|at| {
                if let attr::Attr::EnumName(name) = at {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .unwrap_or(format!("{src}Id"));
        Ident::new(&name, src.span())
    }

    /// Determines the name of the getter method for the ID.
    ///
    /// If a `Getter` attribute is present, its value is used.
    /// Otherwise, the default method name `id` is applied.
    ///
    /// # Arguments
    ///
    /// * `src` - The identifier of the source enum.
    ///
    /// # Returns
    ///
    /// * An `Ident` representing the name of the getter method.
    pub fn getter_name(&self, src: &Ident) -> Ident {
        let name = self
            .attrs
            .iter()
            .find_map(|at| {
                if let attr::Attr::Getter(name) = at {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .unwrap_or(String::from("id"));
        Ident::new(&name, src.span())
    }

    /// Determines the visibility of the generated ID enum.
    ///
    /// - If the `Public` attribute is present, the enum is made public.
    /// - If the `NotPublic` attribute is present, the enum is made private.
    /// - Otherwise, the visibility is inherited from the source enum.
    ///
    /// # Arguments
    ///
    /// * `vis` - The visibility of the source enum.
    ///
    /// # Returns
    ///
    /// * A `Visibility` instance representing the desired visibility.
    pub fn visibility(&self, vis: &Visibility) -> Visibility {
        if self.attrs.iter().any(|at| matches!(at, attr::Attr::Public)) {
            Visibility::Public(syn::token::Pub(proc_macro2::Span::call_site()))
        } else if self
            .attrs
            .iter()
            .any(|at| matches!(at, attr::Attr::NotPublic))
        {
            Visibility::Inherited
        } else {
            vis.clone()
        }
    }

    /// Determines the derive attributes for the generated ID enum.
    ///
    /// - If the `NoDerive` attribute is present, no derive attributes are added.
    /// - If a `Derive` attribute is present, only the specified traits are derived.
    /// - Otherwise, inherits derive attributes from the source enum.
    ///
    /// # Arguments
    ///
    /// * `attrs` - A slice of attributes from the source enum.
    ///
    /// # Returns
    ///
    /// * A vector of `Attribute` instances to be applied to the generated enum.
    pub fn derive(&self, attrs: &[Attribute]) -> Vec<Attribute> {
        if self
            .attrs
            .iter()
            .any(|at| matches!(at, attr::Attr::NoDerive))
        {
            vec![]
        } else if let Some(attr::Attr::Derive(list)) = self
            .attrs
            .iter()
            .find(|at| matches!(at, attr::Attr::Derive(..)))
        {
            let traits: Vec<Ident> = list
                .split(',')
                .map(|s| Ident::new(s.trim(), Span::call_site()))
                .collect();
            vec![parse_quote! { #[derive(#(#traits),*)] }]
        } else {
            attrs
                .iter()
                .filter(|attr| attr.path().is_ident("derive"))
                .cloned()
                .collect()
        }
    }
}

impl Parse for Context {
    /// Parses a stream of tokens into a `Context` struct.
    ///
    /// The expected input can include:
    /// - Attributes in the form of `key = "value"`
    /// - Standalone attributes like `public`, `not_public`, `no_derive`
    ///
    /// # Arguments
    ///
    /// * `input` - The input stream of tokens.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the parsed `Context` or a parsing error.
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut attrs: Vec<attr::Attr> = vec![];
        for expr in Punctuated::<Expr, Token![,]>::parse_terminated(input)? {
            match expr {
                Expr::Assign(a) => {
                    let link = a.clone();
                    if let (Expr::Path(left), Expr::Lit(right)) = (*a.left, *a.right) {
                        if let (Some(left), Lit::Str(value)) = (left.path.get_ident(), right.lit) {
                            let attr =
                                attr::Attr::try_from(left.to_string().as_ref()).map_err(|e| {
                                    syn::Error::new(
                                        left.span(),
                                        format!("Cannot parse attribute \"{left}\": {e}"),
                                    )
                                })?;
                            attrs.push(match attr {
                                attr::Attr::Derive(..) => attr::Attr::Derive(value.value()),
                                attr::Attr::Getter(..) => attr::Attr::Getter(value.value()),
                                attr::Attr::EnumName(..) => attr::Attr::EnumName(value.value()),
                                _ => {
                                    return Err(syn::Error::new(
                                        left.span(),
                                        format!(
                                            "Attribute \"{left}\" cannot be applied at this level"
                                        ),
                                    ));
                                }
                            });
                        } else {
                            return Err(syn::Error::new(
                                link.eq_token.span,
                                "Expecting expression like key = \"value as String\"",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new(
                            link.eq_token.span,
                            "Expecting expression like key = \"value as String\"",
                        ));
                    }
                }
                Expr::Path(p) => {
                    if let Some(ident) = p.path.get_ident() {
                        let attr =
                            attr::Attr::try_from(ident.to_string().as_ref()).map_err(|e| {
                                syn::Error::new(
                                    ident.span(),
                                    format!("Cannot parse attribute: {ident} ({e})"),
                                )
                            })?;
                        attrs.push(match attr {
                            attr::Attr::NoDerive
                            | attr::Attr::NotPublic
                            | attr::Attr::Public
                            | attr::Attr::Display
                            | attr::Attr::DisplayVariant
                            | attr::Attr::DisplayVariantSnake
                            | attr::Attr::DisplayFromValue => attr,
                            _ => {
                                return Err(syn::Error::new(
                                    ident.span(),
                                    format!(
                                        "Attribute \"{ident}\" cannot be applied at this level"
                                    ),
                                ));
                            }
                        });
                    } else {
                        return Err(syn::Error::new_spanned(p, "Cannot extract identifier"));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "Expecting expression like [key = \"value as String\"] or [key]",
                    ));
                }
            }
        }
        Ok(Context::new(attrs))
    }
}
