use core::panic;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::{ptsite::Discount, DataItem};
use sqlite::{self, Connection};

const TABLE_NAME: &str = "resources";

// Create a table in a database if this table does not exists
pub fn create_table(db_name: &str) -> Connection {
    let conn = sqlite::open(db_name).unwrap();
    let creat = "
        CREATE TABLE IF NOT EXISTS resources (
            ID INTEGER PRIMARY KEY AUTOINCREMENT,
            title           TEXT, 
            desc            TEXT,
            src_link        TEXT,
            finished        INTEGER,
            download        INTEGER,
            upload          INTEGER,
            size            REAL,
            publish_time    INTEGER,
            last_update     INTEGER,

            discount        INTEGER,
            discount_due    INTEGER
        );
    ";
    conn.execute(creat).unwrap();
    conn
}

pub fn delete_db<P: AsRef<Path>>(db_name: P) {
    fs::remove_file(db_name).unwrap();
}

fn insert_item(conn: &Connection, data: &DataItem) {
    let ins = data.to_insert_statement();
    match conn.execute(ins.as_str()) {
        Ok(_) => (),
        Err(e) => {
            println!("QUERY: {}", ins);
            panic!("{:?}", e);
        }
    }
}

/// find out whether an item is in database. Same item means
///  OPTION A. same title and same site
///  OPTION B. same id on the same site
pub fn query_same_item(conn: &Connection, data: &DataItem) -> Vec<(usize, DataItem)> {
    let sel = format!(
        "
        SELECT * from resources where (
            src_link like '{}'
        );
    ",
        data.src_link
    );

    let mut res = vec![];
    conn.iterate(sel, |nvs| {
        let item = DataItem::from_name_value_pairs(nvs);
        res.push(item);
        true
    })
    .unwrap();
    res
}

fn update_item(conn: &Connection, id: usize, data: &DataItem) {
    let update = format!(
        "
        UPDATE {}
        SET {}
        WHERE ID = {}
    ",
        TABLE_NAME,
        data.to_name_value_string(),
        id
    );
    conn.execute(&update).expect(&update);
}

fn delete_item(conn: &Connection, id: usize) {
    let del = format!(
        "
        DELETE FROM {}
        WHERE ID = {}
    ",
        TABLE_NAME, id
    );
    conn.execute(del).unwrap();
}

pub fn insert_or_update_item(conn: &Connection, data: &DataItem) {
    let mut rs = query_same_item(conn, data);
    rs.sort_by_key(|x| x.0);
    match rs.len() {
        0 => insert_item(conn, data),
        1 => update_item(conn, rs[0].0, data),
        n => {
            // delete the 1 ~ n - 1 items and update the last one
            rs.iter()
                .take(n - 1)
                .for_each(|(id, _)| delete_item(conn, *id));
            update_item(conn, rs.iter().last().unwrap().0, data)
        }
    }
}

pub fn insert_or_update_batch(conn: &Connection, items: Vec<DataItem>) {
    items.iter().for_each(|x| insert_or_update_item(conn, x))
}

// panic if parse failed
fn parse<F: FromStr>(s: &str) -> F
where
    <F as FromStr>::Err: std::fmt::Debug,
{
    str::parse(s).expect(&format!("parse {:?} failed", s))
}

fn discount_id(discount: &Option<Discount>) -> i32 {
    match discount {
        None => -1,
        Some(x) => match x {
            Discount::DoubleFree => 0,
            Discount::Free => 1,
            _ => 2,
        },
    }
}

impl DataItem {
    fn to_insert_statement(&self) -> String {
        let discount_id: i32 = discount_id(&self.discount);
        let due = match self.discount_due {
            None => "-1".to_string(),
            Some(x) => x.to_string(),
        };
        let ins = format!(
            "
            INSERT INTO resources (
                title, desc, 
                finished, download, upload, 
                size, publish_time, last_update,
                src_link, discount, discount_due
            ) VALUES (
                '{}', '{}', 
                {}, {}, {}, 
                {}, {}, {},
                '{}', {}, {}
            )",
            self.title.replace("'", "''"),
            self.desc.replace("'", "''"),
            self.finished,
            self.download,
            self.upload,
            self.size,
            self.publish_time,
            self.last_update,
            self.src_link,
            discount_id,
            due
        );
        ins
    }

    fn to_name_value_string(&self) -> String {
        let mut mains = format!(
            "
            title = '{}',
            desc = '{}',

            finished = {},
            download = {},
            upload = {},

            size = {},
            publish_time = {},
            last_update = {},

            src_link = '{}',

            discount = {}
        ",
            self.title.replace("'", "''"),
            self.desc.replace("'", "''"),
            self.finished,
            self.download,
            self.upload,
            self.size,
            self.publish_time,
            self.last_update,
            self.src_link,
            discount_id(&self.discount)
        );

        match self.discount_due {
            None => (),
            Some(x) => {
                mains.push_str(&format!(", discount_due = {}", x));
            }
        }

        mains
    }

    fn from_name_value_pairs(nvs: &[(&str, Option<&str>)]) -> (usize, Self) {
        let mut title: Option<&str> = None;
        let mut desc: Option<&str> = None;
        let mut finished: Option<u32> = None;
        let mut download: Option<u32> = None;
        let mut upload: Option<u32> = None;
        let mut size: Option<f32> = None;
        let mut publish_time: Option<i64> = None;
        let mut last_update: Option<i64> = None;
        let mut src_link: Option<&str> = None;
        let mut id: Option<usize> = None;

        for (name, value) in nvs {
            match *name {
                "title" => title = value.to_owned(),
                "desc" => desc = value.to_owned(),
                "finished" => finished = value.to_owned().map(parse),
                "download" => download = value.to_owned().map(parse),
                "upload" => upload = value.to_owned().map(parse),
                "size" => size = value.to_owned().map(parse),
                "publish_time" => publish_time = value.to_owned().map(parse),
                "last_update" => last_update = value.to_owned().map(parse),
                "src_link" => src_link = value.to_owned(),
                "ID" => id = value.to_owned().map(parse),
                "discount" => (),
                "discount_due" => (),
                unknown_key => {
                    println!("unknown_key: {}", unknown_key);
                    panic!();
                }
            }
        }

        let data = DataItem {
            title: title.unwrap().to_owned(),
            desc: desc.unwrap().to_owned(),
            finished: finished.unwrap(),
            download: download.unwrap(),
            upload: upload.unwrap(),
            size: size.unwrap(),
            publish_time: publish_time.unwrap(),
            last_update: last_update.unwrap(),
            src_link: src_link.unwrap().to_owned(),
            // TODO: None here is a expedient
            discount: None,
            discount_due: None,
        };
        (id.unwrap(), data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /// this function is used for test data generation
    fn gen_test_data() -> DataItem {
        DataItem::new(
            "ddd",
            "this is a test",
            5,
            0,
            4,
            1.0,
            45211,
            "https://www.okpt.net/details.php?id=4553&hit=1",
            None,
            None,
        )
    }

    #[test]
    fn test_insert_twice_then_query() {
        let db_name = "./data/test/i2q.sqlite";
        let conn = create_table(db_name);
        let data = gen_test_data();

        // insert once
        insert_item(&conn, &data);
        let res = query_same_item(&conn, &data);
        assert_eq!(res.len(), 1);

        // insert twice
        insert_item(&conn, &data);
        let res = query_same_item(&conn, &data);
        assert_eq!(res.len(), 2);

        // if conn is not dropped here, file descriptor of db_name
        // may prevent immediate removal
        drop(conn);
        // delete the test file
        delete_db(db_name);
    }

    #[test]
    fn test_insert_or_update_item() {
        let db_name = "./data/test/iou.sqlite";
        let conn = create_table(db_name);
        let data = gen_test_data();

        insert_or_update_item(&conn, &data);
        insert_or_update_item(&conn, &data);
        insert_or_update_item(&conn, &data);
        let res = query_same_item(&conn, &data);
        assert_eq!(res.len(), 1);

        drop(conn);
        delete_db(db_name);
    }
}
