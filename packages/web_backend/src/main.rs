
use rust_compiler::{ execute_program };
use rocket::{ post, launch, routes, FromForm };
use rocket::form::Form;
use rocket::fs::TempFile;

use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct KsCodeRequest {
    code: String,
    emit_llvm: bool,
}
#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct KsCodeResponse {
    llvm_code: String,
    return_value: f64,
}

#[post("/run", data = "<code_request>")]
fn run_ks_code(code_request: Json<KsCodeRequest>) -> Json<KsCodeResponse> { 
    let execute_result = execute_program(code_request.code.clone(), code_request.emit_llvm);
    println!("{:?} || {:?}", code_request, execute_result);
    match &execute_result.llvm_code {
        Some(code) => Json(KsCodeResponse{ llvm_code: String::from(code.as_str()), return_value: execute_result.return_value }),
        None => Json(KsCodeResponse{ llvm_code: String::from(""), return_value: execute_result.return_value })
    }
 }
#[launch]
fn start() -> _ {
    rocket::build().mount("/", routes![run_ks_code])
}