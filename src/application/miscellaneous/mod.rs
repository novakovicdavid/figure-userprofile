use serde::Serialize;

use crate::application::ApplicationError;

pub trait ToJsonString {
    fn to_json_string(&self) -> Result<String, ApplicationError>;
}

impl<T> ToJsonString for T
where
    T: Serialize,
{
    fn to_json_string(&self) -> Result<String, ApplicationError> {
        serde_json::to_string(&self)
            .map_err(ApplicationError::from)
    }
}