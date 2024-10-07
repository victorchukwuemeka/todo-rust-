use std::io::{self, Write};
use std::collections::VecDeque;

struct TodoList{
    items : VecDeque<String>,
}

impl TodoList {
    fn new ()-> TodoList{
        TodoList {
            items : VecDeque::new(),
        }
    }

    fn add_item(&mut self, item:String){
        self.items.push_front(item);
    }

    fn remove_item(&mut self)->Option<String>{
        self.items.pop_back()
    }

    fn list_items(&mut self){
        for (index, item) in  self.items.iter().enumerate(){
            println!(" {}: {}", index+1 , item);
        }
    }

    
}

fn main () {
    let mut todo_list = TodoList::new();
    loop{
        println!("Press 1 to add Items");
        println!("Press 2 to delete Items");
        println!(" Press 3 to list Items ");
        println!(" Press 4 to quit ");

        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut item = String::new();
                print!(" enter an item :");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut item).unwrap();
                todo_list.add_item(item.trim().to_string());
            }

            "2" => {
               match todo_list.remove_item() {
                Some(item) => println!("Removed :{}", item),
                None => println!(" No item to remove"),
               } 
            }

            "3" => {
                println!("Things to do : ");
                todo_list.list_items();
            }

            "4" => break,
            __ => println!("Ivalide choice "),
        }
    }
}