extern crate cursive;
mod reddit;
use roux::Subreddit;
use std::io;
use tokio;
use cursive::{Cursive, CursiveExt};
use cursive::traits::*;
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView, TextView, TextArea};
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let mut siv = Cursive::new();
    let sub = String::from("all");

    siv.add_global_callback('q', |s| s.quit());
    siv.load_toml(include_str!("theme/style.toml")).unwrap();
    start_page(&mut siv, sub);
}

// Recursive function that displays 'r/all' by default and calls itself when user changes subreddit //
fn start_page(siv: &mut Cursive, sub:String) {
    // gets 'r/all' posts and formats as ListView //
    let all_posts= reddit::hot_posts(sub.as_str());
    let default_posts = block_on(all_posts);
    let list = format_list(default_posts).scrollable();
        // Creates the Dialog view for the list of posts //
    let dialog = Dialog::new().title(sub).content(list);
    

    // Creates the input box for the subreddit changes //
    let sub_dialog = Dialog::new().title("subs").content(EditView::new().on_submit(|s, text| {
        if text.contains("sub") {
            let input: Vec<&str> = text.split_whitespace().collect();
            s.pop_layer();
            start_page(s, input[1].to_string());
        }
        if text.contains("com ") {
            let input: Vec<&str> = text.split_whitespace().collect();
            s.pop_layer();
            let output = load_comments(s, input[1].to_string(), input[2]);
            s.add_layer(output.scrollable());
        }
    }));

    // Organizes the views into a linear layour //
    let output = LinearLayout::vertical()
                            .child(sub_dialog)
                            .child(dialog);
     
    // Adds the linear layout as a ncurses layer //
    siv.add_layer(output);
    siv.run();
}

fn load_comments(siv: &mut Cursive, sub: String, id: &str) -> LinearLayout { 
    let comm= reddit::get_comments(sub, id);
    let comments = block_on(comm);
    let mut list = ListView::new();

    let output = format_comments(comments);
    return output;
}

fn format_comments(comments: reddit::Comments) -> LinearLayout {
    let mut list = ListView::new();
    for idx in 0..comments.content.len() {
        let mut cont = String::new();
        let mut author = String::new();

        author.push_str(comments.author[idx].as_str());
        cont.push_str(comments.content[idx].as_str());
        list.add_child(author.as_str(), EditView::new());
        list.add_child(cont.as_str(), EditView::new());
    }

    let output = LinearLayout::vertical().child(Dialog::new().title("test").content(list.scrollable()).fixed_height(120).fixed_width(120));
    return output;
}

// Formats the reddit posts for UI //
fn format_list(posts: reddit::Posts) -> ListView {
    let mut list = ListView::new();

    for idx in 0..posts.title.len() {
        let mut content = String::new();
        let mut author = String::new();
        let mut comments = String::new();
        //author.push_str(" ");
        author.push_str(posts.score[idx].to_string().as_str());
        author.push_str(detect_score_bound(posts.score[idx]).as_str());
        content.push_str("     | ");
        content.push_str(posts.id[idx].as_str());
        content.push_str(" | ");
        content.push_str(posts.title[idx].as_str());
        list.add_child(&content,  EditView::new());
        author.push_str("user: ");
        author.push_str(posts.author[idx].as_str());
        list.add_child(&author, EditView::new());
        comments.push_str("     | comments: ");
        comments.push_str(posts.comment_num[idx].to_string().as_str());
        list.add_child(&comments, EditView::new().on_submit(|s, text| {
            s.add_layer(Dialog::new().title(text));
        }));
        list.add_child("-----|", EditView::new());
    }
    return list;
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





