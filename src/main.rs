#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

extern crate serde_json;

use std::collections::HashMap;

use rocket::Request;
use rocket::response::Redirect;

use std::env;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(get: name = "Unknown"))
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
    let context = TemplateContext { name, items: vec!["One", "Two", "Three"] };
    Template::render("index", &context)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}


fn main() {

    let executable = env::current_exe().unwrap();
    let exe_dir = match executable.parent() {
        Some(parent) => parent,
        _ => panic!()
    };
   // let static_path = exe_dir.join("static");
    let cargo_path = 
    rocket::ignite()
     //   .mount("/", StaticFiles::from(static_path))
     //   .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![index, get])
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}
