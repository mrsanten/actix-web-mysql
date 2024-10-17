use super::AppState;
use std::sync::{atomic::AtomicU32, Arc};

pub mod group_controller;
pub mod index_controller;
pub mod user_controller;

pub use group_controller::init as init_group_controller;
pub use index_controller::init as init_index_controller;
pub use user_controller::init as init_user_controller;

/// Logs the route accessed and increments the connection counter atomically.
/// 
/// # Arguments
/// 
/// * `route` - The route accessed, provided as a static string.
/// * `connections` - A reference to an atomic counter (inside an Arc) tracking the number of connections.
fn log_request(route: &'static str, connections: &Arc<AtomicU32>) {
    // Increment the connection count atomically with relaxed ordering for performance.
    let con = connections.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!("Route accessed: {}\n\tActive connections: {}", route, con + 1);
}