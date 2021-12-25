extern crate cursive;
mod reddit;
use roux::Subreddit;
use tokio;
use cursive::{Cursive, CursiveExt};
use cursive::traits::*;
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView, TextView};

#[tokio::main]
async fn main() {
    let default_posts = reddit::hot_posts("all").await;
    let mut list = ListView::new();
    let mut siv = Cursive::new();

    siv.load_toml(include_str!("theme/style.toml")).unwrap();

    let input = EditView::new();

    let dialog = Dialog::new().title("subreddit:").content(input);

    for idx in 0..default_posts.title.len() {
        let mut content = String::new();
        let mut author = String::new();
        let mut comments = String::new();
        //author.push_str(" ");
        author.push_str(default_posts.score[idx].to_string().as_str());
        author.push_str(detect_score_bound(default_posts.score[idx]).as_str());
        content.push_str("     | ");
        content.push_str(default_posts.title[idx].as_str());
        list.add_child(&content,  EditView::new().with_name("subreddit!"));
        author.push_str("user: ");
        author.push_str(default_posts.author[idx].as_str());
        list.add_child(&author, EditView::new());
        comments.push_str("     | comments: ");
        comments.push_str(default_posts.comment_num[idx].to_string().as_str());
        list.add_child(&comments, EditView::new());
        list.add_child("-----|", EditView::new());
}

    siv.add_layer(LinearLayout::vertical()
        //.title(" r/rust ")
        .child(dialog)
        .child(list.scrollable())
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}


// Detects the length of the upvote count, so that the beginning of each post title align //
fn detect_score_bound(upvotes: f64) -> String {
    let score = upvotes.to_string();
    if score.len() == 1 {
        "    | ".to_owned()
    }
    else if score.len() == 2 {
        "   | ".to_owned()
    }
    else if score.len() == 3 {
        "  | ".to_owned()
    }
    else if score.len() == 4 { 
        " | ".to_owned()
    }
    else {
        "| ".to_owned()
    }
}





