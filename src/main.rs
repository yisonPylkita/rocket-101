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

use rocket_contrib::json::Json;
#[post("/login", data = "<login>")]
fn login(login: Form<LoginReq>) -> Json<> {
    if login.username == "user" && login.password == "123" {
        return "Successfull login".to_string();
    }

    "Login failure".to_string()
}

#[derive(FromForm)]
struct SignupReq {
    username: String,
    password: String,
}

#[post("/signup", data = "<signup>")]
fn signup(signup: Form<SignupReq>) -> String {
    "Login failure".to_string()
}

fn main() {
    rocket::ignite().mount("/api", routes![
        login,
        signup,
    ],)
    .mount("/", rocket_contrib::serve::StaticFiles::from("static"))
    .launch();
}