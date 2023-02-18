use mongodb::bson::oid::ObjectId;
use serde::Serializer;

// credit: https://github.com/thedodd/wither/issues/62#issuecomment-753716870
pub fn objectid_serializer<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
      Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
      None => serializer.serialize_none()
    }
}