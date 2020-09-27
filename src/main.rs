#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rocket_contrib;
extern crate serde;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod context;

#[get("/")]
fn index() -> Template {
    let context = context::IndexContext {
        main: context::MainContext::new(),
        header: String::from("Todo List")
    };

    return Template::render("index", &context);
}

#[get("/todos")]
fn todos() -> Template {
    let context = context::TodosContext {
        main: context::MainContext::new(),
        sample: String::from("Some todo thing"),
        todos: vec![String::from("Buy Bread"), 
                    String::from("Buy milk")]
    };

    return Template::render("todos", &context);
}



fn main() {
    rocket::ignite()
        .mount("/", routes![index, todos])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}