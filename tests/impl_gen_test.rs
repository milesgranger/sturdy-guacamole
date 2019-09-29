use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn impl_basic_gen_with_trait() {
    let ipl = Impl::new("That", Some(Trait::new("This")));

    let expected = r#"
        impl This for That
        {
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    )
}

#[test]
fn impl_basic_gen_without_trait() {
    let ipl = Impl::new("That", None);

    let expected = r#"
        impl That
        {
        }
    "#;

    let src_code = ipl.generate();
    println!("{}", &src_code);

    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    )
}
