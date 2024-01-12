use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// todo list
    pub todos: Vec<TodoItem>,
}

#[derive(Debug)]
pub struct TodoItem {
    pub done: bool,
    pub name: String,
}

impl TodoItem {
    pub fn new(done: bool, name: String) -> Self {
        Self {
            done: done,
            name: name,
        }
    }
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            done: false,
            name: "make a cup of tea".to_string(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            todos: vec![],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_todo(&mut self, todo: String) {
        self.todos.push(TodoItem::new(false, todo))
    }

    pub fn remove_todo(&mut self, index: usize) {
        self.todos.remove(index);
    }
}
