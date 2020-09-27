pub struct TodoItem {
    pub name: String,
    pub completed: bool
}

impl TodoItem {
    pub fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: false
        };
    }
}

pub struct TodoList {
    pub items: Vec<TodoItem>
}

impl TodoList {
    pub fn new() -> TodoList {
        return TodoList { items: Vec::new() };
    }

    pub fn add_to_list(&mut self, name: String) {
        self.items.push(TodoItem::new(name));
    }

    pub fn remove_item(&mut self, pos: usize) {
        self.items.remove(pos);
    }

    pub fn mark_completed(&mut self, pos: usize) {
        self.items[pos].completed = true;
    }

    pub fn print(&self) {
        for (pos, item) in self.items.iter().enumerate() {
            println!("{}: [{}] - {}", pos + 1, item.completed, item.name);
        }
    }

    pub fn get(&self) -> Vec<(String, bool)> {
        let mut result: Vec<(String, bool)> = vec![];
        
        for item in &self.items {
            result.push((item.name.clone(), item.completed.clone()));
        }

        result
    }
}

pub enum CommandType {
    Get,
    Add,
    Remove,
    Complete,
    Invalid
}
