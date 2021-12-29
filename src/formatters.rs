use cursive::traits::*;
use cursive::{Cursive, CursiveExt};
use cursive::views::{ListView, EditView, LinearLayout, Dialog, TextArea, CircularFocus, TextView};
use cursive::align::HAlign;
use crate::reddit;
use cursive::theme::{ColorStyle, PaletteColor, Color};

pub fn format_comments(comments: reddit::Comments, sub: &str) -> LinearLayout {
    let mut list = ListView::new();
    for idx in 0..comments.content.len() {
        let mut cont = String::new();
        let mut author = String::new();

        author.push_str(comments.author[idx].as_str());
        cont.push_str(comments.content[idx].as_str());
        list.add_child("", TextView::new(author));
        list.add_child("", TextView::new(cont));
        list.add_child("",TextView::new("--------\n\n".to_owned()));
    }

    let output = LinearLayout::vertical().child(Dialog::new().title(sub).content(list.scrollable()).fixed_height(120).fixed_width(120));
    return output;
}

// Formats the reddit posts for UI //
pub fn format_posts(posts: reddit::Posts) -> ListView {
    let mut list = ListView::new();
    

    for idx in 0..posts.title.len() {
        // Formats strings for use in the layouts below //
        let mut title_str = String::from("| ");
        let mut comment_str = String::from("| Comments: ");
        let mut user_str = String::from("| User: ");
        let mut score_str = posts.score[idx].to_string(); 
        title_str.push_str(posts.id[idx].as_str());
        title_str.push_str(" | ");
        title_str.push_str(posts.title[idx].as_str());
        comment_str.push_str(posts.comment_num[idx].to_string().as_str());
        user_str.push_str(posts.author[idx].as_str());
        score_str.push_str(detect_score_bound(posts.score[idx]).as_str());

        // Sets up Text Views for use in the horizontal/vertical layouts //
        let mut score = TextView::new(score_str);
        let title = TextView::new(title_str);
        let comments = TextView::new(comment_str);
        let users = TextView::new(user_str);
        score.set_style(ColorStyle::new(PaletteColor::Primary, Color::Rgb(0, 128, 0)));

        let delimiter = TextView::new("-------------------------------\n");

        // Sets up Linear Layout views //
        let vertical = LinearLayout::vertical().child(title).child(users).child(comments).child(delimiter);
        let horizontal = LinearLayout::horizontal().child(score).child(vertical);
;
        
        // Adds Linear Layouts to List View //
        list.add_child("", horizontal);
    }
    return list;
}

pub fn detect_score_bound(upvotes: f64) -> String{
    let score = upvotes.to_string();
    if score.len() == 4 {
        " ".to_owned()
    }
    else if score.len() == 3 {
        "  ".to_owned()
    }
    else if score.len() == 2 {
        "   ".to_owned()
    }
    else if score.len() == 1 {
        "    ".to_owned()
    }
    else { 
        "".to_owned()
    }
}
