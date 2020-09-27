extern crate serde;

#[derive(Serialize)]
pub struct MainContext {
    pub title: String
}

impl MainContext {
    pub fn new() -> MainContext {
        MainContext { title: String::from("The Rust TodoList") }
    }
}

#[derive(Serialize)]
pub struct IndexContext {
    pub main: MainContext,
    pub header: String
}

#[derive(Serialize)]
pub struct TodosContext {
    pub main: MainContext,
    pub sample: String,
    pub todos: Vec<String>
}