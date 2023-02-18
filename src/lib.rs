use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqliteConnection};

const DB_MIGRATIONS: &[&str] = &[
    "CREATE TABLE User (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL);",
    "CREATE TABLE Category (id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        lowerid INTEGER NOT NULL,
        upperid INTEGER NOT NULL);",
    "CREATE TABLE Food (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        catid INTEGER NOT NULL,
        FOREIGN KEY(id) REFERENCES Category(id));",
    "CREATE TABLE FoodConsumed (
        id INTEGER PRIMARY KEY NOT NULL,
        userid INTEGER NOT NULL,
        foodid INTEGER NOT NULL,
        FOREIGN KEY(foodid) REFERENCES Food(id),
        FOREIGN KEY(userid) REFERENCES User(id));"
];


pub async fn get_db() -> SqliteConnection {
    let new_db = !std::path::Path::new("kebab.db").exists();

    let mut conn = SqliteConnectOptions::new()
        .filename("kebab.db")
        .create_if_missing(true)
        .connect()
        .await
        .unwrap();

    if new_db {
        init_db(&mut conn).await;
    }

    conn
}

async fn init_db(conn: &mut SqliteConnection) {
    for migration in DB_MIGRATIONS {
        let result = sqlx::query(migration)
            .execute(&mut *conn)
            .await;
        if !result.is_ok() {
            panic!("Failed to init new Database at migration {}", migration);
        }
    }
}