use super::log_request;
use super::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_group_by_id);
    cfg.service(post_group);
    cfg.service(patch_group_by_name);
    cfg.service(delete_group_by_name);
}

#[get("/group/{id}")]
async fn get_group_by_id(
    group_id: web::Path<u64>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    log_request("GET: /group/{id}", &app_state.connections);

    let result = app_state
        .database
        .groups
        .get_group_by_id(group_id.into_inner())
        .await;

    match result {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(group) => HttpResponse::Ok().json(group),
    }
}

#[post("/group")]
async fn post_group(
    group: web::Json<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    log_request("POST: /group", &app_state.connections);

    let result = app_state.database.groups.add_group(group.as_str()).await;

    match result {
        Ok(_) => {
            let group_result = app_state
                .database
                .groups
                .get_group_by_name(group.as_str())
                .await;

            match group_result {
                Ok(group) => HttpResponse::Accepted().json(group),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching group: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error adding group: {}", e)),
    }
}

#[derive(Deserialize, Serialize)]
pub struct GroupUpdate {
    pub old: String,
    pub new: String,
}

#[patch("/group")]
async fn patch_group_by_name(
    update: web::Json<GroupUpdate>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    log_request("PATCH: /group", &app_state.connections);

    let result = app_state
        .database
        .groups
        .update_group(&update.old, &update.new)
        .await;

    match result {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error updating group: {}", e)),
        Ok(_) => HttpResponse::Accepted().body(format!("Updated group: {}", update.new)),
    }
}

#[delete("/group/{name}")]
async fn delete_group_by_name(
    name: web::Path<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    log_request("DELETE: /group/{name}", &app_state.connections);

    let result = app_state.database.groups.delete_group(name.as_str()).await;

    match result {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error deleting group: {}", e)),
        Ok(_) => HttpResponse::Ok().body(format!("Successfully deleted group: {}", name)),
    }
}