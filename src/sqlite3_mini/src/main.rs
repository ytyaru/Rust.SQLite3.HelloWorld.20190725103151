fn main() {
//    let connection = sqlite::open(":memory:").unwrap();
//    let connection = sqlite3::sqlite::open(":memory:").unwrap();
    let connection = sqlite3::open(":memory:").unwrap();
    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users (name, age) VALUES ('Alice', 42);
            INSERT INTO users (name, age) VALUES ('Bob', 69);
            ",
        )
        .unwrap();
    connection
        .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
}

