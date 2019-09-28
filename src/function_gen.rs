use serde::Serialize;
use tera::{Context, Tera};

use crate::traits::SrcCode;
use crate::Generics;

/// Represents a function or method. Determined if any `Parameter` contains `self`
#[derive(Default, Serialize)]
pub struct Function {
    pub name: String,
    pub is_pub: bool,
    pub parameters: Vec<Parameter>,
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
    pub fn add_parameter(&mut self, param: Parameter) {
        self.parameters.push(param)
    }
}

#[derive(Serialize, Default)]
pub struct Parameter {
    pub name: String,
    pub ty: String,
}
impl Parameter {
    pub fn new<S: ToString>(name: S, ty: S) -> Self {
        Self {
            name: name.to_string(),
            ty: ty.to_string(),
        }
    }
}

impl SrcCode for Function {
    fn generate(&self) -> String {
        let template = r#"
        {% if self.is_pub %}pub {% endif %}fn {{ self.name }}({{ parameters | join(sep=", ") }}) -> {{ return_ty }}
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
        context.insert(
            "parameters",
            &self
                .parameters
                .iter()
                .map(|param| format!("{}: {}", param.name, param.ty))
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &context, false).unwrap()
    }
}
