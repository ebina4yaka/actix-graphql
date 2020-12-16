use crate::db::manager::DataPgPool;
use crate::db::photo::{Photo, PhotoNewForm, PhotoUpdateForm};
use crate::graphql::schema::{Context, NewPhoto, UpdatePhoto};
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::Error;
use log::debug;

pub struct PhotoRepository;

impl PhotoRepository {
    pub fn all_photos(context: &Context) -> Result<Vec<Photo>, Error> {
        use crate::schema::photos::dsl::*;
        let conn = &context.pool.get().unwrap();
        photos.load(conn)
    }
    pub fn insert_photo(context: &Context, new_photo: NewPhoto) -> Result<Photo, Error> {
        use crate::schema::photos::dsl::*;
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        let photo_form: PhotoNewForm = (&new_photo).into();
        let rows_inserted = insert_into(photos).values(&photo_form).get_result(conn)?;
        Ok(rows_inserted)
    }
    pub fn update_photo(
        context: &Context,
        pkey: i32,
        update_photo: UpdatePhoto,
    ) -> Result<Photo, Error> {
        use crate::schema::photos::dsl::*;
        use diesel::dsl::update;
        let conn = &context.pool.get().unwrap();
        let photo_form: PhotoUpdateForm = (&update_photo).into();
        let rows_inserted = update(photos.filter(id.eq(pkey)))
            .set(&photo_form)
            .get_result(conn)?;
        Ok(rows_inserted)
    }
    pub fn any_post_photos(pool: &DataPgPool, keys: &[i32]) -> Result<Vec<Photo>, Error> {
        use crate::schema::photos::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = photos.filter(post_id.eq_any(keys));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.get_results::<Photo>(conn)
    }
}
