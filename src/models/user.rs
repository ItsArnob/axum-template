use crate::utils::custom_serde::objectid_serializer;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename(deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "objectid_serializer"
    )]
    pub id: Option<ObjectId>,
    pub username: String,
}
