//!
//! Traits for documentation lines
//!

use crate::internal::Docs;
use crate::Annotation;

/// Provides methods to add documentation line. to elements.
pub trait DocExt {
    /// Add a single documentation line.
    fn add_doc(&mut self, doc: impl Into<Annotation>) -> &mut Self;

    /// Add multiple documentation lines at once.
    fn add_docs(&mut self, docs: impl IntoIterator<Item = impl Into<Annotation>>) -> &mut Self;
}

impl<T: Docs> DocExt for T {
    /// Add a single documentation line.
    fn add_doc(&mut self, doc: impl Into<Annotation>) -> &mut Self {
        self.docs().push(doc.into());
        self
    }

    /// Add multiple documentation lines at once.
    fn add_docs(&mut self, doc: impl IntoIterator<Item = impl Into<Annotation>>) -> &mut Self {
        self.docs().extend(doc.into_iter().map(|d| d.into()));
        self
    }
}
