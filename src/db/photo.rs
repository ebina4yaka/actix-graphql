use crate::db::post::{AssocPost, Post};
use crate::schema::photos;
use chrono::NaiveDateTime;

#[derive(Clone, Eq, PartialEq, Debug, Queryable, Associations, Identifiable)]
#[belongs_to(parent = "Post")]
#[belongs_to(parent = "AssocPost", foreign_key = "post_id")]
#[table_name = "photos"]
pub struct Photo {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub post_id: i32,
}

#[derive(Insertable)]
#[table_name = "photos"]
pub struct PhotoNewForm<'a> {
    pub name: &'a str,
    pub post_id: &'a i32,
    pub description: Option<&'a str>,
}

#[derive(AsChangeset)]
#[table_name = "photos"]
pub struct PhotoUpdateForm<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
}
