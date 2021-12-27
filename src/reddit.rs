use roux::Subreddit;
use std::collections::HashMap;
//use tokio;

pub struct Posts {
    pub subreddit: Vec<String>,
    pub title: Vec<String>,
    pub author: Vec<String>,
    pub score: Vec<f64>,
    pub comment_num: Vec<u64>,
    pub id: Vec<String>
}

pub struct Comments {
    pub content: Vec<String>, 
    pub score: Vec<i32>,
    pub author: Vec<String>
}

pub async fn hot_posts(sub: &str) -> Posts {
    let subreddit = Subreddit::new(sub);

    let hot = subreddit.hot(25, None).await;
    let mut titles: Vec<String> = Vec::new();
    let mut subreddits: Vec<String> = Vec::new();
    let mut authors: Vec<String> = Vec::new();
    let mut scores: Vec<f64> = Vec::new();
    let mut comments: Vec<u64> = Vec::new();
    let mut ids: Vec<String> = Vec::new();

    match hot  {
        //data.data.children[0..20][0].data.title
        Ok(data) => {
            for (idx, data) in data.data.children.iter().enumerate() {
                titles.push(data.data.title.to_owned());
                subreddits.push(data.data.subreddit.to_owned());
                authors.push(data.data.author.to_owned());
                scores.push(data.data.score.to_owned());
                comments.push(data.data.num_comments);
                ids.push(data.data.id.to_owned());
            }
        }, 
        Err(_err) => panic!("Error getting posts")
    };

    let posts = Posts{subreddit: subreddits, title: titles, author: authors, score: scores, comment_num: comments, id: ids};

    return posts
}

pub async fn get_comments(sub: String, id: &str) -> Comments {
    let subreddit = Subreddit::new(sub.as_str());

    let comments = subreddit.article_comments(id, None, Some(100)).await;

    let mut author: Vec<String> = Vec::new();
    let mut content: Vec<String> = Vec::new();
    let mut score: Vec<i32> = Vec::new();

    match comments {
        Ok(data) => {
            for data in data.data.children {
                author.push(data.data.author.unwrap().to_owned());
                content.push(data.data.body.unwrap().to_owned());
                score.push(data.data.score.unwrap());
            }
        }
        Err(err) => panic!("Error getting comments")
    };

    let comments = Comments{content, author, score};
    return comments;

}
