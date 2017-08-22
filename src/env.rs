
use std::collections::HashMap;

#[derive(Debug)]
pub struct EnvI<'a>(HashMap<&'a str, i64>);

impl<'a> EnvI<'a> {

    pub fn new() -> EnvI<'a> {
        return EnvI(HashMap::new());
    }

    pub fn add(&mut self, id: &'a str, value: i64) {
        let ref mut m = self.0;
        m.insert(id, value);
    }

    pub fn lookup(&mut self, id: &'a str) -> Option<&i64> {
        let ref mut m = self.0;
        return m.get(&id);
    }

    pub fn remove(&mut self, id: &'a str) {
        let ref mut m = self.0;
        m.remove(id);
    }
}

#[derive(Debug)]
pub struct EnvF<'a>(HashMap<&'a str, f64>);

impl<'a> EnvF<'a> {

    pub fn new() -> EnvF<'a> {
        return EnvF(HashMap::new());
    }

    pub fn add(&mut self, id: &'a str, value: f64) {
        let ref mut m = self.0;
        m.insert(id, value);
    }

    pub fn lookup(&mut self, id: &'a str) -> Option<&f64> {
        let ref mut m = self.0;
        return m.get(&id);
    }

    pub fn remove(&mut self, id: &'a str) {
        let ref mut m = self.0;
        m.remove(id);
    }
}

#[derive(Debug)]
pub struct EnvS<'a>(HashMap<&'a str, String>);

impl<'a> EnvS<'a> {

    pub fn new() -> EnvS<'a> {
        return EnvS(HashMap::new());
    }

    pub fn add(&mut self, id: &'a str, value: String) {
        let ref mut m = self.0;
        m.insert(id, value);
    }

    pub fn lookup(&mut self, id: &'a str) -> Option<String> {
        let ref mut m = self.0;
        let st2 = &m.get(&id).unwrap()[..];
        return Some(String::from(st2))
    }

    pub fn remove(&mut self, id: String) {
        let ref mut m = self.0;
        m.remove(&id[..]);
    }
}