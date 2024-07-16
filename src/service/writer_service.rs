use std::{env, fs::File, io::Write};
use uuid::Uuid;

use super::base_64_decoder::get_file_contents_from_base_64;

pub fn write_file(data: String) -> String {

    let file_name = env::var("TEMP_FILE_PREFIX").expect("Cannot find TEMP_FILE_PREFIX");

    let input_res = get_file_contents_from_base_64(data);

    let path = format!("/storage/{}-{}.cpp", file_name, Uuid::new_v4());

    println!("path: {:?}", path);

    
    match input_res {
        Ok(data) => {
            
            let mut file = File::create(&path).expect(format!("Cannot create file {}", &path).as_str());
            let _ = file.write(&data);

            return path;
        },

        Err(_) => {}
    };

    return "".to_string();
    
}