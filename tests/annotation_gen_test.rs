pub mod utilities;
use proffer::{Annotation, SrcCode};

#[test]
fn test_annotation_attr() {
    let ann = "#[attr]";
    let annotation = Annotation::from(ann);
    match &annotation {
        &Annotation::ItemAttr(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Annotation::ItemAttr, got {:?}", ann),
    };
    assert_eq!(&annotation.generate(), ann);
}

#[test]
fn test_annotation_mod_attr() {
    let ann = "#![foo_attr]";
    let annotation = Annotation::from(ann);
    match &annotation {
        &Annotation::ScopeAttr(ref s) => assert_eq!(&s, &ann),
        _ => panic!("Expected to match to Annotation::ScopeAttr, got {:?}", ann),
    };
    assert_eq!(&annotation.generate(), ann);
}
