use rocket::form::Form;
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::response::{Flash, Redirect};
use rocket::Request;
use rocket::*;

// #[macro_use]
extern crate rocket;

// --------------------- GET Index

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("templates/index.html"))
}

// --------------------- GET : Account

#[get("/show_form")]
fn show_form() -> RawHtml<&'static str> {
    return RawHtml(include_str!("templates/account.html"));
}

//  --------------------POST : create account

#[derive(FromForm)]
struct CreateAccount<'r> {
    name: &'r str,
}

#[post("/create_account", data = "<account>")]
fn create_account(account: Form<CreateAccount<'_>>) -> Flash<Redirect> {
    println!("{:#?}", account.name);
    // let mut name = account.name;
    Flash::success(Redirect::to(uri!("/")), account.name)
    // return RawHtml(include_str!("templates/account.html"));
}

// --------------------- DELETE : Account (TO TEST)

#[delete("/delete_account/<name>")]
pub fn delete_account(name: &str) -> String {
    format!("{} deleted", name)
    // Flash::success(Redirect::to(uri!("/")), format!("{} deleted", name))
}
// ---------------------GET with route params

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

//TOFIX: --------------------- GET with query params

#[get("/math?<first>&<second>")]
fn multiply(first: i32, second: i32) -> String {
    //   String::from(
    format!("{}", first * second)
    // );
}

// -------------------- (POST with params)

// #[post("/login/<name>")]
// fn login(name: &str) -> Result<&'static str, Flash<Redirect>> {
//     if name == "special_user" {
//         Ok("Hello, special user!")
//     } else {
//         Err(Flash::error(Redirect::to(uri!(index)), "Invalid username."))
//     }
// }

//  ----------------------- Catchers

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

// ------------------------

#[launch]
fn rocket() -> _ {
    // let not_found_catcher = Catcher::new(404, handle_404);
    // let internal_server_error_catcher = Catcher::new(500, handle_500);
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                hello,
                multiply, // login,
                show_form,
                create_account,
                delete_account
            ],
        )
        // .mount("/account", routes![show_form, create_account])
        .register("/", catchers![internal_error, not_found, default])
}
