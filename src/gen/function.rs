//!
//!
//! Function pieces, specifically `Function` which is composed of `FunctionSignature`
//! and `FunctionBody`. Naturally, a `Function` can be used as a "method" for another
//! object, by specifying `self` (or some variant of it) as the first `Parameter` to
//! a `Function` object.
//!

use serde::Serialize;
use tera::{Context, Tera};

use crate::internal::Generics;
use crate::traits::SrcCode;
use crate::{internal, Annotation, AnnotationExt, Generic, SrcCodeVec};

/// Represents a function or method. Determined if any `Parameter` contains `self`
#[derive(Default, Serialize, Clone)]
pub struct Function {
    signature: FunctionSignature,
    body: FunctionBody,
}

/// Represents a function/method signature in source code
#[derive(Default, Serialize, Clone)]
pub struct FunctionSignature {
    name: String,
    is_pub: bool,
    is_async: bool,
    parameters: Vec<Parameter>,
    generics: Vec<Generic>,
    return_ty: Option<String>,
    annotations: Vec<Annotation>,
}

impl FunctionSignature {
    /// Create a new function signature.
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Set this function as `async`
    pub fn set_is_async(&mut self, is_async: bool) -> &mut Self {
        self.is_async = is_async;
        self
    }

    /// Add a parameter to this signature
    pub fn add_parameter(&mut self, param: Parameter) -> &mut Self {
        self.parameters.push(param);
        self
    }

    /// Set a return type, if `None` will result in `()` type.
    pub fn set_return_ty(&mut self, ty: Option<impl ToString>) -> &mut Self {
        self.return_ty = ty.map(|s| s.to_string());
        self
    }

    /// Set if this signature should be prefixed with `pub`
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.is_pub = is_pub;
        self
    }

    /// Set the name of this function.
    pub fn set_name(&mut self, name: impl ToString) -> &mut Self {
        self.name = name.to_string();
        self
    }
}

impl internal::Annotations for FunctionSignature {
    fn annotations_mut(&mut self) -> &mut Vec<Annotation> {
        &mut self.annotations
    }
}

impl internal::Generics for FunctionSignature {
    fn generics_mut(&mut self) -> &mut Vec<Generic> {
        &mut self.generics
    }
    fn generics(&self) -> &[Generic] {
        self.generics.as_slice()
    }
}

impl SrcCode for FunctionSignature {
    fn generate(&self) -> String {
        let template = r#"
        {{ annotations | join(sep="
        ") }}
        {% if self.is_pub %}pub {% endif %}{% if self.is_async %}async {% endif %}fn {{ self.name }}{% if has_generics %}<{{ generic_keys | join(sep=", ") }}>{% endif %}({{ parameters | join(sep=", ") }}) -> {{ return_ty }}{% if has_generics %}
            where
                {% for generic in generics %}{{ generic.name }}: {{ generic.traits | join(sep=" + ") }},
                {% endfor %}{% endif %}"#;
        let mut context = Context::new();
        context.insert("self", &self);
        context.insert(
            "return_ty",
            &self.return_ty.as_ref().unwrap_or(&"()".to_string()),
        );
        context.insert("has_generics", &!self.generics().is_empty());
        context.insert("generics", &self.generics());
        context.insert(
            "generic_keys",
            &self
                .generics()
                .iter()
                .map(|g| g.name())
                .collect::<Vec<&str>>(),
        );
        context.insert("annotations", &self.annotations.to_src_vec());
        context.insert("parameters", &self.parameters.to_src_vec());
        Tera::one_off(template, &context, false).unwrap()
    }
}

/// Represents the function/method's body
#[derive(Default, Serialize, Clone)]
pub struct FunctionBody {
    body: String,
    annotations: Vec<Annotation>,
}

impl internal::Annotations for FunctionBody {
    fn annotations_mut(&mut self) -> &mut Vec<Annotation> {
        &mut self.annotations
    }
}

impl SrcCode for FunctionBody {
    fn generate(&self) -> String {
        let template = r#"
            {{ annotations | join(sep="
            ") }}
            {{ self.body }}
        "#;
        let mut ctx = Context::new();
        ctx.insert("self", &self);
        ctx.insert(
            "annotations",
            &self
                .annotations
                .iter()
                .map(|a| a.generate())
                .collect::<Vec<String>>(),
        );
        Tera::one_off(template, &ctx, false).unwrap()
    }
}

impl Function {
    /// Create a new function
    pub fn new(name: impl ToString) -> Self {
        Self {
            signature: FunctionSignature::new(name),
            ..Self::default()
        }
    }
    /// Add a new parameter to this function
    pub fn add_parameter(&mut self, param: Parameter) -> &mut Self {
        self.signature.parameters.push(param);
        self
    }
    /// Set the return type of this function
    pub fn set_return_ty(&mut self, ty: impl ToString) -> &mut Self {
        self.signature.return_ty = Some(ty.to_string());
        self
    }
    /// Set if this function is public
    pub fn set_is_pub(&mut self, is_pub: bool) -> &mut Self {
        self.signature.set_is_pub(is_pub);
        self
    }
    /// Set if this function is async
    pub fn set_is_async(&mut self, is_async: bool) -> &mut Self {
        self.signature.set_is_async(is_async);
        self
    }
    /// Set the body of the function, this should be valid Rust source code syntax.
    pub fn set_body(&mut self, body: impl ToString) -> &mut Self {
        self.body.body = body.to_string();
        self
    }
    /// Add an annotation before the body of the function
    pub fn add_body_annotation(&mut self, annotation: impl Into<Annotation>) -> &mut Self {
        self.body.add_annotation(annotation);
        self
    }
}

impl internal::Annotations for Function {
    fn annotations_mut(&mut self) -> &mut Vec<Annotation> {
        self.signature.annotations_mut()
    }
}

impl internal::Generics for Function {
    fn generics_mut(&mut self) -> &mut Vec<Generic> {
        &mut self.signature.generics
    }
    fn generics(&self) -> &[Generic] {
        self.signature.generics.as_slice()
    }
}

/// Represents a single parameter to a `Function`
#[derive(Serialize, Default, Clone)]
pub struct Parameter {
    name: String,
    ty: String,
    annotations: Vec<Annotation>,
}
impl Parameter {
    /// Create a new parameter
    ///
    /// Example
    /// -------
    /// ```
    /// use proffer::*;
    ///
    /// let param = Parameter::new("foo", "usize").generate();
    /// let expected = "foo: usize";
    /// assert_eq!(expected, &param);
    /// ```
    ///
    pub fn new(name: impl ToString, ty: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            ty: ty.to_string(),
            ..Self::default()
        }
    }
}

impl internal::Annotations for Parameter {
    fn annotations_mut(&mut self) -> &mut Vec<Annotation> {
        &mut self.annotations
    }
}

impl SrcCode for Parameter {
    fn generate(&self) -> String {
        let template = "{% for annotation in annotations %}{{ annotation }} {% endfor %}{{ self.name }}: {{ self.ty }}";
        let mut ctx = Context::new();
        ctx.insert("self", &self);
        ctx.insert("annotations", &self.annotations.to_src_vec());
        Tera::one_off(template, &ctx, false).unwrap()
    }
}

impl SrcCode for Function {
    fn generate(&self) -> String {
        let template = r#"
        {{ function_signature }}
        {
            {{ body }}
        }
        "#;
        let mut context = Context::new();
        context.insert("body", &self.body.generate());
        context.insert("function_signature", &self.signature.generate());
        Tera::one_off(template, &context, false).unwrap()
    }
}
