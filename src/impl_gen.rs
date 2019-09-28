use serde::Serialize;
use tera::{Context, Tera};

use crate::traits::SrcCode;
use crate::{Function, Generics, Trait};

#[derive(Serialize, Default)]
pub struct Impl {
    pub generics: Generics,
    pub impl_trait: Option<Trait>,
    pub functions: Vec<Function>,
    pub obj_name: String,
}

impl Impl {
    pub fn new<S: ToString>(
        obj_name: S,
        generics: Generics,
        impl_trait: Option<Trait>,
        functions: Vec<Function>,
    ) -> Self {
        Self {
            obj_name: obj_name.to_string(),
            generics,
            functions,
            impl_trait,
        }
    }
}

impl SrcCode for Impl {
    fn generate(&self) -> String {
        let template = r#"
            impl for {{ self.obj_name }} {
                {% for function in functions %}
                {{ function }}
                {% endfor %}
            }
        "#;
        unimplemented!()
    }
}
