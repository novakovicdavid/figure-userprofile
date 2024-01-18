use derive_name::{Name, Named};
use serde::Serialize;
use serde_json::{Map, Value};
use crate::application::ApplicationError;

pub fn to_json_string<T>(serializable: T) -> Result<String, ApplicationError>
    where T: Serialize
{
    serde_json::to_string(&serializable)
        .map_err(|e| ApplicationError::UnexpectedError(e.into()))
}

pub fn to_json_string_with_name<T>(serializable: T) -> Result<String, ApplicationError>
where T: Serialize + Name
{
    let value = to_json_value_with_name(serializable)?;

    to_json_string(value)
}

pub fn to_json_value_with_name<T>(serializable: T) -> Result<Map<String, Value>, ApplicationError>
    where T: Serialize + Name
{
    let mut map = Map::new();

    let value = serde_json::to_value(&serializable)
        .map_err(|e| ApplicationError::UnexpectedError(e.into()))?;

    map.insert(serializable.name().to_string(), value);

    Ok(map)
}