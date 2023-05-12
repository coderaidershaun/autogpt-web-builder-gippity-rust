use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForexPrice {
    pub id: u64,
    pub name: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    forex_prices: HashMap<u64, ForexPrice>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            forex_prices: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // FOREX PRICE CRUD DATA RELATED
    fn insert_forex_price(&mut self, forex_price: ForexPrice) {
        self.forex_prices.insert(forex_price.id, forex_price);
    }

    fn get_forex_price(&self, id: &u64) -> Option<&ForexPrice> {
        self.forex_prices.get(id)
    }

    fn get_all_forex_prices(&self) -> Vec<&ForexPrice> {
        self.forex_prices.values().collect()
    }

    fn update_forex_price(&mut self, forex_price: ForexPrice) {
        self.forex_prices.insert(forex_price.id, forex_price);
    }

    // USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

async fn create_forex_price(
    app_state: web::Data<AppState>,
    forex_price: web::Json<ForexPrice>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_forex_price(forex_price.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_forex_price(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_forex_price(&id.into_inner()) {
        Some(forex_price) => HttpResponse::Ok().json(forex_price),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn read_all_forex_prices(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let forex_prices = db.get_all_forex_prices();
    HttpResponse::Ok().json(forex_prices)
}

async fn update_forex_price(
    app_state: web::Data<AppState>,
    forex_price: web::Json<ForexPrice>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.update_forex_price(forex_price.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db = app_state.db.lock().unwrap();

    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in!")
        }
        _ => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data = web::Data::new(AppState {
        db: Mutex::new(db),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost:") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/forex_price", web::post().to(create_forex_price))
            .route("/forex_price", web::get().to(read_all_forex_prices))
            .route("/forex_price/{id}", web::get().to(read_forex_price))
            .route("/forex_price/{id}", web::put().to(update_forex_price))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}