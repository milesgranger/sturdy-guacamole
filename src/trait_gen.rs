use serde::Serialize;

use crate::traits::SrcCode;
use crate::FunctionSignature;
use tera::{Context, Tera};

#[derive(Serialize, Default)]
pub struct Trait {
    pub name: String,
    pub is_pub: bool,
    pub trait_bounds: Vec<String>,
    signatures: Vec<FunctionSignature>,
}

impl Trait {
    pub fn new<S: ToString>(name: S, is_pub: bool) -> Self {
        let mut t = Self::default();
        t.name = name.to_string();
        t.is_pub = is_pub;
        t
    }
    pub fn add_signature(&mut self, signature: FunctionSignature) {
        self.signatures.push(signature)
    }
}

impl SrcCode for Trait {
    fn generate(&self) -> String {
        let template = r#"
            {% if self.is_pub %}pub {% endif %}trait {{ self.name }}
            {
                {% for signature in signatures %}{{ signature }};{% endfor %}
            }
        "#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert(
            "signatures",
            &self
                .signatures
                .iter()
                .map(|s| s.generate())
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &context, false).unwrap()
    }
}
