use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn basic_gen() {
    let tr8t = Trait::new("Foo").set_is_pub(true);
    let expected = r#"
        pub trait Foo
        {
        }
    "#;

    let src_code = tr8t.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn gen_with_method_signatures() {
    let tr8t = Trait::new("Foo")
        .set_is_pub(true)
        .add_signature(FunctionSignature::new("foo"))
        .add_signature(FunctionSignature::new("bar"));
    let expected = r#"
        pub trait Foo
        {
            fn foo() -> ();
            fn bar() -> ();
        }
    "#;

    let src_code = tr8t.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}

#[test]
fn gen_with_generics() {
    let tr8t = Trait::new("Foo")
        .set_is_pub(true)
        .add_signature(FunctionSignature::new("foo").add_parameter(Parameter::new("name", "T")))
        .add_signature(FunctionSignature::new("bar"))
        .add_generic(Generic::new("T").add_trait_bounds(vec!["ToString"]));
    let expected = r#"
        pub trait Foo<T>
            where
                T: ToString,
        {
            fn foo(name: T) -> ();
            fn bar() -> ();
        }
    "#;

    let src_code = tr8t.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}
