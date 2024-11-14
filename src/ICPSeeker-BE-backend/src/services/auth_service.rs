use candid::Principal;
use ic_cdk::api::time;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::models::job_seeker::{AuthInfo, ValidationResult};

thread_local! {
    static AUTH_SESSIONS: RefCell<HashMap<Principal, AuthInfo>> = RefCell::new(HashMap::new());
}

pub fn create_session(principal: Principal) -> AuthInfo {
    let now = time();
    let auth_info = AuthInfo {
        principal_id: principal,
        session_created_at: now,
        last_active: now,
    };

    AUTH_SESSIONS.with(|sessions| {
        sessions.borrow_mut().insert(principal, auth_info.clone());
    });

    auth_info
}

pub fn verify_session(principal: Principal) -> bool {
    AUTH_SESSIONS.with(|sessions| {
        if let Some(session) = sessions.borrow().get(&principal) {
            let now = time();
            now - session.session_created_at < 24 * 60 * 60 * 1_000_000_000
        } else {
            false
        }
    })
}

pub fn update_last_active(principal: Principal) {
    AUTH_SESSIONS.with(|sessions| {
        if let Some(session) = sessions.borrow_mut().get_mut(&principal) {
            session.last_active = time();
        }
    });
}