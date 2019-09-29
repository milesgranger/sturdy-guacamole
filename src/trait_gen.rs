use serde::Serialize;

use crate::traits::SrcCode;

#[derive(Serialize, Default)]
pub struct Trait {
    pub name: String,
    pub trait_bounds: Vec<String>,
}

impl Trait {
    pub fn new<S: ToString>(name: S) -> Self {
        let mut t = Self::default();
        t.name = name.to_string();
        t
    }
}

impl SrcCode for Trait {
    fn generate(&self) -> String {
        unimplemented!("SrcCode not implemented for Trait")
    }
}
