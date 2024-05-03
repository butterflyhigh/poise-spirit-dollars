use std::{error::Error, fs::{self, File}, io::{Seek, Write}, sync::{Arc, Mutex}};

use serde::{de::DeserializeOwned, Serialize};

pub mod users;

// i was posessed by the holy spirit and wrote this goofy ahh code

pub struct Database<T: DeserializeOwned + Clone + Default + Serialize + std::fmt::Debug> {
    file: Arc<Mutex<File>>,
    pub data: Arc<Mutex<T>>
}

pub fn create_database<T: DeserializeOwned + Clone + Default + Serialize + std::fmt::Debug>(path: &str) -> Result<Database<T>, Box<dyn Error>> {
    let data = T::default();
    let str_data = serde_json::to_string(&data)?;
    let mut file = fs::File::create(path)?;
    file.write(&str_data.as_bytes())?;

    Ok(Database { file: Arc::new(Mutex::new(file)), data: Arc::new(Mutex::new(data)) })
}

pub fn open_database<T: Default + DeserializeOwned + Clone + Serialize + std::fmt::Debug>(path: &str) -> Result<Database<T>, std::io::Error> {
    let data_str = fs::read_to_string(path)?;
    println!("{}", data_str);
    let data: T = match serde_json::from_str(&data_str) {
        Ok(data) => {
            println!("Success serializing data");
            data
        },
        Err(e) => {
            eprintln!("ERROR: {e}");
            T::default()
        }
    };
    let file = fs::File::create(path)?;
    let db = Database { file: Arc::new(Mutex::new(file)), data: Arc::new(Mutex::new(data)) };

    Ok(db)
}

pub fn sync<T: DeserializeOwned + Clone + Default + Serialize + std::fmt::Debug>(db: &Database<T>) -> Result<(), std::io::Error> {
    let mut file = db.file.lock().expect("Mutex is poisoned! Oh no!");

    file.set_len(0)?;
    file.rewind()?;

    let data = db.data.lock().expect("Mutex is poisoned! Oh no!");
    let read = data.clone();

    file.write(serde_json::to_string(&read)?.as_bytes())?;

    println!("{:#?}", read);

    Ok(())
}
