use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub org_id: Option<i64>,
    pub tenant_org_id: Option<i64>,
    pub appid: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let org_id = parts
            .headers
            .get("x-org-id")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<i64>().ok());
        let tenant_org_id = parts
            .headers
            .get("x-tenant-org-id")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<i64>().ok());
        let appid = parts
            .headers
            .get("x-appid")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());

        Ok(Self {
            org_id,
            tenant_org_id,
            appid,
        })
    }
}

