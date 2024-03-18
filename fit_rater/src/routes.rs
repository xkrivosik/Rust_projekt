use rocket::{get, put, post, delete, routes};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: usize,
    name: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

impl<T> ApiResponse<T> {
    fn new(data: T) -> Self {
        ApiResponse { success: true, data }
    }
}

// In-memory database
static mut ITEMS: Vec<Item> = Vec::new();
static mut NEXT_ID: usize = 1;

#[get("/")]
fn index() -> &'static str {
    "Welcome to my web app!"
}

#[get("/items")]
fn get_item() -> Json<ApiResponse<Vec<Item>>> {
    let items: Vec<Item>;
    unsafe {
        items = ITEMS.clone();
    }
    Json(ApiResponse::new(items))
}

#[post("/items", format = "json", data = "<item>")]
fn add_item(item: Json<Item>) -> Json<ApiResponse<usize>> {
    let mut id: usize = 0;
    unsafe {
        id = NEXT_ID;
        NEXT_ID += 1;
        ITEMS.push(Item { id: id, name: item.name.clone() });
    }
    Json(ApiResponse::new(id))
}

#[put("/items/<id>", format = "json", data = "<item>")]
fn update_item(id: usize, item: Json<Item>) -> Json<ApiResponse<()>> {
    let mut found = false;
    unsafe {
        for i in ITEMS.iter_mut() {
            if i.id == id {
                found = true;
                i.name = item.name.clone();
                break;
            }
        }
    }
    if found {
        Json(ApiResponse::new(()))
    } else {
        Json(ApiResponse { success: false, data: () })
    }
}

#[delete("/items/<id>")]
fn delete_item(id: usize) -> Json<ApiResponse<()>> {
    let mut found = false;
    unsafe {
        ITEMS.retain(|item| {
            if item.id == id {
                found = true;
                false
            } else {
                true
            }
        });
    }
    if found {
        Json(ApiResponse::new(()))
    } else {
        Json(ApiResponse { success: false, data: () })
    }
}

#[delete("/items")]
fn delete_all_items() -> Json<ApiResponse<()>> {
    unsafe {
        ITEMS.clear();
    }
    Json(ApiResponse::new(()))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        add_item,
        get_item,
        update_item,
        delete_item,
        delete_all_items // New route for deleting all items
    ]
}
