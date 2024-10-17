use actix_web::{web, App, HttpServer};
use actix_web_mysql::config::Config;
use actix_web_mysql::dao::Database;
use actix_web_mysql::{controller, AppState};
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the configuration from the file
    let config = Config::from_file("config.json")?;
    println!("App URL: {}", config.get_app_url());
    println!("Database URL: {}", config.get_database_url());

    // Initialize the database context using the database URL from the config
    let db_context = Database::new(&config.get_database_url()).await?;
    println!("Conn db: {}", config.get_database_url());

    // Create the shared application state with a connection counter and database context
    // - `connections`: An atomic counter to keep track of the number of connections
    // - `context`: The database context wrapped in an Arc for safe sharing between threads
    let app_state = web::Data::new(AppState {
        connections: Arc::new(AtomicU32::new(0)),
        database: Arc::new(db_context),
    });

    // Create the HTTP server and configure the routes/controllers
    let app = HttpServer::new(move || {
        App::new()
            // Add shared application state
            .app_data(app_state.clone())
            // Initialize index, user, and group controllers
            .configure(controller::init_index_controller)
            .configure(controller::init_user_controller)
            .configure(controller::init_group_controller)
    })
    // Bind the server to the URL specified in the config
    .bind(config.get_app_url())?;
    println!("Listening on: {}", config.get_app_url());

    // Run the server and await its completion
    app.run().await.map_err(Into::into)
}