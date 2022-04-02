extern crate httplib;

use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter,Rejection};

use httplib::handlers;
use httplib::models;
use httplib::types::{ItemsDb};

#[tokio::main]
async fn main() {
    let items_db: ItemsDb = Arc::new(Mutex::new(HashMap::new()));
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let routes = root.with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}