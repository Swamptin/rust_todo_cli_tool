use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use json_to_table::json_to_table;

fn main() {
    let error = usage();
    println!("{}", error);
    let action = std::env::args().nth(1).expect(format!("{}", error));
    let item = std::env::args().nth(2).expect(format!("{}", error));

    let mut todo = Todo::new().expect("Initialisation of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "show" {
        todo.show() 
    } else {
        todo.usage();
    }

    fn usage() -> String{
        let message = "--- Usage ---\nThis app can be used to create and update personal TODO items.\nTo run this app please launch it with one of the following options: \n\nadd \"String\": This creates a new TODO item and sets it to true. \ncomplete \"String\": This marks a TODO item as done/false \nshow table: Output the contents of your TODO list. \nUsage: Display this usage message";
        message
    }
}
struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
       let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // serialize json as HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error has occurred: {}", e),
        }
    }

    fn insert(&mut self, key: String){
        // inster a new item into our map.
        // we pass true as a value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    // Function to print the contents of db.json to the screen
    // Originally stolen from Rust By Example
    // Searching returned the json_to_table crate, which is what I will use going forward
    fn show(&mut self) {
        let file = File::open("./db.json").expect("File should open read only");
        let value: serde_json::Value = serde_json::from_reader(file).expect("File should be proper JSON format.");
        let table = json_to_table(&value).to_string();
        println!("{}", table);
    }
    
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn usage(&mut self){
        let message = "--- Usage ---\nThis app can be used to create and update personal TODO items.\nTo run this app please launch it with one of the following options: \n\nadd \"String\": This creates a new TODO item and sets it to true. \ncomplete \"String\": This marks a TODO item as done/false \nshow table: Output the contents of your TODO list. \nUsage: Display this usage message";
        println!("{}",message);
    }
}
