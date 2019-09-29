use serde::Serialize;

use crate::traits::SrcCode;
use tera::{Context, Tera};

#[derive(Serialize, Default)]
pub struct Trait {
    pub name: String,
    pub is_pub: bool,
    pub trait_bounds: Vec<String>,
}

impl Trait {
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut t = Self::default();
        t.name = name.to_string();
        t.is_pub = is_pub;
        t
    }
}

impl SrcCode for Trait {
    fn generate(&self) -> String {
        let template = r#"
            {% if self.is_pub %}pub {% endif %}trait {{ self.name }}
            {
            }
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        Tera::one_off(template, &context, false).unwrap()
    }
}
