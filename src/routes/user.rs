use std::sync::{Arc, Mutex};

use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use actix_web::{post};
use todo_macro::todo_app;
use crate::db::{self, UserRole};

#[derive(Serialize, Deserialize)]
struct GetUsersResponse {
    users: Vec<db::User>
}


#[todo_app]
struct CreateUserResponse {
    id: String
}

#[derive(Serialize, Deserialize)]
struct SigninResponse {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    role: UserRole
}

#[post("/signup")]
pub async fn sign_up(request: Json<CreateUserRequest>, db: Data<Arc<Mutex<db::Db>>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let user = db.users.iter().find(|u| {
        u.username == request.username
    });

    match user {
        Some(_) => {
            HttpResponse::BadRequest().body("User already exists")
        },
        None => {
            let id = db.create_user(request.username.clone(), request.password.clone(), request.role.clone());
            HttpResponse::Ok().json(CreateUserResponse{
                id: id
            })
        }     
    }
}

#[post("/signin")]
pub async fn sign_in(request: Json<CreateUserRequest>, db: Data<Arc<Mutex<db::Db>>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let user = db.get_user_by_username(request.username.clone());
    match user {
        Some(user) => {
            if user.password == request.password {
                let claims = Claims {
                    sub: user.id.clone(),
                    exp: 10000000000000000
                };

                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
                HttpResponse::Ok().json(SigninResponse {
                    token
                })
            } else {
                HttpResponse::BadRequest().body("Incorrect creds")
            }
        },
        None => {
            HttpResponse::BadRequest().body("User doesnt exist")
        }
    }
}