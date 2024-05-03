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

pub fn gift(data: &Arc<Mutex<UserData>>, sender: &String, receiver: &String, amount: f64) -> Option<f64> {
    data.lock().unwrap().gift(sender, receiver, amount)
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

    pub fn gift(&mut self, sender: &String, receiver: &String, amount: f64) -> Option<f64> {
        if let Some(sender_balance) = self.users.get(sender).clone() {
            if *sender_balance < amount {
                None
            } else {
                let b = *sender_balance - amount;
                let receiver_balance = self.users.get(receiver).unwrap_or(&0.0).clone();
                self.users.insert(receiver.to_owned(), receiver_balance + amount);
                self.users.insert(sender.to_owned(), b);
                Some(amount)
            }
        } else {
            None
        }
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self { users: HashMap::<String, f64>::new() }
    }
}
