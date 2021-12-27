# tiny_reddit

Tiny Reddit is a command line interface for Reddit. The goal is to have a lightweight and blazingly fast client running entirely on the terminal.

Written with Rust utilizing cursive, and roux for the interface and the reddit API wrapper respectively.

<img src="https://i.imgur.com/usClEKf.gif" height="250"/>

## TODO:
1. Add ability to log in.
2. Fix comment section layout.
3. Add ability to upvote/downvote both comments and posts.
4. Optimize.

## BUGS/UNINTENTIONAL FEATURES:
1. Comments only work for the rust subreddit currently.
2. Sometimes the app closes by itself (I think it's got to do with the reddit API moreso than nCurses).
