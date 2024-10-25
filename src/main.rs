use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct KvStore {
    store: HashMap<String, String>
}

impl KvStore {
    fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    fn insert(&mut self , key: String, value: String) {
        self.store.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        self.store.get(Key)
    }

    fn save(&self, filename: &str) -> io::Result<()> {
        let data = serde_json::to_string (&self.store)?;
        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut data =  String::new();
        file.read_to_string(&mut data)?;
        let store = serde_json::from_str(&data).unwrap_or_default();
        Ok(KvStore {store})
    }
}


fn main() -> io::Result<()>  {
    let filename = "kv_store.json";

    let mut kv = KvStore::load(filename).unwrap_or_else(|_| KvStore::new());

    kv.insert("name".to_string(), "Alice".to_string());
    kv.insert("city".to_string(), "Wonderland".to_string());

    if let Some(value) = kv.get("name") {
        println!("name: {}", value);
        
    } else {
        println!("name not found");
    }
    kv.save(filename)?;

    Ok((()))

}