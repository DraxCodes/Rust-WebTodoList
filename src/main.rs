#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rocket_contrib;
extern crate serde;
extern crate lazy_static;

mod context;
mod todo;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use rocket::request::Form;
use todo::TodoList;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TODOS: Mutex<TodoList> = Mutex::new(TodoList::new());
}

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
        todos: TODOS.lock().unwrap().get()
    };

    return Template::render("todos", &context);
}

#[post("/todos", data = "<userdata>")]
fn add_todo(userdata: Form<TodoItem>) -> Redirect {
    TODOS.lock().unwrap().add_to_list(userdata.added_todo.clone());
    Redirect::to(uri!(todos))
}

#[derive(FromForm)]
struct TodoItem {
    added_todo: String
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, todos, add_todo])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
