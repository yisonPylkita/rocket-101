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

#[post("/login", data = "<login>")]
fn login_req(login: Form<LoginReq>) -> String {
    if login.username == "user" && login.password == "123" {
        return "Successfull login".to_string();
    }

    "Login failure".to_string()
}

fn main() {
    rocket::ignite().mount("/api", routes![
        login_req,
    ],)
    .mount("/", rocket_contrib::serve::StaticFiles::from("static"))
    .launch();
}