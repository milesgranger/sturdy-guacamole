pub mod utilities;
use proffer::{Annotation, SrcCode};

#[test]
fn test_annotation_attr() {
    let ann = "#[attr]";
    let annotation = Annotation::from(ann);
    match &annotation {
        &Annotation::Attr(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Annotation::Attr, got {:?}", ann),
    };
    assert_eq!(&annotation.generate(), ann);
}

#[test]
fn test_annotation_comment() {
    let ann = "// comment";
    let annotation = Annotation::from(ann);
    match &annotation {
        &Annotation::Comment(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Annotation::Comment, got {:?}", ann),
    };
    assert_eq!(&annotation.generate(), ann);
}

#[test]
fn test_annotation_doc() {
    let ann = "/// docky-doc";
    let annotation = Annotation::from(ann);
    match &annotation {
        &Annotation::Doc(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Annotation::Doc, got {:?}", ann),
    };
    assert_eq!(&annotation.generate(), ann);
}

#[test]
fn test_annotation_mod_attr() {
    let ann = "#![mod_attr]";
    let annotation = Annotation::from(ann);
    match &annotation {
        &Annotation::ModuleAttr(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Annotation::ModuleAttr, got {:?}", ann),
    };
    assert_eq!(&annotation.generate(), ann);
}

#[test]
fn test_annotation_mod_doc() {
    let ann = "//! module doc";
    let annotation = Annotation::from(ann);
    match &annotation {
        &Annotation::ModuleDoc(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Annotation::ModuleDoc, got {:?}", ann),
    };
    assert_eq!(&annotation.generate(), ann);
}
