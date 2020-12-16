use crate::schema::posts;
use chrono::NaiveDateTime;

#[derive(Clone, Eq, PartialEq, Debug, Queryable, Identifiable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct PostNewForm<'a> {
    pub title: &'a str,
    pub content: Option<&'a str>,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostUpdateForm<'a> {
    pub title: Option<&'a str>,
    pub content: Option<&'a str>,
}

#[derive(Eq, PartialEq, Debug, Queryable, Clone, Identifiable)]
#[table_name = "posts"]
pub struct AssocPost {
    pub id: i32,
}
