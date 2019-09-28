use proffer::*;

fn normalize_whitespace(s: &str) -> String {
    s.split("\n")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .collect::<String>()
}

#[test]
fn function_gen_basic() {
    let function = Function::new("foo", true);

    let expected = r#"
        pub fn foo() -> ()
        {
        }
    "#;

    let src_code = function.generate();
    println!("{}", &src_code);
    assert_eq!(
        normalize_whitespace(expected),
        normalize_whitespace(&src_code)
    );
}
