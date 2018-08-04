use rocket::Outcome::{self, Failure, Success};
use rocket::Request;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, State};
use std::convert::From;
use std::fmt;
use chrono::prelude::*;
use diesel::prelude::*;
use models;
use db;

#[derive(Clone, Debug, Deserialize)]
pub struct AccessTokenAuth {
    pub token: models::AccessToken,
    pub user: models::User
}

impl<'a, 'r> FromRequest<'a, 'r> for AccessTokenAuth {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> Outcome<Self, (Status, ()), ()> {
        use ::schema::access_tokens::dsl::*;
        use ::schema::users::dsl::*;

        let pool = req.guard::<State<db::PgPool>>()?;
        let conn = match pool.get() {
            Ok(c) => c,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
        };
        
        let components_vec = match req.headers().get_one("Authorization") {
            Some(v) => v,
            None => return Failure((Status::Unauthorized, ())),
        };

        let components = components_vec.split(' ').collect::<Vec<&str>>();
        let decoded_value = match components[0] {
            "Bearer" => components[1],
            _ => return Failure((Status::Unauthorized, ())),
        };

        let token_obj: models::AccessToken = match access_tokens.filter(token.eq(decoded_value)).first(&conn) {
            Ok(t) => {
                let task_typed : models::AccessToken = t;
                let now = Utc::now();
                if task_typed.expires_at < now {
                    return Failure((Status::Unauthorized, ()))
                };
                task_typed
            },
            _ => return Failure((Status::Unauthorized, ())),
        };

        let user_obj = match users.find(token_obj.user_id).first(&conn) {
            Ok(u) => {
                let user_typed : models::User = u;
                if !user_typed.is_active {
                    return Failure((Status::Unauthorized, ()))
                };
                user_typed
            },
            _ => return Failure((Status::Unauthorized, ())),
        };

        Success(AccessTokenAuth {
            token: token_obj,
            user: user_obj,
        })
    }
}

