use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InputDTO {
    base64_code: String,
    functions: Vec<String>,
    compiler: String,
    args: Vec<String>,
}

impl InputDTO {
    pub fn get_base64_code(&self) -> &str {
        return &self.base64_code;
    }

    pub fn get_functions(&self) -> &Vec<String> {
        return &self.functions;
    }

    pub fn get_compiler(&self) -> &String {
        return &self.compiler;
    }

    pub fn get_args(&self) -> &Vec<String> {
        return &self.args;
    }
}
