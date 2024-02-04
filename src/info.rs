use std::ops::{Index, IndexMut};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Info {
    name: String,
    email_ids: Vec<String>
}

impl Info{
    pub fn new(name: String, email_ids: Vec<String>) -> Self {
        Self{name, email_ids}
    }
}

impl Index<usize> for Info {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.email_ids[index]
    }
}

impl IndexMut<usize> for Info {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.email_ids[index]
    }
}
