use crate::api::error::ApiError;
use poem::error::ParseJsonError;
use poem::http::header;
use poem::{FromRequest, Request, RequestBody};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct JsonValidation<T: Validate + DeserializeOwned>(pub T);

impl<'a, T> FromRequest<'a> for JsonValidation<T>
where
    T: Validate + DeserializeOwned,
{
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> poem::Result<Self> {
        let content_type = req
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|content_type| content_type.to_str().ok())
            .ok_or(ApiError::Parsing(ParseJsonError::ContentTypeRequired))?;
        if !is_json_content_type(content_type) {
            return Err(
                ApiError::Parsing(ParseJsonError::InvalidContentType(content_type.into())).into(),
            );
        }

        let body: T = serde_json::from_slice(&body.take()?.into_bytes().await?)
            .map_err(|err| ApiError::Parsing(ParseJsonError::Parse(err)))?;

        if let Err(errors) = body.validate() {
            return Err(ApiError::Validation(errors).into());
        }
        Ok(JsonValidation(body))
    }
}

fn is_json_content_type(content_type: &str) -> bool {
    matches!(content_type.parse::<mime::Mime>(), 
        Ok(content_type) if content_type.type_() == "application" 
        && (content_type.subtype() == "json"
        || content_type
            .suffix()
            .map_or(false, |v| v == "json")))
}
