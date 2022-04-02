use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    let id = warp::path::param().and_then(|id: u64| async move {
        if id != 0 {
            Ok(format!("Hello #{}", id))
        } else {
            Err(warp::reject::not_found())
        }
    });
    let routes = warp::get().and(
        hello
            .or(id),
    );
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}