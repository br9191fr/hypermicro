
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Rejection};

use crate::models::{ShoppingListItem};

pub type ItemsDb = Arc<Mutex<HashMap<usize, ShoppingListItem>>>;
pub type Result<T> = std::result::Result<T, Rejection>;