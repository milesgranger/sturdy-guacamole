//!
//! Create `enum` objects
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
    /// Annotation representing an attribute for type. ie. `#[foo(bar)]`
    Attr(String),
    /// Annotation representing a comment. ie. `// foo`
    Comment(String),
    /// Annotation representing a doc string. ie. `/// documentation`
    Doc(String),
    /// Annotation representing a module attribute. ie. `#![warn(...)]`
    ModuleAttr(String),
    /// Annotation representing a module level doc string. ie. `//! module documentation`
    ModuleDoc(String),
}

impl<S: ToString> From<S> for Annotation {
    fn from(annotation: S) -> Self {
        let annotation = annotation.to_string();

        if annotation.starts_with("//!") {
            Annotation::ModuleDoc(annotation)
        } else if annotation.starts_with("///") {
            Annotation::Doc(annotation)
        } else if annotation.starts_with("//") {
            Annotation::Comment(annotation)
        } else if annotation.starts_with("#!") {
            Annotation::ModuleAttr(annotation)
        } else if annotation.starts_with('#') {
            Annotation::Attr(annotation)
        } else {
            panic!("No Annotation match for '{}'", annotation)
        }
    }
}

impl SrcCode for Annotation {
    fn generate(&self) -> String {
        match self {
            Annotation::Attr(s)
            | Annotation::Comment(s)
            | Annotation::Doc(s)
            | Annotation::ModuleAttr(s)
            | Annotation::ModuleDoc(s) => s.to_owned(),
        }
    }
}
