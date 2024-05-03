use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BetData {
    pub users: HashMap<String, f64>
}
