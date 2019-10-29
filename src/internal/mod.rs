use crate::Field;

/// Internal trait to get access to the container storing the annotations.
/// Used for the generic implementation of `AnnotationExt`
pub trait Annotations {
    fn annotations(&mut self) -> &mut Vec<String>;
}

/// Internal trait to get access to the container storing the inner and outer annotations.
/// Used for the generic implementation of `InnerAndOuterAnnotationExt`
pub trait InnerAndOuterAnnotations {
    fn inner_annotations(&mut self) -> &mut Vec<String>;
    fn outer_annotations(&mut self) -> &mut Vec<String>;
}

/// Internal trait to get access to the container storing the fields.
/// Used for the generic implementation of `FieldsExt`
pub trait Fields {
    fn fields(&mut self) -> &mut Vec<Field>;
}
