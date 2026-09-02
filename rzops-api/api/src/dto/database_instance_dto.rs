use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CreateDatabaseInstanceRequest {
    pub server_id: Option<Uuid>,
    pub name: String,
    pub db_type: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub environment: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub is_self_installed: Option<bool>,
    pub importance: Option<String>,
    pub is_ops_managed: Option<bool>,
    pub port: Option<i32>,
    pub instance_name: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct UpdateDatabaseInstanceRequest {
    pub server_id: Option<Uuid>,
    pub name: Option<String>,
    pub db_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub environment: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub is_self_installed: Option<bool>,
    pub importance: Option<String>,
    pub is_ops_managed: Option<bool>,
    pub port: Option<i32>,
    pub instance_name: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct ListDatabaseInstancesQuery {
    pub status: Option<String>,
    pub environment: Option<String>,
    pub db_type: Option<String>,
    pub server_id: Option<Uuid>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DatabaseInstanceResponse {
    pub id: Uuid,
    pub server_id: Option<Uuid>,
    pub name: String,
    pub db_type: String,
    pub description: Option<String>,
    pub status: String,
    pub environment: Option<String>,
    pub offline_time: Option<DateTime<Utc>>,
    pub is_self_installed: bool,
    pub importance: Option<String>,
    pub is_ops_managed: bool,
    pub port: Option<i32>,
    pub instance_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DatabaseInstanceListResponse {
    pub data: Vec<DatabaseInstanceResponse>,
    pub count: i64,
}
