use super::log_request;
use super::AppState;
use crate::model::User;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(post_user);
    cfg.service(patch_user);
    cfg.service(delete_user);
}

#[get("/user/{id}")]
async fn get_user(
    user_id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    log_request("GET: /user", &app_state.connections);

    match app_state.database.users.get_user_by_id(&user_id).await {
        Ok(mut user) => match app_state.database.users_to_groups.get_groups_by_user_id(&user.id).await {
            Ok(groups) => {
                user.groups = groups;
                HttpResponse::Ok().json(user)
            }
            Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching groups: {}", e)),
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/user")]
async fn post_user(
    user: web::Json<User>, 
    app_state: web::Data<AppState>
) -> impl Responder {
    log_request("POST: /user", &app_state.connections);

    let mut user = user.into_inner();
    user.id = Uuid::new_v4().to_string();

    match app_state.database.users.add_user(&user).await {
        Ok(_) => {
            if !user.groups.is_empty() {
                if let Err(e) = app_state.database.users_to_groups.add_user_groups(&user.id, &user.groups).await {
                    return HttpResponse::InternalServerError().body(format!("Error adding user groups: {}", e));
                }
            }
            HttpResponse::Created().body(user.id)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error adding user: {}", e)),
    }
}

#[patch("/user")]
async fn patch_user(
    user: web::Json<User>, 
    app_state: web::Data<AppState>
) -> impl Responder {
    log_request("PATCH: /user", &app_state.connections);

    let user = user.into_inner();

    match app_state.database.users.update_user(&user).await {
        Ok(0) => HttpResponse::NotFound().finish(),
        Ok(_) => {
            if let Err(e) = app_state.database.users_to_groups.update_user_groups(&user).await {
                return HttpResponse::InternalServerError().body(format!("Error updating user groups: {}", e));
            }
            HttpResponse::Accepted().json(user)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error updating user: {}", e)),
    }
}

#[delete("/user/{id}")]
async fn delete_user(
    id: web::Path<String>, 
    app_state: web::Data<AppState>
) -> impl Responder {
    log_request("DELETE: /user", &app_state.connections);

    match app_state.database.users.delete_user(id.as_str()).await {
        Ok(0) => HttpResponse::NotFound().finish(),
        Ok(_) => HttpResponse::Ok().body(format!("Deleted user with id: {}", id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error deleting user: {}", e)),
    }
}