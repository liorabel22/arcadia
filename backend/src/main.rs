mod handlers;
mod models;
mod periodic_tasks;
mod repositories;
mod routes;
mod services;
mod tracker;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web::Data};
use periodic_tasks::scheduler::run_periodic_tasks;
use reqwest::Url;
use routes::init;
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashSet, env};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use arcadia_backend::{Arcadia, Error, OpenSignups, Result, api_doc::ApiDoc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Ok(env_path) = dotenvy::dotenv() {
        println!("Loading environment from {}", env_path.display());
    } else {
        println!("No .env present, using env vars from the host instead");
    }

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let host = env::var("ACTIX_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("ACTIX_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Server running at http://{host}:{port}");

    let open_signups = if env::var("ARCADIA_OPEN_SIGNUPS").unwrap() == "true" {
        OpenSignups::Enabled
    } else {
        OpenSignups::Disabled
    };

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET env var is not set");

    let site_name = env::var("ARCADIA_SITE_NAME").expect("ARCADIA_SITE_NAME env var is not set");

    let frontend_url = env::var("ARCADIA_FRONTEND_URL")
        .ok()
        .and_then(|s| Url::parse(&s).ok())
        .expect("ARCADIA_FRONTEND_URL env var is not set");

    let tracker_url = env::var("ARCADIA_TRACKER_URL")
        .ok()
        .and_then(|s| Url::parse(&s).ok())
        .expect("ARCADIA_TRACKER_URL malformed or missing");

    let tracker_api_key =
        env::var("ARCADIA_TRACKER_API_KEY").expect("ARCADIA_TRACKER_API_KEY env var is not set");

    let tracker_announce_interval: u32 = env::var("ARCADIA_TRACKER_ANNOUNCE_INTERVAL")
        .expect("ARCADIA_TRACKER_ANNOUNCE_INTERVAL env var is not set")
        .parse()
        .expect("ARCADIA_TRACKER_ANNOUNCE_INTERVAL is not a valid u32");

    let tracker_announce_interval_grace_period: u32 =
        env::var("ARCADIA_TRACKER_ANNOUNCE_INTERVAL_GRACE_PERIOD")
            .expect("ARCADIA_TRACKER_ANNOUNCE_INTERVAL_GRACE_PERIOD env var is not set")
            .parse()
            .expect("ARCADIA_TRACKER_ANNOUNCE_INTERVAL_GRACE_PERIOD is not a valid u32");

    let allowed_torrent_clients = env::var("ARCADIA_ALLOWED_TORRENT_CLIENTS")
        .ok()
        .map(|s| {
            s.split(',')
                .map(|s| s.trim().as_bytes().to_vec())
                .collect::<HashSet<Vec<u8>>>()
        })
        .expect("ARCADIA_ALLOWED_TORRENT_CLIENTS env var is not set");

    let global_upload_factor: f64 = env::var("ARCADIA_GLOBAL_UPLOAD_FACTOR")
        .expect("ARCADIA_GLOBAL_UPLOAD_FACTOR env var is not set")
        .parse()
        .expect("ARCADIA_GLOBAL_UPLOAD_FACTOR env var is not a valid f64");

    let global_download_factor: f64 = env::var("ARCADIA_GLOBAL_DOWNLOAD_FACTOR")
        .expect("ARCADIA_GLOBAL_DOWNLOAD_FACTOR env var is not set")
        .parse()
        .expect("ARCADIA_GLOBAL_DOWNLOAD_FACTOR env var is not a valid f64");

    let tmdb_api_key = env::var("TMDB_API_KEY").ok();
    if tmdb_api_key.is_none() {
        println!("TMDB_API_KEY env var is not set. TMDB data fetching won't be available")
    }

    let smtp_host = env::var("SMTP_HOST").ok();
    let smtp_port = env::var("SMTP_PORT").ok().and_then(|s| s.parse().ok());
    let smtp_username = env::var("SMTP_USERNAME").ok();
    let smtp_password = env::var("SMTP_PASSWORD").ok();
    let smtp_from_email = env::var("SMTP_FROM_EMAIL").ok();
    let smtp_from_name = env::var("SMTP_FROM_NAME").ok();

    // Log email configuration status
    if smtp_host.is_some()
        && smtp_port.is_some()
        && smtp_username.is_some()
        && smtp_password.is_some()
        && smtp_from_email.is_some()
        && smtp_from_name.is_some()
    {
        println!("Email service configured and enabled");
    } else {
        println!("Email service not configured - emails will be skipped");
    }

    let arc = Data::new(Arcadia {
        pool: pool.clone(),
        open_signups,
        jwt_secret: jwt_secret.clone(),
        site_name: site_name.clone(),
        frontend_url: frontend_url.clone(),
        tracker_url: tracker_url.clone(),
        tracker_api_key: tracker_api_key.clone(),
        tracker_announce_interval,
        tracker_announce_interval_grace_period,
        allowed_torrent_clients: allowed_torrent_clients.clone(),
        global_download_factor,
        global_upload_factor,
        tmdb_api_key,
        smtp_host,
        smtp_port,
        smtp_username,
        smtp_password,
        smtp_from_email,
        smtp_from_name,
    });
    let arc_periodic_tasks = arc.clone();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(arc.clone())
            .configure(init) // Initialize routes
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/swagger-json/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(format!("{host}:{port}"))?
    .run();

    tokio::spawn(async {
        if let Err(e) = run_periodic_tasks(arc_periodic_tasks).await {
            eprintln!("Error running cron tasks: {e:?}");
        }
    });

    server.await
}
