//!
//! Traits for annotations
//!

use crate::internal::Annotations;
use crate::Annotation;

/// Provides methods to add annotations to elements.
pub trait AnnotationExt {
    /// Add a single annotation.
    fn add_annotation(&mut self, annotation: impl Into<Annotation>) -> &mut Self;

    /// Add multiple annotations at once.
    fn add_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl Into<Annotation>>,
    ) -> &mut Self;
}

impl<T: Annotations> AnnotationExt for T {
    /// Add a single annotation.
    fn add_annotation(&mut self, annotation: impl Into<Annotation>) -> &mut Self {
        self.annotations_mut().push(annotation.into());
        self
    }

    /// Add multiple annotations at once.
    fn add_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl Into<Annotation>>,
    ) -> &mut Self {
        self.annotations_mut()
            .extend(annotations.into_iter().map(|a| a.into()));
        self
    }
}
