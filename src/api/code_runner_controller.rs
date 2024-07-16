use actix_web::{post, web::{self, Json}, HttpResponse, Responder, Scope};

use crate::{dto::{input::InputDTO, response::ResponseDTO}, service::compile_service};

pub fn get_scope() -> Scope {
    web::scope("")
        .service(compile)
}

#[post("/compile")]
async fn compile(payload: Json<InputDTO>) -> impl Responder {
    let input = payload.0;
    let base_64_code = input.get_base_64_code().to_string();
    let functions = input.get_functions().clone();
    let compilers = input.get_compilers().clone();
    let args_map = input.get_args().clone();

    let output_res = compile_service::compile_cpp_to_assembly(base_64_code, functions, compilers, args_map);
    let response = ResponseDTO::new();

    let output = match output_res {
        Ok(output) => output,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::BadRequest().into();
        }
    };

    let response = response.set_output(output);

    HttpResponse::Ok()
        .json(response)
}
