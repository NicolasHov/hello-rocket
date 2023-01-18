use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::form::FromForm;
use rocket::form::Form;

#[macro_use]
extern crate rocket;

// ---------------------GET with params

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

// --------------------- POST with params

#[get("/")]
fn index(flash: Option<FlashMessage<'_>>) -> String {
    flash.map(|flash| format!("{}: {}", flash.kind(), flash.message()))
         .unwrap_or_else(|| "Welcome!".to_string())
}

#[post("/login/<name>")]
fn login(name: &str) -> Result<&'static str, Flash<Redirect>> {
    if name == "special_user" {
        Ok("Hello, special user!")
    } else {
        Err(Flash::error(Redirect::to(uri!(index)), "Invalid username."))
    }
}


//  --------------------POST with data

#[derive(FromForm)]
struct Task<'r> {
   #[field(validate = len(1..))]
   description: &'r str,
   completed: bool
}

#[post("/", data = "<task>")]
fn new(task: Form<Task<'_>>) -> Flash<Redirect> {
    println!("{:#?}",task.description);
    Flash::success(Redirect::to(uri!(index)), "Task added.")
}

//  -----------------------

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, login, new])
        .mount("/hi", routes![index])
}
