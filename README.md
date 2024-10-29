[![LICENSE](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE.txt)
[![](https://github.com/icsmw/enum_ids/actions/workflows/on_pull_request.yml/badge.svg)](https://github.com/icsmw/enum_ids/actions/workflows/on_pull_request.yml)
![Crates.io](https://img.shields.io/crates/v/enum_ids)

# `enum_ids` Procedural Macro

`enum_ids` is a procedural attribute macro designed to generate a companion enum with only the variant names from an existing enum. It provides customizable options to control attribute inheritance, method naming, visibility, and the naming of the generated enum.

## Usage

Apply the `#[enum_ids]` attribute to your enum to automatically generate a corresponding ID enum and a getter method.

```rust
use enum_ids::enum_ids;

#[enum_ids]
#[derive(Debug, Clone)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

fn main() {
    let kind_a = Kind::A(10);
    let id_a = kind_a.id();
    println!("{:?}", id_a); // Outputs: A
}
```

The macro generates:

```rust
#[derive(Debug, Clone)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

impl Kind {
    pub fn id(&self) -> KindId {
        match self {
            Kind::A(..) => KindId::A,
            Kind::B { .. } => KindId::B,
            Kind::C => KindId::C,
        }
    }
}

#[derive(Debug, Clone)]
pub enum KindId {
    A,
    B,
    C,
}

fn main() {
    let kind_a = Kind::A(10);
    let id_a = kind_a.id();
    println!("{:?}", id_a); // Outputs: A
}
```


## Supported Attributes

The `enum_ids` macro supports various attributes to customize its behavior. These attributes can be combined as needed.

`#[enum_ids]` - Inherits all derive attributes from the parent enum and names the getter method `id()`.
Default: The generated enum will be named by appending Id to the parent enum's name (e.g., KindId for Kind).

`#[enum_ids(derive = "Trait1, Trait2, ...")]` - Does not inherit derive attributes from the parent enum but adds the specified derive traits.

Example:

```rust
#[enum_ids(derive = "Serialize, Clone")]
pub enum Kind {
    A(i32),
    B(String),
    C,
}
```

`#[enum_ids(no_derive)]` - Does not add any derive attributes to the generated enum, regardless of the parent enum's attributes.

Example:
```rust
#[enum_ids(no_derive)]
pub enum Kind {
    A(i32),
    B(String),
    C,
}
```

`#[enum_ids(getter = "method_name")]` - Defines a custom name for the getter method instead of the default `id()`.

Example:

```rust

#[enum_ids(getter = "get_id")]
pub enum Kind {
    A(i32),
    B(String),
    C,
}
```
`#[enum_ids(name = "CustomName")]` - Manually sets the name of the generated enum instead of using the default naming convention (ParentNameId).

Example:
```rust
#[enum_ids(name = "KindIdentifier")]
pub enum Kind {
    A(i32),
    B(String),
    C,
}
```

`#[enum_ids(public)]` - Makes the generated enum pub, regardless of the parent enum's visibility.

Example:
```rust
#[enum_ids(public)]
pub enum Kind {
    A(i32),
    B(String),
    C,
}
```

`#[enum_ids(not_public)]` - Makes the generated enum non-public (private), overriding any inherited visibility from the parent enum.

Example:
```rust
#[enum_ids(not_public)]
pub enum Kind {
    A(i32),
    B(String),
    C,
}
```

## Combined Attributes
You can combine multiple attributes to achieve the desired configuration. For example:

```rust
#[enum_ids(derive = "Serialize", getter = "get_id", name = "KindIdentifier", public)]
#[derive(Debug, Clone)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}
```
This will generate:

```rust
#[derive(Serialize)]
pub enum KindIdentifier {
    A,
    B,
    C,
}

impl Kind {
    pub fn get_id(&self) -> KindIdentifier {
        match self {
            A(..) => KindIdentifier:A,
            B {..} => KindIdentifier:B,
            C => KindIdentifier::C,
        }
    }
}
```

## Getting ID of parent enum

In case if attribute `getter` hasn't been used, getting of ID would be possible on method `id()` of parent enum.

Example
```rust
use enum_ids::enum_ids;

#[enum_ids]
#[derive(Debug, Clone)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}
fn main() {
    assert_eq!(Kind::A(10).id(), KindId::A);
}
```

## Getting a full list of all IDs

`enum_ids` also gives the possibility to get a full list of all IDs. Compared with standard `into_iter`, this the method will not move data of the origin enum.

Example
```rust
use enum_ids::enum_ids;

#[enum_ids]
#[derive(Debug, Clone)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

fn main() {
    let mut count = 0;
    let mut all = KindId::into_iter();
    while let Some(id) = all.next() {
        println!("{id:?}");
    }
}
```

## Notes

The generated getter method matches each variant of the original enum and returns the corresponding variant of the ID enum.
Ensure that the specified derive traits are in scope where the macro is used.
The macro currently supports unit, tuple, and struct variants.