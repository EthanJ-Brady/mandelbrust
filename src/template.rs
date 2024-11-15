use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub templates: HashMap<String, Vec<Template>>,
}

#[derive(Debug, Deserialize)]
pub struct Template {
    x: f64,
    y: f64,
    z: f64,
    m: u32,
}
