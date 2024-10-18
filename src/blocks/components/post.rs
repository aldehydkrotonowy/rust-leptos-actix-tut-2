use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::*;

struct Post {
    id: usize,
    title: String,
    content: String,
}
struct PostId(usize);
struct PostParams {
    id: PostId,
}
enum PostError {
    #[error("Id of the post is invalid")]
    InvalidId,
    #[error("Post not found")]
    PostNotFound,
    #[error("Server error")]
    ServerError,
}

#[component]
pub fn Post() -> impl IntoView {
    let query = use_params::<PostParams>();
    let id = move || query.with(|q| q.as_ref().map(|q| q.id).map_err(|e| PostError::InvalidId));
    let post = create_resource(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(id.0)
                .await
                .map(|post| post.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });

    let post_view = move || post.with().map();
}

static POSTS: Vec<Post> = vec![Post {
    id: 0,
    title: "Post Title".to_string(),
    content: "Post ontent".to_string(),
}];

#[server(GetPost, "/api")]
async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
    Ok(POSTS.iter().find(|p| p.id == id))
}
