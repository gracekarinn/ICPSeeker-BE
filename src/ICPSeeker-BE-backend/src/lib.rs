mod models;
mod services;

use candid::Principal;
use ic_cdk_macros::*;
use models::job_seeker::{CreateProfilePayload, UpdateProfilePayload, ProfileResponse, AuthResponse};
use services::{auth_service, job_seeker_service};

#[init]
fn init() {
    ic_cdk::println!("Initializing canister");
}

#[update]
fn init_auth() -> AuthResponse {
    auth_service::init_auth()
}

#[query]
fn verify_auth() -> bool {
    let caller = ic_cdk::caller();
    auth_service::verify_session(caller)
}

#[update]
fn logout() -> bool {
    let caller = ic_cdk::caller();
    auth_service::logout(caller)
}

#[update]
async fn create_profile(payload: CreateProfilePayload) -> ProfileResponse {
    let caller = ic_cdk::caller();
    
    if !auth_service::verify_session(caller) {
        return ProfileResponse::AuthenticationError("Not authenticated".to_string());
    }
    
    job_seeker_service::create_profile(payload).await
}

#[query]
fn get_profile() -> ProfileResponse {
    let caller = ic_cdk::caller();
    
    if !auth_service::verify_session(caller) {
        return ProfileResponse::AuthenticationError("Not authenticated".to_string());
    }
    
    job_seeker_service::get_profile(caller)
}

#[update]
async fn update_profile(payload: UpdateProfilePayload) -> ProfileResponse {
    let caller = ic_cdk::caller();
    
    if !auth_service::verify_session(caller) {
        return ProfileResponse::AuthenticationError("Not authenticated".to_string());
    }
    
    job_seeker_service::update_profile(caller, payload).await
}