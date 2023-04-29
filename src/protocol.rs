use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

pub enum MessageKind {
    Sync,

}


pub struct SyncMessage {
    //Hash of the range query
    pub id: String,
    /// Legnth of Set "the total len of set"
    pub len: usize,
    /// Range Start and End with the length in int 
    pub start: String,
    pub end: String,
    /// Should start with a big range and get smaller 
    /// when the range gets to 1 that should be where to insert a change the sender willknow the items and make
    /// sure theres no duplicates by itterating to a point where the innitiator sends a sync where the reange_len is 1
    pub range_len: i32,
    //pub range_hash: Vec<u8>,
}

impl SyncMessage {
    pub fn get_range_hash(&mut self) -> u64 {
        let mut s = DefaultHasher::new();
        self.start.hash(&mut s);
        self.end.hash(&mut s);
        s.finish() 
    }

}

