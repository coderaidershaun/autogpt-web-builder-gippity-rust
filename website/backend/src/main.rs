use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FitnessProgress {
    pub id: u64,
    pub user_id: u64,
    pub timezone: String,
    pub progress: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    fitness_progress: HashMap<u64, FitnessProgress>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            fitness_progress: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // FITNESS PROGRESS DATA RELATED
    fn insert_fitness_progress(&mut self, progress: FitnessProgress) {
        self.fitness_progress.insert(progress.id, progress);
    }

    fn get_fitness_progress(&self, id: &u64) -> Option<&FitnessProgress> {
        self.fitness_progress.get(id)
    }

    fn get_all_fitness_progress(&self) -> Vec<&FitnessProgress> {
        self.fitness_progress.values().collect()
    }

    fn delete_fitness_progress(&mut self, id: &u64) {
        self.fitness_progress.remove(id);
    }

    fn update_fitness_progress(&mut self, progress: FitnessProgress) {
        self.fitness_progress.insert(progress.id, progress);
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
    client: Client,
}

#[derive(Serialize, Deserialize)]
struct ExerciseData {
    id: u64,
    name: String,
}

async fn fetch_exercise_data(client: web::Data<Client>) -> impl Responder {
    if let Ok(response) = client.get("https://wger.de/api/v2/exercise").send().await {
        let exercise_data: Vec<ExerciseData> = response.json().await.unwrap_or_else(|_| vec![]);
        HttpResponse::Ok().json(exercise_data)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn create_fitness_progress(app_state: web::Data<AppState>, progress: web::Json<FitnessProgress>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert_fitness_progress(progress.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_fitness_progress(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_fitness_progress(&id.into_inner()) {
        Some(progress) => HttpResponse::Ok().json(progress),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn read_all_fitness_progress(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let fitness_progress = db.get_all_fitness_progress();
    HttpResponse::Ok().json(fitness_progress)
}

async fn update_fitness_progress(app_state: web::Data<AppState>, progress: web::Json<FitnessProgress>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.update_fitness_progress(progress.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_fitness_progress(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.delete_fitness_progress(&id.into_inner());
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
        },
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
        client: Client::new(),
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
            .route("/fitness_progress", web::post().to(create_fitness_progress))
            .route("/fitness_progress", web::get().to(read_all_fitness_progress))
            .route("/fitness_progress/{id}", web::get().to(read_fitness_progress))
            .route("/fitness_progress/{id}", web::put().to(update_fitness_progress))
            .route("/fitness_progress/{id}", web::delete().to(delete_fitness_progress))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/exercises", web::get().to(fetch_exercise_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}