use crate::dao::Database;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;

pub mod config;
pub mod controller;
pub mod dao;
pub mod model;

// Application state shared across different handlers.
pub struct AppState {
    pub connections: Arc<AtomicU32>, // Tracks the number of active connections.
    pub database: Arc<Database>,      // Shared database context.
}