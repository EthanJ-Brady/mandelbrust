use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct FractalTemplates {
    pub templates: HashMap<String, Vec<FractalTemplate>>,
}

impl FractalTemplates {
    pub fn get(&self, key: &str) -> Option<&Vec<FractalTemplate>> {
        self.templates.get(key)
    }
}

#[derive(Debug, Deserialize)]
pub struct FractalTemplate {
    x: f64,
    y: f64,
    z: f64,
    m: u32,
}
