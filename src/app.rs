use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use thiserror::Error;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-actix-tut-2.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
enum PostError {
    #[error("Id of the post is invalid")]
    InvalidId,
    #[error("Post not found")]
    PostNotFound,
    #[error("Server error")]
    ServerError,
}
struct PostId(usize);
struct PostParams {
    id: PostId,
}
#[component]
fn Post() -> impl IntoView {
    let query = use_params::<PostParams>();
    let id = move || query.with(|q| q.as_ref().map(|q| q.id).map_err(|_| PostError::InvalidId));
    let post = create_resource(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(id.0)
                .await
                .map(|data| data.ok_or(PostError::PostNotFound))
                .map_err(|_| PostError::ServerError)
                .flatten(),
        }
    });

    view!{}
}


#[server(GetPost, "/api")]
async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
    Ok(POSTS.iter().find(|p| p.id == id))
}