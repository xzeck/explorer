use base64::{prelude::BASE64_STANDARD, Engine, DecodeError};

pub fn get_file_contents_from_base_64(base_64_data: String) -> Result<Vec<u8>, DecodeError> {

    let decoded = BASE64_STANDARD.decode(base_64_data);

    return decoded
}