use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignupResponse {
    user_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateProfileRequest {
    pub profile_id: i32,
    pub display_name: String,
    pub bio: String,
    pub zodiac: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiscoveryResponse {
    pub discovery_id: i32,
    pub display_name: String,
    pub bio: String,
    pub zodiac: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LikeRequest {
    pub liker_id: i32,
    pub liked_id: i32,
    pub status: bool,
}
