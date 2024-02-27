use std::io;

struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoItem {
    fn update_title(&mut self, new_title: String) {
        self.title = new_title;
    }
    fn update_completion_status(&mut self, completed: bool) {
        self.completed = completed;
    }
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }
    fn add_item(&mut self, title: String) {
        let id = self.items.len() as u64 + 1;
        let new_item = TodoItem {
            id,
            title: title.clone(),
            completed: false,
        };
        self.items.push(new_item);
        println!("Added: {}", title)
    }
    fn list_items(&self) {
        if self.items.is_empty() {
            println!("Your to-do list is empty.");
        } else {
            println!("Your to-do list");
            for item in &self.items {
                let status = if item.completed { "[X]" } else { "[]" };
                println!("{} {} - {}", status, item.id, item.title);
            }
        }
    }
    fn completed_item(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            println!("Completed: {}", item.title);
        } else {
            println!("Item with ID {} not found. ", id);
        }
    }
    fn delete_item(&mut self, id: u64) {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            let deleted_item = self.items.remove(index);
            println!("Deleted: {}", deleted_item.title);
        } else {
            println!("Item with ID {} not found.", id);
        }
    }

    fn update_item_title(&mut self, id: u64, new_title: String) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.update_title(new_title.clone());
            println!("Updated title of item {}: {}", id, new_title);
        } else {
            println!("Item with ID {} not found.", id);
        }
    }
    fn update_item_completion_status(&mut self, id: u64, completed: bool) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.update_completion_status(completed);
            let status = if completed { "[X]" } else { "[]" };
            println!("Item {} {}", id, status);
        } else {
            println!("Item with ID {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("1.  Add Item");
        println!("2.  List Item");
        println!("3.  Complete Item");
        println!("4.  Delete Item");
        println!("5.  Update Item Title");
        println!("6.  Update Item Completion Status");
        println!("7.  Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the title of the new item:");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");
                todo_list.add_item(title.trim().to_string());
            }
            2 => {
                todo_list.list_items();
            }
            3 => {
                println!("Enter the ID of the completed item: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.completed_item(id);
            }
            4 => {
                println!("Enter the ID of the item to delete: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todo_list.delete_item(id);
            }
            5 => {
                println!("Enter the ID of the item to update: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                println!("Enter the new title: ");
                let mut new_title = String::new();
                io::stdin()
                    .read_line(&mut new_title)
                    .expect("Failed to read line");
                todo_list.update_item_title(id, new_title.trim().to_string());
            }
            6 => {
                println!("Enter the ID of the item to update: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read_line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                println!("Mark as completed? (true/false): ");
                let mut completed_input = String::new();
                io::stdin()
                    .read_line(&mut completed_input)
                    .expect("Failed to read line");
                let completed: bool = match completed_input.trim().parse() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                todo_list.update_item_completion_status(id, completed)
            }
            7 => {
                println!("Exiting the program.");
                break;
            }

            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.")
            }
        }
    }

    // println!("Hello, world!");
}
