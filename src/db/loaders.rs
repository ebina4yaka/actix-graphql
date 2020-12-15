use self::diesel::prelude::*;
use crate::db::manager::DataPgPool;
use crate::db::post::Post;
use crate::db::post_repository::PostRepository;
use async_trait::async_trait;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use log::error;
use std::collections::HashMap;

extern crate diesel;

pub struct PostsLoadFn {
    pub pool: DataPgPool,
}

impl PostsLoadFn {
    pub fn posts(&self, keys: &[i32]) -> Vec<Post> {
        let result = PostRepository::any_posts(&self.pool, keys);
        match result {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                Vec::new()
            }
        }
    }
}

#[async_trait]
impl BatchFn<i32, Post> for PostsLoadFn {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Post> {
        let res = self.posts(keys);
        res.iter().map(|p| (p.id, p.clone())).collect()
    }
}

pub type PostsLoader = Loader<i32, Post, PostsLoadFn>;

pub fn create_posts_loader(pool: &DataPgPool) -> PostsLoader {
    Loader::new(PostsLoadFn { pool: pool.clone() }).with_yield_count(100)
}

pub struct Loaders {
    pub posts_loader: PostsLoader,
}

impl Loaders {
    pub fn new(pool: &DataPgPool) -> Loaders {
        Loaders {
            posts_loader: create_posts_loader(pool),
        }
    }
}
