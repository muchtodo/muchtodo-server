use chrono::prelude::*;
use std::fmt;
use schema::*;

#[derive(Clone, Debug, Queryable, Serialize, Deserialize, Identifiable, Associations)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub date_joined: DateTime<Utc>,
    pub is_staff: bool,
    pub is_active: bool,
    pub avatar: Option<String>
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Task {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub user_id: i32,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub name: String,
    pub notes: Option<String>,
    pub completable: bool,
    pub completed: bool,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>
}

#[derive(Clone, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[table_name = "clients"]
#[belongs_to(User)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub user_id: i32,
    pub identifier: String,
    pub secret: String,
    pub response_type: String,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Client {{ id: {}, identifier: {}, secret: [REDACTED], response_type: {} }}",
            self.id, self.identifier, self.response_type
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[table_name = "grant_types"]
pub struct GrantType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[table_name = "access_tokens"]
#[belongs_to(User)]
#[belongs_to(Client)]
pub struct AccessToken {
    pub id: i32,
    pub token: String,
    pub client_id: i32,
    pub user_id: i32,
    pub grant_id: i32,
    pub scope: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "access_tokens"]
pub struct NewAccessToken {
    pub client_id: i32,
    pub grant_id: i32,
    pub scope: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[table_name = "refresh_tokens"]
#[belongs_to(AccessToken)]
#[belongs_to(Client)]
pub struct RefreshToken {
    pub id: i32,
    pub token: String,
    pub client_id: i32,
    pub access_token_id: i32,
    pub scope: String,
    pub issued_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    pub client_id: i32,
    pub scope: String,
    pub issued_at: DateTime<Utc>,
}
