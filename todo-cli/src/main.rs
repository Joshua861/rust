use color_eyre::eyre::Result;
use colored::Colorize;
use std::collections::HashMap;
use std::env::args;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        Ok(Todo { map })
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
    fn list(&self) {
        for (k, v) in &self.map {
            match v {
                true => println!("{}", k.bold()),
                false => println!("{}", k.bright_black().strikethrough()),
            }
        }
    }
    fn clear(&mut self, what: String) -> Result<(), std::io::Error> {
        if what == "all" {
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open("db.txt")?;

            f.write_all(b"")?;
        } else if what == "completed" {
            println!("{:#?}", self);
        } else {
            println!("Thats not an option.")
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let action = args().nth(1).expect("Please specify an action");
    let item = args().nth(2).expect("Please specify an item.");

    let mut todo = Todo::new().expect("Inititalizing the db failed.");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => (),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => (),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    } else if action == "clear" {
        todo.clear(item)?;
    } else {
        println!("That isnt one of the commands")
    }

    let mut todo = Todo::new().expect("Inititalizing the db failed.");
    todo.list();

    Ok(())
}
