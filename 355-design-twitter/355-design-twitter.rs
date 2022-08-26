use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    iter, vec,
};

// Your Twitter object will be instantiated and called as such:
/*
let obj = Twitter::new();
obj.post_tweet(userId, tweetId);
let ret_2: Vec<i32> = obj.get_news_feed(userId);
obj.follow(followerId, followeeId);
obj.unfollow(followerId, followeeId);
*/

#[derive(Hash, Eq, PartialEq, Debug)]
struct Tweet {
    id: i32,
    seq: usize,
}

#[derive(Default)]
struct User {
    tweets: HashSet<Tweet>,
    followings: HashSet<i32>,
    followers: HashSet<i32>,
}

impl User {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
struct Twitter {
    users: HashMap<i32, User>,
    tweets_count: usize,
    /*
      ["Twitter","postTweet","postTweet","getNewsFeed"]
      [[],[1,5],[1,3],[1]]
    */
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let user = self.users.entry(user_id).or_default();
        self.tweets_count += 1;
        user.tweets.insert(Tweet {
            id: tweet_id,
            seq: self.tweets_count,
        });
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        if let Some(user) = self.users.get(&user_id) {
            let mut all_tweets: Vec<&Tweet> = user
                .followings
                .iter()
                .chain(iter::once(&user_id))
                .map(|following_id| {
                    self.users.get(following_id).unwrap().tweets.iter()
                    // .map(|x| x.clone())
                })
                .flatten()
                .collect();
            all_tweets.sort_by_key(|k| Reverse((*k).seq));
            all_tweets.iter().map(|x| x.id).take(10).collect()
        } else {
            vec![]
        }
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let user = self.users.entry(followee_id).or_default();
        user.followers.insert(follower_id);
        let user = self.users.entry(follower_id).or_default();
        user.followings.insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(user) = self.users.get_mut(&followee_id) {
            user.followers.remove(&follower_id);
        }
        if let Some(user) = self.users.get_mut(&follower_id) {
            user.followings.remove(&followee_id);
        }
    }
}
