//!
//! Trait(s) specific to code generation objects within this crate.
//!

use crate::internal::{Annotations, Fields, InnerAndOuterAnnotations};
use crate::Field;

/// Trait implemented for elements representing the ability to render as
/// raw source code.
pub trait SrcCode {
    /// Given current configuration, give the resulting source code.
    #[must_use]
    fn generate(&self) -> String;
}

/// Provides methods to add annotations to elements.
pub trait AnnotationExt {
    /// Add a single annotation.
    fn add_annotation(&mut self, annotation: impl ToString) -> &mut Self;

    /// Add multiple annotations at once.
    fn add_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self;
}

impl<T: Annotations> AnnotationExt for T {
    /// Add a single annotation.
    fn add_annotation(&mut self, annotation: impl ToString) -> &mut Self {
        self.annotations().push(annotation.to_string());
        self
    }

    /// Add multiple annotations at once.
    fn add_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self {
        self.annotations()
            .extend(annotations.into_iter().map(|a| a.to_string()));
        self
    }
}

/// Provides methods to add annotations to elements.
pub trait InnerAndOuterAnnotationExt {
    /// Add a single inner annotation.
    fn add_inner_annotation(&mut self, annotation: impl ToString) -> &mut Self;

    /// Add multiple inner annotations at once.
    fn add_inner_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self;

    /// Add a single outer annotation.
    fn add_outer_annotation(&mut self, annotation: impl ToString) -> &mut Self;

    /// Add multiple outer annotations at once.
    fn add_outer_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self;
}

impl<T: InnerAndOuterAnnotations> InnerAndOuterAnnotationExt for T {
    /// Add a single inner annotation.
    fn add_inner_annotation(&mut self, annotation: impl ToString) -> &mut Self {
        self.inner_annotations().push(annotation.to_string());
        self
    }

    /// Add multiple inner annotations at once.
    fn add_inner_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self {
        self.inner_annotations()
            .extend(annotations.into_iter().map(|a| a.to_string()));
        self
    }

    /// Add a single outer annotation.
    fn add_outer_annotation(&mut self, annotation: impl ToString) -> &mut Self {
        self.outer_annotations().push(annotation.to_string());
        self
    }

    /// Add multiple outer annotations at once.
    fn add_outer_annotations(
        &mut self,
        annotations: impl IntoIterator<Item = impl ToString>,
    ) -> &mut Self {
        self.outer_annotations()
            .extend(annotations.into_iter().map(|a| a.to_string()));
        self
    }
}

/// Provides methods to add fields to elements.
pub trait FieldExt {
    /// Add a single field.
    fn add_field(&mut self, field: Field) -> &mut Self;

    /// Add multiple fields at once.
    fn add_fields<'a>(&mut self, fields: impl IntoIterator<Item = &'a Field>) -> &mut Self;
}

impl<T: Fields> FieldExt for T {
    /// Add a single field.
    fn add_field(&mut self, field: Field) -> &mut Self {
        self.fields().push(field);
        self
    }

    /// Add multiple fields at once.
    fn add_fields<'a>(&mut self, fields: impl IntoIterator<Item = &'a Field>) -> &mut Self {
        self.fields()
            .extend(fields.into_iter().map(|f| f.to_owned()));
        self
    }
}
