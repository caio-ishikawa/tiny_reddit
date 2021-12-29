extern crate cursive;
mod reddit;
mod formatters;
use tokio;
use cursive::{Cursive, CursiveExt};
use cursive::traits::*;
use cursive::views::{EditView, LinearLayout, ListView, Dialog};
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let mut siv = Cursive::new();
    let sub = String::from("all");

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('b', |s| start_page(s, "all".to_owned()));
    siv.load_toml(include_str!("theme/style.toml")).unwrap();
    start_page(&mut siv, sub);
}

// Recursive function that displays 'r/all' by default and calls itself when user changes subreddit //
fn start_page(siv: &mut Cursive, sub:String) {
    siv.pop_layer();
    // gets 'r/all' posts and formats as ListView //
    let all_posts= reddit::hot_posts(sub.as_str());
    let default_posts = block_on(all_posts);
    let list= formatters::format_posts(default_posts).scrollable();
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
            let output = load_comments(input[1].to_string(), input[1],  input[2]);
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

fn load_comments(sub: String, subreddit_name: &str, id: &str) -> LinearLayout { 
    // Gets comment (type) //
    let comm= reddit::get_comments(sub, id);
    let comments = block_on(comm);

    // Formats the comments for output //
    let output = formatters::format_comments(comments, subreddit_name);
    return output;
}
