use cursive::traits::*;
use cursive::{Cursive, CursiveExt};
use cursive::views::{ListView, EditView, LinearLayout, Dialog, TextArea, CircularFocus, TextView};
use crate::reddit;

pub fn format_comments(comments: reddit::Comments, sub: &str) -> LinearLayout {
    let mut list = ListView::new();
    for idx in 0..comments.content.len() {
        let mut cont = String::new();
        let mut author = String::new();

        author.push_str(comments.author[idx].as_str());
        cont.push_str(comments.content[idx].as_str());
        list.add_child("", TextView::new(author));
        list.add_child("", TextView::new(cont));
        list.add_child("",TextView::new("--------\n".to_owned()));
    }

    let output = LinearLayout::vertical().child(Dialog::new().title(sub).content(list.scrollable()).fixed_height(120).fixed_width(120));
    return output;
}

// Formats the reddit posts for UI //
pub fn format_posts(posts: reddit::Posts) -> ListView {
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

pub fn detect_score_bound(upvotes: f64) -> String {
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
