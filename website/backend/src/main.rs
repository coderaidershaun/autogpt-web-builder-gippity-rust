use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use reqwest::Client as HttpClient;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FitnessProgress {
    pub id: u64,
    pub user_id: u64,
    pub progress_data: String,
    pub timezone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    fitness_progresses: HashMap<u64, FitnessProgress>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            fitness_progresses: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // FITNESS_PROGRESS CRUD OPERATIONS
    fn insert_progress(&mut self, progress: FitnessProgress) {
        self.fitness_progresses.insert(progress.id, progress);
    }

    fn get_progress(&self, id: &u64) -> Option<&FitnessProgress> {
        self.fitness_progresses.get(id)
    }

    fn get_all_progresses(&self) -> Vec<&FitnessProgress> {
        self.fitness_progresses.values().collect()
    }

    fn delete_progress(&mut self, id: &u64) {
        self.fitness_progresses.remove(id);
    }

    fn update_progress(&mut self, progress: FitnessProgress) {
        self.fitness_progresses.insert(progress.id, progress);
    }

    // USER DATA RELATED OPERATIONS
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
    http_client: HttpClient,
}

#[async_trait]
trait ExternalDataFetcher {
    async fn fetch_external_data(&self, url: &str) -> Result<String, reqwest::Error>;
}

#[async_trait]
impl ExternalDataFetcher for AppState {
    async fn fetch_external_data(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.http_client.get(url).send().await?;
        let content = response.text().await?;
        Ok(content)
    }
}

async fn create_progress(
    app_state: web::Data<AppState>,
    progress: web::Json<FitnessProgress>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_progress(progress.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_progress(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_progress(&id.into_inner()) {
        Some(progress) => HttpResponse::Ok().json(progress),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn read_all_progresses(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let progresses = db.get_all_progresses();
    HttpResponse::Ok().json(progresses)
}

async fn update_progress(
    app_state: web::Data<AppState>,
    progress: web::Json<FitnessProgress>,
) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.update_progress(progress.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_progress(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.delete_progress(&id.into_inner());
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
        http_client: HttpClient::new(),
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
            .route("/progress", web::post().to(create_progress))
            .route("/progress", web::get().to(read_all_progresses))
            .route("/progress/{id}", web::get().to(read_progress))
            .route("/progress/{id}", web::put().to(update_progress))
            .route("/progress/{id}", web::delete().to(delete_progress))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}