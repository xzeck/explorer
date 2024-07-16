
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InputDTO {
    base_64_code: String,
    functions: Vec<String>,
    compilers: String,
    args: Vec<String>
}

impl InputDTO {
    pub fn get_base_64_code(&self) -> &str {
        return &self.base_64_code;
    }

    pub fn get_functions(&self) -> &Vec<String> {
        return &self.functions;
    }

    pub fn get_compilers(&self) -> &String {
        return &self.compilers;
    }

    pub fn get_args(&self) -> &Vec<String> {
        return &self.args;
    }
}