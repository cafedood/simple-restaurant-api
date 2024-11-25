use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize)]
struct MenuItem {
    name: String,
    cooking_time: u32,  // minutes
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct Order {
    items: Vec<MenuItem>,
}

type TableId = u32;
type Orders = HashMap<TableId, Order>;

struct AppState {
    orders: Arc<Mutex<Orders>>,
}

/// Add new items for a table
/// Parameters: table_id, new_items
/// 
/// The cooking time is generated randomly based on requirements: 
/// The application MAY assign a length of time for the item to prepare as a random time between 5-15 minutes.
async fn add_items(
    data: web::Data<AppState>,
    table_id: web::Path<TableId>,
    new_items: web::Json<Vec<String>>,
) -> impl Responder {
    let mut orders = data.orders.lock().unwrap();
    let order = orders.entry(table_id.into_inner()).or_default();
    let mut rng = rand::thread_rng();

    for item_name in new_items.into_inner() {
        let cooking_time = rng.gen_range(5..=15); 
        println!("Generated cooking time: {} for {}", cooking_time, item_name);
        order.items.push(MenuItem {
            name: item_name,
            cooking_time,
        })
    }
    HttpResponse::Ok().json(order.clone())
}

/// Remove an item for a table
/// Parameters: tableid, item_name
async fn remove_item(
    data: web::Data<AppState>,
    path: web::Path<(TableId, String)>,
) -> impl Responder {
    let (table_id, item_name) = path.into_inner();
    let mut orders = data.orders.lock().unwrap();

    if let Some(order) = orders.get_mut(&table_id) {
        order.items.retain(|item| item.name != item_name);
        HttpResponse::Ok().json(order.clone())
    } else {
        println!("Cannot find table {} when 'remove_item'", table_id);
        HttpResponse::NotFound().body("Table not found!")
    }
}

/// List all items for a table
/// Parameters: table_id
async fn get_items(
    data: web::Data<AppState>,
    table_id: web::Path<TableId>,
) -> impl Responder {
    let orders = data.orders.lock().unwrap();
    let tableid = &table_id.into_inner();
    if let Some(order) = orders.get(tableid) {
        HttpResponse::Ok().json(order.clone())
    } else {
        println!("Cannot find table {} when 'get_item'", tableid);
        HttpResponse::NotFound().body("Table not found")
    }
}

/// Get a specific item for a table
/// Parameters: table_id, item_name
async fn get_item(
    data: web::Data<AppState>,
    path: web::Path<(TableId, String)>,
) -> impl Responder {
    let (table_id, item_name) = path.into_inner();
    let orders = data.orders.lock().unwrap();

    if let Some(order) = orders.get(&table_id) {
        let mut matches = Vec::new();
        for it in &order.items {
            if it.name == item_name {
                matches.push(it);
            }
        }
        if matches.len() > 0 {
            HttpResponse::Ok().json(matches.clone())
        } else {
            HttpResponse::NotFound().body("Item not found")
        }
    } else {
        HttpResponse::NotFound().body("Table not found")
    }
}

/// Main 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orders = Arc::new(Mutex::new(HashMap::new()));
    let app_state = web::Data::new(AppState { orders });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route(
                "/tables/{table_id}/orders",
                web::post().to(add_items),
            )
            .route(
                "/tables/{table_id}/orders/{item_name}",
                web::delete().to(remove_item),
            )
            .route(
                "/tables/{table_id}/orders",
                web::get().to(get_items),
            )
            .route(
                "/tables/{table_id}/orders/{item_name}",
                web::get().to(get_item),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4) // same as example https://actix.rs/docs/server 
    .run()
    .await
}


