use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MerchantRole {
    Owner,
    StoreManager,
    SupportAgent,
    LogisticsCoordinator,
    Auditor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    ManageBilling,
    ConfigureMarginFloor,
    OverrideFraudBlock,
    DispatchShipments,
    ViewAnalytics,
    RespondToChats,
    ManageTeam,
}

/// Evaluates whether a given `MerchantRole` is authorized for a specific `Permission`.
pub fn role_has_permission(role: MerchantRole, perm: Permission) -> bool {
    match role {
        MerchantRole::Owner => true,
        MerchantRole::StoreManager => !matches!(perm, Permission::ManageBilling | Permission::ManageTeam),
        MerchantRole::SupportAgent => {
            matches!(perm, Permission::RespondToChats | Permission::ViewAnalytics)
        }
        MerchantRole::LogisticsCoordinator => {
            matches!(perm, Permission::DispatchShipments | Permission::ViewAnalytics)
        }
        MerchantRole::Auditor => matches!(perm, Permission::ViewAnalytics),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberRecord {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: MerchantRole,
    pub two_factor_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoAuthSession {
    pub provider: String,
    pub state_token: String,
    pub redirect_url: String,
}
