extern crate cursive;

use roux::Subreddit;
use tokio;
use cursive::{Cursive, CursiveExt};
use cursive::traits::*;
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView, TextView};

struct Posts {
    subreddit: Vec<String>,
    title: Vec<String>,
    author: Vec<String>,
    score: Vec<f64>,
    comment_num: Vec<u64>
}

#[tokio::main]
async fn main() {
    let posts = hot_posts().await;

    let mut siv = Cursive::new();

    let mut list = ListView::new();

    for idx in 0..posts.title.len() {
        let mut content = String::new();
        let mut author = String::new();
        let mut comments = String::new();
        author.push_str(" ");
        author.push_str(posts.score[idx].to_string().as_str());
        author.push_str(detect_score_bound(posts.score[idx]).as_str());
        content.push_str("     | ");
        content.push_str(posts.title[idx].as_str());
        list.add_child(&content,  EditView::new());
        author.push_str("user: ");
        author.push_str(posts.author[idx].as_str());
        list.add_child(&author, EditView::new());
        comments.push_str("     | comments: ");
        comments.push_str(posts.comment_num[idx].to_string().as_str());
        list.add_child(&comments, EditView::new());
        list.add_child("-----|", EditView::new());
    }

    siv.add_layer(Dialog::new().title(" r/rust ").content(list.scrollable()));

    siv.add_global_callback('q', |s| s.quit());
    
    siv.run();
}


// Detects the length of the upvote count, so that the beginning of each post title align //
fn detect_score_bound(upvotes: f64) -> String {
    let score = upvotes.to_string();
    if score.len() == 1 {
        "   | ".to_owned()
    }
    else if score.len() == 2 {
        "  | ".to_owned()
    }
    else if score.len() == 3 {
        " | ".to_owned()
    }
    else {
        "| ".to_owned()
    }
}

async fn hot_posts() -> Posts {
    let subreddit = Subreddit::new("rust");

    let hot = subreddit.hot(25, None).await;
    let titles: Vec<String>;
    let subreddits: Vec<String>;
    let authors: Vec<String>;
    let scores: Vec<f64>;
    let comments: Vec<u64>;

    match hot  {
        //data.data.children[0..20][0].data.title
        Ok(data) => {
            titles = data.data.children.iter().map(|n| n.data.title.to_owned()).collect();
            subreddits = data.data.children.iter().map(|n| n.data.subreddit.to_owned()).collect();
            authors = data.data.children.iter().map(|n| n.data.author.to_owned()).collect();
            scores = data.data.children.iter().map(|n| n.data.score.to_owned()).collect();
            comments = data.data.children.iter().map(|n| n.data.num_comments).collect();
        }, 
        Err(_err) => panic!("Error getting posts")
    };

    let posts = Posts{subreddit: subreddits, title: titles, author: authors, score: scores, comment_num: comments};

    return posts
}



