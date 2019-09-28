use serde::Serialize;
use tera::{Context, Tera};

use crate::traits::SrcCode;
use crate::Generics;

/// Represents a function or method. Determined if any `Parameter` contains `self`
#[derive(Default, Serialize)]
pub struct Function {
    pub name: String,
    pub is_pub: bool,
    pub parameters: Vec<String>,
    pub generics: Generics,
    pub return_ty: Option<String>,
    pub block: String,
}

impl Function {
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut f = Self::default();
        f.name = name.to_string();
        f.is_pub = is_pub;
        f
    }
}

impl SrcCode for Function {
    fn generate(&self) -> String {
        let template = r#"
        {% if self.is_pub %}pub {% endif %}fn {{ self.name }}({{ self.parameters | join(sep=", ") }}) -> {{ return_ty }}
        {
            {{ self.block }}
        }
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert(
            "return_ty",
            &self.return_ty.as_ref().unwrap_or(&"()".to_string()),
        );
        Tera::one_off(template, &context, false).unwrap()
    }
}
