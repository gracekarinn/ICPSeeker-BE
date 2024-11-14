mod models;
mod services;

use candid::Principal;
use ic_cdk_macros::*;
use models::job_seeker::{CreateProfilePayload, UpdateProfilePayload, ProfileResponse};
use services::job_seeker_service;

#[init]
fn init() {
    ic_cdk::println!("Initializing canister");
}

#[update]
async fn create_profile(payload: CreateProfilePayload) -> ProfileResponse {
    job_seeker_service::create_profile(payload).await
}

#[query]
fn get_profile() -> ProfileResponse {
    let caller = ic_cdk::caller();
    job_seeker_service::get_profile(caller)
}

#[update]
async fn update_profile(payload: UpdateProfilePayload) -> ProfileResponse {
    let caller = ic_cdk::caller();
    job_seeker_service::update_profile(caller, payload).await
}