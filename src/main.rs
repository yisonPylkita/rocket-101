// TODO: separate API and static pages

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket::request::Form;

#[derive(FromForm)]
struct LoginReq {
    username: String,
    password: String,
}

#[post("/login_req", data = "<login>")]
fn login_req(login: Form<LoginReq>) -> String {
    if login.username == "user" && login.password == "123" {
        return "Successfull login".to_string();
    }

    "Login failure".to_string()
}

fn main() {
    use rocket_contrib::serve;
    rocket::ignite().mount("/", routes![
        login_req,
    ],)
    .mount("/", serve::StaticFiles::new("static", serve::Options::Index | serve::Options::DotFiles))
    .launch();
}