use std::env;
use std::collections::HashMap;
use serde_json;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let command = &args[1];
    fn load_kvp() -> HashMap<String, String> {
    if std::path::Path::new("kv_store.json").exists() {
        let data = std::fs::read_to_string("kv_store.json").expect("Unable to read file");
        serde_json::from_str::<HashMap<String, String>>(&data).unwrap()
    } else {
        HashMap::new()
    }
    }

    match command.as_str() {
    "set" => {
        let key = &args[2];
        let value = &args[3];
        set(&mut store, key, value);
    }
    "get" => {
        let key = &args[2];
        get(&store, key);
    }
    "delete" => {
        let key = &args[2];
        delete(&mut store, key);
    }
    "list" => {
        list(&store);
    }
    _ => println!("Invalid command!"),
    }

}

fn set(store: &mut HashMap<String, String>, key: &String, value: &String){
    store.insert(key.to_string(), value.to_string());
    save_kvp(store);
}

fn get(store: &HashMap<String, String>, key: &String){
    match store.get(key.to_string().as_str()) {
        Some(value) => println!("{}", value),
        None => println!("Key not found!"),
    }

}

fn delete(store: &mut HashMap<String, String>, key: &String){
    if store.remove(key).is_none() {
        println!("Key not found!");
    }
    save_kvp(store);
}

fn list(store: &HashMap<String, String>){
    for (key, value) in store.iter() {
        println!("{}: {}", key, value);
    }
}   


fn save_kvp(store: &HashMap<String, String>) {
    let serialized = serde_json::to_string(store).unwrap();
    std::fs::write("kv_store.json", serialized).expect("Unable to write file");
}

fn load_kvp() -> HashMap<String, String> {
    let data = std::fs::read_to_string("kv_store.json").expect("Unable to read file");
    serde_json::from_str::<HashMap<String, String>>(&data).unwrap()
}