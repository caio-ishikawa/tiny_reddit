# tiny_reddit

Tiny Reddit is a command line interface for Reddit. The goal is to have a lightweight and blazingly fast client running entirely on the terminal.

Written with Rust using [cursive](https://github.com/gyscos/cursive), and [roux](https://github.com/Kibouo/roux.rs) for the interface and the reddit API wrapper respectively.

<img src="https://i.imgur.com/M2g0nas.gif" height="250"/>

## TODO:
1. Make commands for opening comments and posts more intuitive.
2. Add ability to log in.
3. Improve comment layout.
4. Add ability to upvote/downvote both comments and posts.
5. Optimize.
6. Add command to load more posts/comments.
7. Add ability to read posts.
8. Figure out how to implement images.

## BUGS/UNINTENTIONAL FEATURES:
~~1. Comments only work for the rust subreddit currently.~~ fixed

2. App crashes when the number of comments exceeds the limit given in the get_comments function.

3. Box width ramdonly increases/decreases when changing subreddits.
