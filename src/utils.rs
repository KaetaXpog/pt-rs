use std::{fs, io::Write};

pub trait SaveTo{
    fn save_to(&self, path: &str);
}

impl SaveTo for String{
    fn save_to(&self, path: &str) {
        fs::File::create(path).unwrap()
        .write_all(self.as_bytes()).unwrap();
    }
}

