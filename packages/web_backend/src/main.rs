
use rust_compiler::{ execute_program };
use rocket::{ post, launch, routes, FromForm };
use rocket::form::Form;
use rocket::fs::TempFile;

use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct KsCodeRequest<'r> {
    code: &'r str,
    emit_llvm: bool,
}

#[post("/run", data = "<code_request>")]
fn run_ks_code(code_request: Json<KsCodeRequest<'_>>) { 
    println!("{:?}", code_request);
 }
#[launch]
fn start() -> _ {
    rocket::build().mount("/", routes![run_ks_code])
}