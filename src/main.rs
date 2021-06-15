mod conductor;
mod error;
mod sm2;
mod store;

use conductor::Conductor;
use store::ID;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    Add { desc: String },
    Edit { id: ID, desc: String },
    View { id: ID },
    Remove { id: ID },
    Review { id: Option<ID> },
    List { qry: Option<String> },
}

fn main() {
    let opts = Opt::from_args();

    let cond = Conductor::new();

    match opts {
        Opt::Add { desc } => cond.add_item(&desc),
        Opt::Edit { id, desc } => cond.edit_item(id, &desc),
        Opt::View { id } => cond.view_item(id),
        Opt::Remove { id } => cond.remove_item(id),
        Opt::List { qry } => cond.list_items(qry),
        Opt::Review { id } => match id {
            Some(id) => cond.review_by_id(id),
            None => cond.review(),
        },
    }
}
