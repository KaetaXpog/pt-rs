use std::{fs, io::Write};
use std::thread::sleep;
use std::time::Duration;

pub trait SaveTo{
    fn save_to(&self, path: &str);
}

impl SaveTo for String{
    fn save_to(&self, path: &str) {
        fs::File::create(path).unwrap()
        .write_all(self.as_bytes()).unwrap();
    }
}

pub fn sleep_secs(n: u64){
    sleep(Duration::from_secs(n))
}
