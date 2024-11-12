use std::{convert::TryFrom, fmt};

/// Represents the various attributes that can be applied to the `enum_ids` procedural macro.
///
/// This enum is used to parse and handle different attributes provided to the macro,
/// allowing customization of its behavior such as controlling trait derivations,
/// method naming, visibility, and the naming of the generated enum.
#[derive(Clone, Debug)]
pub enum Attr {
    /// Adds implementation of `std::fmt::Display` for origin enum; can be used only with unnamed
    /// fields like `Field(value)` and will add convertation `Field(value) => value.to_string()`
    DisplayFromValue,

    /// Adds implementation of `std::fmt::Display`
    Display,

    /// Adds implementation of `std::fmt::Display` without reference to name of enum
    DisplayVariant,

    /// Prevents the copying of any `derive` attributes from the source enum to the generated enum.
    NoDerive,

    /// Specifies additional traits to derive for the generated enum.
    ///
    /// The associated `String` contains a comma-separated list of trait names.
    Derive(String),

    /// Defines a custom name for the getter method instead of the default `id()`.
    ///
    /// The associated `String` specifies the desired method name.
    Getter(String),

    /// Sets a custom name for the generated enum instead of the default naming convention (`ParentNameId`).
    ///
    /// The associated `String` specifies the desired enum name.
    EnumName(String),

    /// Sets the visibility of the generated enum to private, overriding any inherited visibility.
    NotPublic,

    /// Sets the visibility of the generated enum to public, regardless of the source enum's visibility.
    Public,
}

impl TryFrom<&str> for Attr {
    type Error = String;

    /// Attempts to convert a string slice to an `Attr` variant.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that holds the attribute name.
    ///
    /// # Returns
    ///
    /// * `Ok(Attr)` if the string matches a known attribute.
    /// * `Err(String)` if the attribute is unknown.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if Attr::Derive(String::new()).to_string() == value {
            Ok(Attr::Derive(String::new()))
        } else if Attr::Getter(String::new()).to_string() == value {
            Ok(Attr::Getter(String::new()))
        } else if Attr::EnumName(String::new()).to_string() == value {
            Ok(Attr::EnumName(String::new()))
        } else if Attr::NoDerive.to_string() == value {
            Ok(Attr::NoDerive)
        } else if Attr::Display.to_string() == value {
            Ok(Attr::Display)
        } else if Attr::DisplayVariant.to_string() == value {
            Ok(Attr::DisplayVariant)
        } else if Attr::DisplayFromValue.to_string() == value {
            Ok(Attr::DisplayFromValue)
        } else if Attr::NotPublic.to_string() == value {
            Ok(Attr::NotPublic)
        } else if Attr::Public.to_string() == value {
            Ok(Attr::Public)
        } else {
            Err(format!("Unknown attribute \"{value}\""))
        }
    }
}

impl fmt::Display for Attr {
    /// Formats the `Attr` variant as a string.
    ///
    /// This is useful for converting the enum variants back to their string representations.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// * `fmt::Result` indicating success or failure.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Derive(..) => "derive",
                Self::Getter(..) => "getter",
                Self::EnumName(..) => "name",
                Self::Display => "display",
                Self::DisplayVariant => "display_variant",
                Self::DisplayFromValue => "display_from_value",
                Self::NoDerive => "no_derive",
                Self::NotPublic => "not_public",
                Self::Public => "public",
            }
        )
    }
}
