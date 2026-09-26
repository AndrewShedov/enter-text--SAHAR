use actix_files::Files;
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use askama::Template;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::statement::prepared::PreparedStatement;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

// UUID and security constants
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

// Global state with cached prepared statements
struct AppState {
    session: Arc<Session>,
    stmt_select: PreparedStatement,
    stmt_insert: PreparedStatement,
    stmt_delete: PreparedStatement,
}

// --- Anti-CSRF Middleware (Cross-Site Request Forgery Protection) ---
// Check Origin and Referer headers to ensure the request comes from our site
fn validate_origin(req: &HttpRequest) -> Result<(), Error> {
    let origin = req.headers().get("Origin").and_then(|v| v.to_str().ok());
    let referer = req.headers().get("Referer").and_then(|v| v.to_str().ok());

    let is_valid_origin = origin.map_or(false, |o| o == ALLOWED_ORIGIN);
    let is_valid_referer = referer.map_or(false, |r| r.starts_with(ALLOWED_ORIGIN));

    // If there is neither Origin nor Referer, or they do not match the allowed ones - block
    if !is_valid_origin && !is_valid_referer {
        return Err(error::ErrorForbidden(
            "Security Alert: CSRF detected (Invalid Origin/Referer)",
        ));
    }

    Ok(())
}

// --- Routes ---
async fn index(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let id = Uuid::parse_str(PROTO_ID).map_err(error::ErrorInternalServerError)?;

    let mut content = "Database is empty".to_string();
    let mut is_empty = true;

    // Using execute_unpaged for prepared statements (safe and fast)
    if let Ok(res) = state
        .session
        .execute_unpaged(&state.stmt_select, (id,))
        .await
    {
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
    req: HttpRequest, // Parameter for reading HTTP headers
    state: web::Data<AppState>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, Error> {
    // 1. Validate source (Anti-CSRF)
    validate_origin(&req)?;

    // ARTIFICIAL 2-SECOND DELAY WHEN ADDING/UPDATE
    // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    // /ARTIFICIAL 2-SECOND DELAY WHEN ADDING/UPDATE

    let id = Uuid::parse_str(PROTO_ID).map_err(error::ErrorInternalServerError)?;

    // 2. Sanitization: trim edges and protect against empty strings made of spaces
    let new_content = form.content.trim();

    if !new_content.is_empty() {
        // Parameterized query (Absolute protection against CQL injections via PreparedStatement)
        state
            .session
            .execute_unpaged(&state.stmt_insert, (id, new_content.to_string()))
            .await
            .map_err(error::ErrorInternalServerError)?;
    }

    // Check who sent the request: HTMX or a browser without JS
    if req.headers().contains_key("hx-request") {
        // Return only the updated HTML block for targeted replacement
        let tmpl = ContentTemplate {
            content: new_content.to_string(),
            is_empty: new_content.is_empty(),
        };
        let body = tmpl.render().map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    } else {
        // Backup plan: Redirect to the homepage for a full refresh
        Ok(HttpResponse::SeeOther()
            .insert_header(("Location", "/"))
            .finish())
    }
}

async fn delete_content(
    req: HttpRequest, // Parameter for checking headers
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // 1. Validate source (Anti-CSRF)
    validate_origin(&req)?;

    // ARTIFICIAL 2-SECOND DELAY WHEN DELETING TEXT
    // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    // /ARTIFICIAL 2-SECOND DELAY WHEN DELETING TEXT

    let id = Uuid::parse_str(PROTO_ID).map_err(error::ErrorInternalServerError)?;

    // Parameterized query for deletion
    state
        .session
        .execute_unpaged(&state.stmt_delete, (id,))
        .await
        .map_err(error::ErrorInternalServerError)?;

    // Check who sent the request: HTMX or a browser without JS
    if req.headers().contains_key("hx-request") {
        let tmpl = ContentTemplate {
            content: "Database is empty".to_string(),
            is_empty: true,
        };
        let body = tmpl.render().map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    } else {
        // Backup plan: Redirect
        Ok(HttpResponse::SeeOther()
            .insert_header(("Location", "/"))
            .finish())
    }
}

// Automatic schema creation in ScyllaDB
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

    // Preparing statements (PreparedStatement - injection protection + DB performance)
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
            // Incoming data limit (Protection against DoS attacks via huge payloads)
            .app_data(web::FormConfig::default().limit(4096))
            // Strict security headers (Zero 'unsafe-inline' tolerance)
            .wrap(
                middleware::DefaultHeaders::new()
                    // Protection against Clickjacking
                    .add(("X-Frame-Options", "DENY"))
                    // Protection against MIME-sniffing
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add((
                        "Content-Security-Policy",
                        // Disable inline scripts and styles. Allow own Origin ('self') only
                        "default-src 'self'; script-src 'self'; style-src 'self';",
                    ))
                    .add((
                        "Strict-Transport-Security",
                        "max-age=31536000; includeSubDomains; preload",
                    ))
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
