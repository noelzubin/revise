mod conductor;
mod error;
mod store;

use conductor::Conductor;
use store::ID;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    CreateDeck {
        name: String,
    },
    ListDecks,
    Add {
        deck: ID,
        desc: String,
    },
    Edit {
        id: ID,
        desc: String,
    },
    View {
        id: ID,
    },
    Remove {
        id: ID,
    },
    Review {
        deck_id: ID,
    },
    ReviewCard {
        id: ID,
    },
    List {
        deck_id: Option<ID>,
        #[structopt(long)]
        all: bool,
    },
}

fn main() {
    let opts = Opt::from_args();

    let cond = Conductor::new();

    match opts {
        Opt::CreateDeck { name } => cond.add_deck(&name),
        Opt::ListDecks => cond.list_decks(),
        Opt::Add {
            deck: deck_id,
            desc,
        } => cond.add_card(deck_id, &desc),
        Opt::Edit { id, desc } => cond.edit_card(id, &desc),
        Opt::View { id } => cond.view_card(id),
        Opt::Remove { id } => cond.remove_card(id),
        Opt::List { deck_id, all } => cond.list_cards(deck_id, all),
        Opt::Review { deck_id } => cond.review(deck_id),
        Opt::ReviewCard { id } => cond.review_by_id(id),
    }
}
