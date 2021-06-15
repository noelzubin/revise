use crate::sm2;
use crate::store::{SqliteStore, Store, ID};
use chrono::{DateTime, Duration, Utc};
use colored::*;
use std::fmt;
use std::io::{stdin, stdout, Write};

pub struct Conductor<S: Store> {
    store: S,
}

impl Conductor<SqliteStore> {
    pub fn new() -> Self {
        let store = SqliteStore::new();
        Conductor { store }
    }

    pub fn add_item(&self, desc: &str) {
        self.store.add_item(desc).unwrap();
    }

    pub fn edit_item(&self, id: ID, desc: &str) {
        self.store.edit_item(id, desc).unwrap();
    }

    pub fn view_item(&self, id: ID) {
        let item = self.store.get_item(id).unwrap();
        println!("{}", item);
    }

    pub fn remove_item(&self, id: ID) {
        self.store.remove_item(id).unwrap();
    }

    // TODO: display as table 
    pub fn list_items(&self, qry: Option<String>) {
        let mut items = self.store.get_items().unwrap();

        if let Some(qry) = qry {
            items = items.into_iter().filter(|item| item.desc.contains(&qry)).collect();
        }

        for item in items.iter() {
            println!("{}\n", item);
        }
    }

    pub fn review_by_id(&self, id: ID) {
        let item = self.store.get_item(id).unwrap();
        self.review_item(item);
    }

    pub fn review(&self) {
        let item = self.store.get_items().unwrap();
        for item in item
            .into_iter()
            .filter(|item| item.data.next_show_date < Utc::now())
        {
            self.review_item(item);
        }
    }

    fn review_item(&self, item: Item) {
        println!("{}", item);

        let first_char = loop {
            let mut buf = String::new();
            print!("input value: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut buf).unwrap();
            let buf = &buf.trim();

            let first_char = buf.chars().next().unwrap();
            if buf.len() != 1 || "qs012345".find(first_char).is_none() {
                println!("invalid input");
                continue;
            }

            break first_char;
        };

        match first_char {
            'q' => {
                panic!("quitting");
            }
            's' => {
                return;
            }
            n => {
                let n: u32 = n.to_digit(10).unwrap();

                if n > 5 {
                    println!("invalid input");
                }

                let now = Utc::now();

                let prev = sm2::SmValue {
                    repetitions: item.data.repetition,
                    interval: item.data.interval,
                    ease_factor: item.data.ease_factor,
                };

                let value = sm2::calc(n, prev);

                let new_item_data = ItemData {
                    interval: value.interval,
                    repetition: value.repetitions,
                    ease_factor: value.ease_factor,
                    next_show_date: now + Duration::days(value.interval),
                };

                let rev = Review {
                    item_id: item.id,
                    review_time: now,
                };

                println!(
                    "next review date: {}",
                    new_item_data
                        .next_show_date
                        .with_timezone(&chrono::Local)
                        .format("%Y-%m-%d %H:%M")
                        .to_string()
                );

                self.store.update_item(item.id, new_item_data).unwrap();
                self.store.add_review(rev).unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct Item {
    pub id: i64,
    pub desc: String,
    pub created_at: DateTime<Utc>,
    pub data: ItemData,
}

#[derive(Debug)]
pub struct ItemData {
    pub interval: i64,
    pub repetition: i64,
    pub ease_factor: f64,
    pub next_show_date: DateTime<Utc>,
}

pub struct Review {
    pub review_time: DateTime<Utc>,
    pub item_id: i64,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {}\nease factor: {}\t interval: {} \t repetition: {}\nnext show date: {}",
            self.id,
            self.desc.bright_green().bold().underline(),
            self.data.ease_factor.to_string().bold(),
            self.data.interval.to_string().bold(),
            self.data.repetition.to_string().bold(),
            self.data
                .next_show_date
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M")
                .to_string()
                .yellow()
        )
    }
}
