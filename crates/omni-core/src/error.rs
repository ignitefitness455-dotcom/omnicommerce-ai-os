use serde::{Deserialize, Serialize};
use thiserror::Error;

/// RFC 9457 / RFC 7807 Problem Details envelope for consistent REST API error responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub error_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("High fraud risk blocked: {0}")]
    FraudBlocked(String),

    #[error("External integration error ({provider}): {message}")]
    ExternalProvider { provider: String, message: String },

    #[error("Internal system error: {0}")]
    Internal(String),
}

impl DomainError {
    pub fn to_problem_details(&self, instance: Option<String>) -> ProblemDetails {
        match self {
            DomainError::Validation(detail) => ProblemDetails {
                error_type: "https://api.omnicommerce.ai/errors/validation-failed".to_string(),
                title: "Request Validation Failed".to_string(),
                status: 422,
                detail: detail.clone(),
                instance,
                code: Some("VALIDATION_FAILED".to_string()),
            },
            DomainError::Unauthorized(detail) => ProblemDetails {
                error_type: "https://api.omnicommerce.ai/errors/unauthorized".to_string(),
                title: "Authentication or Signature Verification Failed".to_string(),
                status: 401,
                detail: detail.clone(),
                instance,
                code: Some("UNAUTHORIZED".to_string()),
            },
            DomainError::NotFound(detail) => ProblemDetails {
                error_type: "https://api.omnicommerce.ai/errors/not-found".to_string(),
                title: "Resource Not Found".to_string(),
                status: 404,
                detail: detail.clone(),
                instance,
                code: Some("NOT_FOUND".to_string()),
            },
            DomainError::FraudBlocked(detail) => ProblemDetails {
                error_type: "https://api.omnicommerce.ai/errors/fraud-shield-blocked".to_string(),
                title: "Blocked by OmniCommerce Fraud Shield".to_string(),
                status: 403,
                detail: detail.clone(),
                instance,
                code: Some("FRAUD_SHIELD_BLOCKED".to_string()),
            },
            DomainError::ExternalProvider { provider, message } => ProblemDetails {
                error_type: "https://api.omnicommerce.ai/errors/upstream-provider".to_string(),
                title: format!("Upstream Provider Error ({})", provider),
                status: 502,
                detail: message.clone(),
                instance,
                code: Some("UPSTREAM_ERROR".to_string()),
            },
            DomainError::Internal(detail) => ProblemDetails {
                error_type: "https://api.omnicommerce.ai/errors/internal".to_string(),
                title: "Internal Server Error".to_string(),
                status: 500,
                detail: detail.clone(),
                instance,
                code: Some("INTERNAL_ERROR".to_string()),
            },
        }
    }
}
