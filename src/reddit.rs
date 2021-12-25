use roux::Subreddit;
//use tokio;

pub struct Posts {
    pub subreddit: Vec<String>,
    pub title: Vec<String>,
    pub author: Vec<String>,
    pub score: Vec<f64>,
    pub comment_num: Vec<u64>
}

pub async fn hot_posts(sub: &str) -> Posts {
    let subreddit = Subreddit::new(sub);

    let hot = subreddit.hot(25, None).await;
    let mut titles: Vec<String> = Vec::new();
    let mut subreddits: Vec<String> = Vec::new();
    let mut authors: Vec<String> = Vec::new();
    let mut scores: Vec<f64> = Vec::new();
    let mut comments: Vec<u64> = Vec::new();

    match hot  {
        //data.data.children[0..20][0].data.title
        Ok(data) => {
            for data in data.data.children {
                titles.push(data.data.title.to_owned());
                subreddits.push(data.data.subreddit.to_owned());
                authors.push(data.data.author.to_owned());
                scores.push(data.data.score.to_owned());
                comments.push(data.data.num_comments);
            }
        }, 
        Err(_err) => panic!("Error getting posts")
    };

    let posts = Posts{subreddit: subreddits, title: titles, author: authors, score: scores, comment_num: comments};

    return posts
}
