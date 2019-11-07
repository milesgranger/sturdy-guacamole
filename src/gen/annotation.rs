//!
//! Create annotation objects using the `Annotation` enum.
//!
//!
//! Example
//! -------
//! ```
//! use proffer::*;
//!
//! let a = Annotation::from("#![be_cool]");
//!
//! let src_code = a.generate();
//! let expected = "#![be_cool]";
//! assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code))
//! ```

use crate::SrcCode;
use serde::{Deserialize, Serialize};

/// Represents a single Rust annotation to a module, function, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Annotation {
    /// Annotation representing an attribute for an item. ie. `#[foo(bar)]`
    ItemAttr(String),
    /// Annotation representing a scoped level attribute. ie. `#![warn(...)]`
    ScopeAttr(String),
}

// TODO: Use TryFrom when https://github.com/rust-lang/rust/issues/50133 is resolved.
impl<S: ToString> From<S> for Annotation {
    fn from(annotation: S) -> Self {
        let annotation = annotation.to_string();

        if annotation.starts_with("#!") {
            Annotation::ScopeAttr(annotation)
        } else if annotation.starts_with('#') {
            Annotation::ItemAttr(annotation)
        } else {
            panic!("No Annotation match for '{}'", annotation)
        }
    }
}

impl SrcCode for Annotation {
    fn generate(&self) -> String {
        match self {
            Annotation::ScopeAttr(s) | Annotation::ItemAttr(s) => s.to_owned(),
        }
    }
}
