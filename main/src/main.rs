use actix_files::Files;
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use askama::Template;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::statement::prepared::PreparedStatement;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

// UUID
const PROTO_ID: &str = "11111111-1111-1111-1111-111111111111";
const ALLOWED_ORIGIN: &str = "http://127.0.0.1:8080";

// --- Structures ---
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    content: String,
    is_empty: bool,
}

#[derive(Template)]
#[template(path = "content.html")]
struct ContentTemplate {
    content: String,
    is_empty: bool,
}

#[derive(Deserialize)]
struct FormData {
    content: String,
}

// Global state with cached queries
struct AppState {
    session: Arc<Session>,
    stmt_select: PreparedStatement,
    stmt_insert: PreparedStatement,
    stmt_delete: PreparedStatement,
}

// --- Anti-CSRF (Anti-CSRF) middleware ---
fn validate_origin(req: &HttpRequest) -> Result<(), Error> {
    if let Some(origin) = req.headers().get("Origin") {
        if let Ok(origin_str) = origin.to_str() {
            if origin_str != ALLOWED_ORIGIN {
                return Err(error::ErrorForbidden("Cross-Site Request Forgery detected"));
            }
        }
    }
    Ok(())
}

// --- Routes ---
async fn index(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let id = Uuid::parse_str(PROTO_ID).map_err(error::ErrorInternalServerError)?;
    
    let mut content = "Database is empty".to_string();
    let mut is_empty = true;

    // Using execute_unpaged for prepared queries
    if let Ok(res) = state.session.execute_unpaged(&state.stmt_select, (id,)).await {
        if let Ok(rows) = res.into_rows_result() {
            if let Ok(Some(row)) = rows.maybe_first_row::<(String,)>() {
                content = row.0;
                is_empty = false;
            }
        }
    }

    let tmpl = IndexTemplate { content, is_empty };
    let body = tmpl.render().map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

async fn save_content(
    req: HttpRequest, // Added a parameter for reading headers
    state: web::Data<AppState>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, Error> {
    validate_origin(&req)?;

    // ARTIFICIAL 2-SECOND DELAY WHEN ADDING/UPDATE
    // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    // /ARTIFICIAL 2-SECOND DELAY WHEN ADDING/UPDATE

    let id = Uuid::parse_str(PROTO_ID).map_err(error::ErrorInternalServerError)?;
    let new_content = form.content.trim();

    if !new_content.is_empty() {
        // Parameterized query (injection protection via PreparedStatement)
        state.session
            .execute_unpaged(&state.stmt_insert, (id, new_content.to_string()))
            .await
            .map_err(error::ErrorInternalServerError)?;
    }

    // Checking who sent the request: HTML or a browser without JS
    if req.headers().contains_key("hx-request") {
        // We return only the updated HTML block
        let tmpl = ContentTemplate {
            content: new_content.to_string(),
            is_empty: new_content.is_empty(),
        };
        let body = tmpl.render().map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    } else {
        // Backup Plan: Redirect to the homepage for a complete refresh
        Ok(HttpResponse::SeeOther().insert_header(("Location", "/")).finish())
    }
}

async fn delete_content(
    req: HttpRequest, // Parameter for reading headers
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    validate_origin(&req)?;

    // ARTIFICIAL 2-SECOND DELAY WHEN DELETING TEXT
    // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    // /ARTIFICIAL 2-SECOND DELAY WHEN DELETING TEXT

    let id = Uuid::parse_str(PROTO_ID).map_err(error::ErrorInternalServerError)?;
    
    state.session
        .execute_unpaged(&state.stmt_delete, (id,))
        .await
        .map_err(error::ErrorInternalServerError)?;

    // Checking who sent the request: HTML or a browser without JS
    if req.headers().contains_key("hx-request") {
        let tmpl = ContentTemplate {
            content: "Database is empty".to_string(),
            is_empty: true,
        };
        let body = tmpl.render().map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    } else {
        // Backup Plan: Redirect to the homepage for a complete refresh
        Ok(HttpResponse::SeeOther().insert_header(("Location", "/")).finish())
    }
}

// Automatic schema creation
async fn initialize_schema(session: &Session) {
    println!("🧪 Checking data schema...");
    session
        .query_unpaged(
            "CREATE TABLE IF NOT EXISTS sahar_prototype.data (
            id uuid PRIMARY KEY,
            content text,
            created_at timestamp
        )",
            &[],
        )
        .await
        .expect("❌ Error creating 'data' table");
    println!("✨ Table 'data' verified/created successfully");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
        .expect("❌ Failed to connect to ScyllaDB");

    initialize_schema(&session).await;

    // Preparing statements (injection protection + speed)
    let stmt_select = session
        .prepare("SELECT content FROM sahar_prototype.data WHERE id = ? LIMIT 1")
        .await
        .expect("Failed to prepare select query");
    let stmt_insert = session
        .prepare("INSERT INTO sahar_prototype.data (id, content, created_at) VALUES (?, ?, toTimestamp(now()))")
        .await
        .expect("Failed to prepare insert query");
    let stmt_delete = session
        .prepare("DELETE FROM sahar_prototype.data WHERE id = ?")
        .await
        .expect("Failed to prepare delete query");

    let app_state = web::Data::new(AppState {
        session: Arc::new(session),
        stmt_select,
        stmt_insert,
        stmt_delete,
    });

    println!("✅ ScyllaDB is ready");
    println!("🚀 SAHAR is running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // Limiting incoming data (DoS protection)
            .app_data(web::FormConfig::default().limit(4096)) 
            // Basic Security Headers
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';"))
                    .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin")),
            )
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/api/content", web::post().to(save_content))
            .route("/api/content/delete", web::post().to(delete_content))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}