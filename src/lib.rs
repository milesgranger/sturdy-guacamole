
use tera::{Context, Tera};
use serde::{Serialize};



pub trait SrcCode {
    fn generate(&self) -> String;
}


#[derive(Default, Serialize)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub annotations: Vec<String>,
    pub docs: Vec<String>,
}

impl Field {
    pub fn new<S: ToString>(name: S, ty: S) -> Self {
        let mut f = Field::default();
        f.name = name.to_string();
        f.ty = ty.to_string();
        f
    }
}

impl SrcCode for Field {
    fn generate(&self) -> String {
        let template = r#"
            {% for doc in docs %}{{ doc }}{% endfor %}
            {% for annotation in annotations %}{{ annotation }}{% endfor %}
            pub {{name}}: {{ty}},
        "#;
        let mut context = Context::new();
        context.insert("name", &self.name);
        context.insert("ty", &self.ty);
        context.insert("annotations", &self.annotations);
        context.insert("docs", &self.docs);
        Tera::one_off(template, &context, false).unwrap()
    }
}


#[derive(Default, Serialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub docs: Vec<String>
}

impl Struct {
    pub fn new<S: ToString>(name: S) -> Self {
        let mut s = Struct::default();
        s.name = name.to_string();
        s
    }

    pub fn field(&mut self, field: Field) {
        self.fields.push(field)
    }
}

impl SrcCode for Struct {
    fn generate(&self) -> String {
        let template = r#"
         pub struct {{name}} {
            {% for field in fields %}{{field}}{% endfor %}
         }
        "#;
        let mut context = Context::new();
        context.insert("name", &self.name);

        let fields = self.fields.iter()
            .map(|f| f.generate())
            .collect::<Vec<String>>();
        context.insert("fields", &fields);
        Tera::one_off(template, &context, false).unwrap()
    }
}


#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_struct_gen() {
        let mut struct_ = Struct::new("Basic");

        let mut f = Field::new("field1", "String");
        f.annotations.push("#[serde = w]".to_string());
        f.docs.push("/// Some example documentation".to_string());
        struct_.field(f);

        struct_.field(Field::new("field2", "usize"));
        let expected = r#"
            pub struct Basic {
                pub field: usize
            }
        "#.to_owned();
        println!("{}", struct_.generate());
        //assert_eq!(src_code, expected);
    }
}