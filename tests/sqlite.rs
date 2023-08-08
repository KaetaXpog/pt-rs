#[test]
fn test_sqlite(){
    let connection = sqlite::open("./data/test.sqlite").unwrap();

    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
    ";
    match connection.execute(query){
        Ok(_) => (),
        Err(_) => ()
    }

    let query = "SELECT * FROM users WHERE age > 50";

    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .unwrap();
}
