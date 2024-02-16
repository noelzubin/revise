use crate::store::{SqliteStore, Store, ID};
use chrono::{DateTime, Duration, Utc};
use colored::*;
use fsrs::{MemoryState, FSRS};
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

    pub fn add_deck(&self, name: &str) {
        self.store.add_deck(name).unwrap();
    }

    pub fn list_decks(&self) {
        let decks = self.store.list_decks().unwrap();
        decks.iter().for_each(|deck| {
            println!("{}.\t{}", deck.id, deck.name);
        });
    }

    pub fn add_card(&self, deck_id: ID, desc: &str) {
        self.store.add_card(deck_id, desc).unwrap();
    }

    // TODO: display as table
    pub fn list_cards(&self, deck_id: Option<ID>, all: bool) {
        let mut cards = self.store.get_cards().unwrap();

        if let Some(deck_id) = deck_id {
            cards = cards
                .into_iter()
                .filter(|card| card.deck_id == deck_id)
                .collect();
        }

        if !all {
            let now = Utc::now();
            cards = cards
                .into_iter()
                .filter(|card| card.next_show_date < now)
                .collect();
        }

        for card in cards.into_iter().map(|mut c| {
            // if listing all cards. show only first line of desc
            c.desc = get_first_line(&c.desc);
            c
        }) {
            println!("{}\n", card);
        }
    }

    pub fn edit_card(&self, id: ID, desc: &str) {
        self.store.edit_card(id, desc).unwrap();
    }

    pub fn view_card(&self, id: ID) {
        let card = self.store.get_card(id).unwrap();
        println!("{}", card);
    }

    pub fn remove_card(&self, id: ID) {
        self.store.remove_card(id).unwrap();
    }

    pub fn review_by_id(&self, id: ID) {
        let card = self.store.get_card(id).unwrap();
        self.review_card(card);
    }

    pub fn review(&self, deck_id: ID) {
        let cards = self.store.get_cards().unwrap();
        for card in cards
            .into_iter()
            .filter(|card| card.deck_id == deck_id)
            .filter(|card| card.next_show_date < Utc::now())
        {
            self.review_card(card);
        }
    }

    fn review_card(&self, card: Card) {
        println!("{}", card);

        let first_char = loop {
            let mut buf = String::new();
            print!("\ninput value (q:quit s:skip 0:again 1:hard 2:good 3:easy): ");
            stdout().flush().unwrap();
            stdin().read_line(&mut buf).unwrap();
            let buf = &buf.trim();

            let first_char = buf.chars().next().unwrap();
            if buf.len() != 1 || "qs0123".find(first_char).is_none() {
                println!("invalid input");
                continue;
            }

            break first_char;
        };

        let last_review = self.store.get_last_review(card.id).unwrap();

        match first_char {
            'q' => {
                panic!("quitting");
            }
            's' => {
                return;
            }
            n => {
                let n: u32 = n.to_digit(10).unwrap();

                if n > 4 {
                    println!("invalid input");
                }

                let fsrs = FSRS::new(Some(&[])).unwrap();

                let last_date = last_review
                    .as_ref()
                    .map(|lr| lr.review_time)
                    .unwrap_or(card.created_at);

                let now = Utc::now();
                let days_elapsed = (now - last_date).num_days() as u32;

                let next_states = fsrs
                    .next_states(
                        last_review.as_ref().map(|r| MemoryState {
                            difficulty: r.difficulty,
                            stability: r.stability,
                        }),
                        0.9,
                        days_elapsed,
                    )
                    .unwrap();

                let next_state = match n {
                    0 => next_states.again,
                    1 => next_states.hard,
                    2 => next_states.good,
                    3 => next_states.easy,
                    _ => panic!("invalid input"),
                };

                let revision = Review {
                    id: 0,
                    card_id: card.id,
                    difficulty: next_state.memory.difficulty,
                    stability: next_state.memory.stability,
                    interval: next_state.interval,
                    last_interval: days_elapsed,
                    review_time: now,
                };

                self.store.add_review(revision).unwrap();
                let next_show_date = now + Duration::days(next_state.interval as i64);
                self.store.update_card(card.id, next_show_date).unwrap();

                println!(
                    "next review date: {}",
                    next_show_date
                        .with_timezone(&chrono::Local)
                        .format("%Y-%m-%d %H:%M")
                        .to_string()
                );
            }
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub id: ID,
    pub deck_id: ID,
    pub deck: String,
    pub desc: String,
    pub next_show_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Deck {
    pub id: ID,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

pub struct Review {
    pub id: ID,
    pub card_id: ID,
    pub interval: u32,
    pub last_interval: u32,
    pub review_time: DateTime<Utc>,
    pub stability: f32,
    pub difficulty: f32,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. \t[{}] {}\n\tnext show date: {}",
            self.id,
            self.deck.white().dimmed(),
            self.desc.bright_green().bold(),
            self.next_show_date
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M")
                .to_string()
                .yellow()
        )
    }
}

fn get_first_line(s: &str) -> String {
    s.lines().next().unwrap().to_string()
}
