use serde::Serialize;
use tera::{Context, Tera};

use crate::{Generics};
use crate::traits::SrcCode;


#[derive(Default, Serialize)]
pub struct Function {
    pub name: String,
    pub is_pub: bool,
    pub parameters: Vec<String>,
    pub generics: Generics,
    pub return_ty: String,
    pub block: String,
}

impl Function {
    pub fn new<S: ToString>(name: S, is_pub: bool, parameters: Vec<String>, generics: Generics, return_ty: S, block: S) -> Self {
        Self {
            name: name.to_string(),
            is_pub,
            parameters,
            generics,
            return_ty: return_ty.to_string(),
            block: block.to_string()
        }
    }
}

impl SrcCode for Function {
    fn generate(&self) -> String {
        let template = r#"
        {% if self.is_pub %}pub {% endif %}fn {{ self.name }}({{ self.parameters | join(sep=", ") }}) -> {{ self.return_ty }}
        {
            {{ self.block }}
        }
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        Tera::one_off(template, &context, false).unwrap()
    }
}
