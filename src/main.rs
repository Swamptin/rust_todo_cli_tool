use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufRead;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

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
    // Currently stolen from Rust By Example
    fn show(&mut self) {
       if let Ok(lines) = Self::read_lines("./db.json") {
           // Consumes the iterator returns option String
           let mut out = String::from("");
           for line in lines.flatten() {
               // This can be made prettier using the following sudo-code
               // mut Message = "|";
               // Message = Message.append(line[key]);
               // Message = Message.append(checkboxtype(line[value]));
               // Println!("{}", Message);
               // Found a bit of something that could be good. It's in a youtube video
               if line.contains("{") {
               } else if line.contains("}"){
               } else {
                   let result = line.replace("\":", "\"|");
                   out = format!("{}|{} |\n", &out, &result);
               }
           }
           println!("|----|----|");
           println!("{}", out);
       }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
