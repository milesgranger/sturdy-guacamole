use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn basic_gen() {
    let tr8t = Trait::new("Foo", true);

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
    let mut tr8t = Trait::new("Foo", true);
    tr8t.add_signature(FunctionSignature::new("foo", false));
    tr8t.add_signature(FunctionSignature::new("bar", false));
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
