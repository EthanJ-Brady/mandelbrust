use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct FractalTemplates {
    pub templates: HashMap<String, Vec<FractalTemplate>>,
}

impl FractalTemplates {
    pub fn get(&self, key: &str) -> Option<&Vec<FractalTemplate>> {
        self.templates.get(key)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct FractalTemplate {
    x: f64,
    y: f64,
    z: f64,
    m: u32,
}

pub fn get_fractal_templates() -> Result<FractalTemplates, Box<dyn std::error::Error>> {
    let toml_content = fs::read_to_string("fractal_templates.toml")?;
    let templates: FractalTemplates = toml::from_str(&toml_content)?;
    Ok(templates)
}

pub fn get_fractal_template(name: &str) -> Option<Vec<FractalTemplate>> {
    let templates = match get_fractal_templates() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading fractal_templates.toml: {}", e);
            return None;
        }
    };
    templates.get(name).cloned()
}
