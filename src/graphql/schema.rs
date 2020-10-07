use crate::db::manager::DataPgPool;
use crate::db::photo;
use crate::db::photo_repository::PhotoRepository;
use crate::db::post;
use crate::db::post_repository::PostRepository;
use juniper::{EmptySubscription, FieldError, FieldResult, ID};

#[derive(Clone, Debug)]
pub struct Photo {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Photo insert struct")]
pub struct NewPhoto {
    pub name: String,
    pub post_id: i32,
    pub description: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Photo update struct")]
pub struct UpdatePhoto {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Post insert struct")]
pub struct NewPost {
    pub title: String,
    pub content: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Post update struct")]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A Photo returns struct")]
impl Photo {
    pub fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn url(&self) -> String {
        format!("http://hogehoge/{}", self.id)
    }
    pub fn post(&self, context: &Context) -> FieldResult<Post> {
        let post = PostRepository::find_post(context, self.post_id)?;
        Ok(post.into())
    }
}

// diesel Photo to GraphQL Photo
impl From<photo::Photo> for Photo {
    fn from(photo: photo::Photo) -> Self {
        Self {
            id: photo.id,
            post_id: photo.post_id,
            name: photo.name,
            description: photo.description.map_or("".to_string(), |d| d),
        }
    }
}

// diesel NewPhoto to GraphQL PhotoNewForm
impl<'a> From<&'a NewPhoto> for photo::PhotoNewForm<'a> {
    fn from(new_photo: &'a NewPhoto) -> Self {
        Self {
            name: &new_photo.name,
            post_id: &new_photo.post_id,
            description: new_photo.description.as_ref().map(AsRef::as_ref),
        }
    }
}

// diesel UpdatePhoto to GraphQL PhotoUpdateForm
impl<'a> From<&'a UpdatePhoto> for photo::PhotoUpdateForm<'a> {
    fn from(update_photo: &'a UpdatePhoto) -> Self {
        Self {
            name: update_photo.name.as_ref().map(AsRef::as_ref),
            description: update_photo.description.as_ref().map(AsRef::as_ref),
        }
    }
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A Post returns struct")]
impl Post {
    pub fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn content(&self) -> String {
        self.content.clone()
    }
    pub fn photos(&self, context: &Context) -> FieldResult<Vec<Photo>> {
        let photos = PhotoRepository::post_photos(context, self.id)?;
        Ok(photos.into_iter().map(|t| t.into()).collect())
    }
}

// diesel Post to GraphQL Post
impl From<post::Post> for Post {
    fn from(post: post::Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content.expect(""),
        }
    }
}

// diesel NewPost to GraphQL PostNewForm
impl<'a> From<&'a NewPost> for post::PostNewForm<'a> {
    fn from(new_post: &'a NewPost) -> Self {
        Self {
            title: &new_post.title,
            content: new_post.content.as_ref().map(AsRef::as_ref),
        }
    }
}

// diesel UpdatePost to GraphQL PostUpdateForm
impl<'a> From<&'a UpdatePost> for post::PostUpdateForm<'a> {
    fn from(update_post: &'a UpdatePost) -> Self {
        Self {
            title: update_post.title.as_ref().map(AsRef::as_ref),
            content: update_post.content.as_ref().map(AsRef::as_ref),
        }
    }
}

pub struct Query;
pub struct Mutation;

pub struct Context {
    pub pool: DataPgPool,
}
impl juniper::Context for Context {}

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn all_photos(&self, context: &Context) -> FieldResult<Vec<Photo>> {
        PhotoRepository::all_photos(context)
            .and_then(|photos| Ok(photos.into_iter().map(|t| t.into()).collect()))
            .map_err(Into::into)
    }
    fn all_posts(&self, context: &Context) -> FieldResult<Vec<Post>> {
        PostRepository::all_posts(context)
            .and_then(|posts| Ok(posts.into_iter().map(|t| t.into()).collect()))
            .map_err(Into::into)
    }
}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn create_photo(&self, context: &Context, new_photo: NewPhoto) -> Result<Photo, FieldError> {
        let result = PhotoRepository::insert_photo(context, new_photo)?;
        Ok(result.into())
    }
    fn update_photo(
        &self,
        context: &Context,
        id: i32,
        update_photo: UpdatePhoto,
    ) -> Result<Photo, FieldError> {
        let result = PhotoRepository::update_photo(context, id, update_photo)?;
        Ok(result.into())
    }
    fn create_post(&self, context: &Context, new_post: NewPost) -> Result<Vec<Post>, FieldError> {
        PostRepository::insert_post(context, new_post)
            .and_then(|posts| Ok(posts.into_iter().map(|t| t.into()).collect()))
            .map_err(Into::into)
    }
    fn update_post(
        &self,
        context: &Context,
        id: i32,
        update_post: UpdatePost,
    ) -> Result<Post, FieldError> {
        let result = PostRepository::update_post(context, id, update_post)?;
        Ok(result.into())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::<Context>::new())
}
