use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserData {
    pub users: HashMap<String, f64>
}

pub fn get(data: &Arc<Mutex<UserData>>, key: &String) -> Option<f64> {
    data.lock().unwrap().get(key)
}

pub fn add_balance(data: &Arc<Mutex<UserData>>, user: &String, amount: f64) -> Option<f64> {
    data.lock().unwrap().add_balance(user, amount)
}

impl UserData {
    pub fn add_balance(&mut self, user: &String, amount: f64) -> Option<f64> {
        match self.users.get_mut(user) {
            Some(mut user_balance) => {
                let mut new_balance = *user_balance + amount;
                user_balance = &mut new_balance;
                Some(*user_balance + amount)
            },
            None => {
                self.users.insert(user.to_owned(), amount);
                None
            }
        }
    }

    pub fn get(&self, key: &String) -> Option<f64> {
        match self.users.get(key) {
            Some(amt) => {
                Some(*amt)
            },
            None => {
                None // very L code
            },
        }
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self { users: HashMap::<String, f64>::new() }
    }
}
