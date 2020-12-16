use self::diesel::prelude::*;
use crate::db::manager::DataPgPool;
use crate::db::photo::Photo;
use crate::db::photo_repository::PhotoRepository;
use crate::db::post::AssocPost;
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

fn create_assoc_posts(keys: Vec<i32>) -> Vec<AssocPost> {
    keys.into_iter().map(|k| AssocPost { id: k }).collect()
}

pub struct PostPhotosLoadFn {
    pub pool: DataPgPool,
}

impl PostPhotosLoadFn {
    pub fn post_photos(&self, keys: &[i32]) -> Vec<Photo> {
        let query_result = PhotoRepository::any_post_photos(&self.pool, keys);
        match query_result {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                Vec::new()
            }
        }
    }
}

#[async_trait]
impl BatchFn<i32, Vec<Photo>> for PostPhotosLoadFn {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Vec<Photo>> {
        let assoc_posts: Vec<AssocPost> = create_assoc_posts(keys.to_vec());
        let post_photos = self.post_photos(keys).grouped_by(&assoc_posts);
        let result = assoc_posts
            .iter()
            .zip(post_photos)
            .map(|assoc| (assoc.0.id, assoc.1))
            .collect();
        result
    }
}

pub type PostPhotosLoader = Loader<i32, Vec<Photo>, PostPhotosLoadFn>;

pub fn create_post_photos_loader(pool: &DataPgPool) -> PostPhotosLoader {
    Loader::new(PostPhotosLoadFn { pool: pool.clone() }).with_yield_count(100)
}

pub struct Loaders {
    pub posts_loader: PostsLoader,
    pub post_photos_loader: PostPhotosLoader,
}

impl Loaders {
    pub fn new(pool: &DataPgPool) -> Loaders {
        Loaders {
            posts_loader: create_posts_loader(pool),
            post_photos_loader: create_post_photos_loader(pool),
        }
    }
}
