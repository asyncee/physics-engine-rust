use std::collections::HashMap;

use crate::shader::Shader;

pub struct ResourceManager {
    shaders: HashMap<String, Shader>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }

    pub fn load_shader(&mut self, name: &str) {
        let shader = Shader::from_files(
            format!("shaders/{name}.vert").as_str(),
            format!("shaders/{name}.frag").as_str(),
        );
        self.shaders.insert(name.to_string(), shader);
    }

    pub fn get_shader(&self, name: &str) -> &Shader {
        self.shaders.get(name).unwrap()
    }
}
