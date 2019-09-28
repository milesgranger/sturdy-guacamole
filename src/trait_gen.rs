use serde::Serialize;

use crate::traits::SrcCode;

#[derive(Serialize, Default)]
pub struct Trait {
    pub id: String,
    pub trait_names: Vec<String>,
}

impl SrcCode for Trait {
    fn generate(&self) -> String {
        unimplemented!("SrcCode not implemented for Trait")
    }
}
