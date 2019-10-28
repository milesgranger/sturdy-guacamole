use proffer::*;
use syn::ItemEnum;

#[test]
fn gen_enum_basic() {
    let e = Enum::new("Foo")
        .add_variant(Variant::new("A"))
        .add_variant(Variant::new("B"))
        .set_is_pub(true)
        .to_owned();

    let src_code = e.generate();
    println!("{}", &src_code);

    let expected = r#"
        pub enum Foo
        {
            A,
            B,
        }
    "#;

    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
    syn::parse_str::<ItemEnum>(&src_code).unwrap();
}

#[test]
fn gen_enum_with_generic() {
    let e = Enum::new("Foo")
        .add_variant(Variant::new("A"))
        .add_variant(Variant::new("B").set_inner(Some("(T)")).to_owned())
        .add_generic(Generic::new("T"))
        .to_owned();

    let src_code = e.generate();
    println!("{}", &src_code);

    let expected = r#"
        enum Foo<T>
            where
                T: ,
        {
            A,
            B(T),
        }
    "#;
    assert_eq!(norm_whitespace(expected), norm_whitespace(&src_code));
    syn::parse_str::<ItemEnum>(&src_code).unwrap();
}
