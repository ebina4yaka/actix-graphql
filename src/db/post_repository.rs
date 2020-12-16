use crate::db::manager::DataPgPool;
use crate::db::post::{Post, PostNewForm, PostUpdateForm};
use crate::graphql::schema::{Context, NewPost, UpdatePost};
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::Error;
use log::debug;

pub struct PostRepository;

impl PostRepository {
    pub fn all_posts(context: &Context) -> Result<Vec<Post>, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &context.pool.get().unwrap();
        posts.load(conn)
    }
    pub fn find_post(context: &Context, pkey: i32) -> Result<Post, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &context.pool.get().unwrap();
        let select_query = posts.filter(id.eq(pkey));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.get_result::<Post>(conn)
    }
    pub fn insert_post(context: &Context, new_post: NewPost) -> Result<Vec<Post>, Error> {
        use crate::schema::posts::dsl::*;
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        let post_form: PostNewForm = (&new_post).into();
        insert_into(posts)
            .values(&post_form)
            .get_result(conn)
            .and_then(|_: Post| PostRepository::all_posts(context))
    }
    pub fn update_post(
        context: &Context,
        pkey: i32,
        update_post: UpdatePost,
    ) -> Result<Post, Error> {
        use crate::schema::posts::dsl::*;
        use diesel::dsl::update;
        let conn = &context.pool.get().unwrap();
        let post_form: PostUpdateForm = (&update_post).into();
        let rows_inserted = update(posts.filter(id.eq(pkey)))
            .set(&post_form)
            .get_result(conn)?;
        Ok(rows_inserted)
    }
    pub fn any_posts(pool: &DataPgPool, keys: &[i32]) -> Result<Vec<Post>, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = posts.filter(id.eq_any(keys));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.get_results::<Post>(conn)
    }
}
