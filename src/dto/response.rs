use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseDTO {
    output: Option<HashMap<String, Vec<String>>>
}

impl ResponseDTO {
    pub fn new() -> ResponseDTO {
        ResponseDTO {
            output: None
        }
    }

    pub fn set_output(mut self, output: HashMap<String, Vec<String>>) -> ResponseDTO {
        self.output = Some(output);

        return self;
    }
}