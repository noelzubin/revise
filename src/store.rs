use crate::conductor::{Item, ItemData, Review};
use crate::error::{ReviseError, ReviseResult};
use crate::sm2;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub type ID = i64;

pub trait Store {
    fn add_item(&self, desc: &str) -> ReviseResult<()>;
    fn edit_item(&self, id: ID, desc: &str) -> ReviseResult<()>;
    fn update_item(&self, id: ID, data: ItemData) -> ReviseResult<()>;
    fn get_item(&self, id: ID) -> ReviseResult<Item>;
    fn remove_item(&self, id: ID) -> ReviseResult<()>;
    fn get_items(&self) -> ReviseResult<Vec<Item>>;
    fn add_review(&self, review: Review) -> ReviseResult<()>;
}

pub struct SqliteStore {
    conn: Connection,
}

impl Store for SqliteStore {
    fn add_item(&self, desc: &str) -> ReviseResult<()> {
        let data = sm2::SmValue::inital();
        let sql = "INSERT INTO items
        (desc, repetition, interval, ease_factor, next_show_date, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)";

        let now = Utc::now();

        self.conn
            .execute(
                sql,
                params![
                    &desc,
                    data.repetitions,
                    data.interval,
                    data.ease_factor,
                    &now,
                    &now
                ],
            )?;

        Ok(())
    }

    fn edit_item(&self, id: i64, desc: &str) -> ReviseResult<()> {
        let sql = "UPDATE items SET desc = $1 WHERE id = $2";
        let resp = self.conn.execute(sql, params![desc, &id])?;

        if resp == 0 {
            return Err(ReviseError::NotFoundError(id));
        }

        Ok(())
    }

    fn update_item(&self, id: i64, data: ItemData) -> ReviseResult<()> {
        let sql = "UPDATE items SET repetition=$1, interval=$2, ease_factor=$3, next_show_date=$4 where id = $5";

        self.conn
            .execute(
                sql,
                params![
                    data.repetition,
                    data.interval,
                    data.ease_factor,
                    data.next_show_date,
                    id
                ],
            )?;

        Ok(())
    }

    fn get_item(&self, id: i64) -> ReviseResult<Item> {
        let sql = "SELECT id, desc, created_at, repetition, interval, ease_factor, next_show_date FROM items WHERE id = $1";
        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query_map([id], Item::from_row)?;
        let row = rows.next().unwrap()?;
        Ok(row)
    }

    fn remove_item(&self, id: i64) -> ReviseResult<()> {
        self.conn
            .execute("DELETE FROM items WHERE id=$1", &[&id])?;

        Ok(())
    }

    fn get_items(&self) -> ReviseResult<Vec<Item>> {
        let sql = "SELECT id, desc, created_at, repetition, interval, ease_factor, next_show_date FROM items";
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], Item::from_row)?;
        let items = rows.collect::<rusqlite::Result<Vec<Item>>>()?;
        Ok(items)
    }

    fn add_review(&self, review: Review) -> ReviseResult<()> {
        let sql = "INSERT INTO reviews (review_time, item_id) VALUES ($1, $2)";
        self.conn
            .execute(sql, params![review.review_time, review.item_id])?;

        Ok(())
    }
}

impl SqliteStore {
    pub fn new() -> Self {
        let conn = Connection::open(data_path()).unwrap();

        conn.execute(
            "CREATE TABLE if not exists items (
      id integer primary key autoincrement,
      desc text NOT NULL,
      repetition INTEGER NOT NULL,
      interval INTEGER NOT NULL,
      ease_factor real NOT NULL,
      next_show_date text NOT NULL,
      created_at text NOT NULL
      )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS items_desc_key ON items(desc)",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE if not exists reviews (
      id integer primary key autoincrement,
      review_time text NOT NULL,
      item_id integer NOT NULL,
      FOREIGN KEY(item_id) REFERENCES items(id)
    )",
            [],
        )
        .unwrap();

        SqliteStore { conn }
    }
}

pub fn data_dir() -> PathBuf {
    let mut dir = dirs::data_local_dir().expect("failed to find dir");
    dir = dir.join("revise");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn data_path() -> PathBuf {
    let dir = data_dir();
    dir.join("data.sqlite")
}

impl Item {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Item> {
        Ok(Item {
            id: row.get(0)?,
            desc: row.get(1)?,
            created_at: row.get(2)?,
            data: ItemData {
                repetition: row.get(3)?,
                interval: row.get(4)?,
                ease_factor: row.get(5)?,
                next_show_date: row.get(6)?,
            },
        })
    }
}
