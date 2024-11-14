use crate::models::job_seeker::{JobSeeker, CreateProfilePayload, UpdateProfilePayload, ProfileResponse};
use ic_cdk::storage;
use std::cell::RefCell;
use std::collections::HashMap;
use candid::Principal;

thread_local! {
    static JOB_SEEKERS: RefCell<HashMap<Principal, JobSeeker>> = RefCell::new(HashMap::new());
}

pub async fn create_profile(payload: CreateProfilePayload) -> ProfileResponse {
    let caller = ic_cdk::caller();
    
    if !payload.email.contains('@') {
        return ProfileResponse::ValidationError("Invalid email format".to_string());
    }

    JOB_SEEKERS.with(|job_seekers| {
        if job_seekers.borrow().contains_key(&caller) {
            return ProfileResponse::AlreadyExists;
        }

        let now = ic_cdk::api::time();
        let job_seeker = JobSeeker {
            principal_id: caller,
            name: payload.name,
            email: payload.email,
            phone_number: payload.phone_number,
            cv_data: payload.cv_data,
            linkedin_url: None,
            portfolio_url: None,
            current_job_title: None,
            years_of_experience: None,
            education: None,
            skills: None,
            expected_salary: None,
            preferred_location: None,
            job_type: None,
            work_type: None,
            bank_account: None,
            is_profile_complete: true,
            created_at: now,
            updated_at: now,
        };

        job_seekers.borrow_mut().insert(caller, job_seeker.clone());
        ProfileResponse::Success(job_seeker)
    })
}

pub fn get_profile(principal: Principal) -> ProfileResponse {
    JOB_SEEKERS.with(|job_seekers| {
        match job_seekers.borrow().get(&principal) {
            Some(job_seeker) => ProfileResponse::Success(job_seeker.clone()),
            None => ProfileResponse::NotFound,
        }
    })
}

pub async fn update_profile(principal: Principal, payload: UpdateProfilePayload) -> ProfileResponse {
    JOB_SEEKERS.with(|job_seekers| {
        let mut job_seekers = job_seekers.borrow_mut();
        
        match job_seekers.get_mut(&principal) {
            Some(job_seeker) => {
                job_seeker.linkedin_url = payload.linkedin_url;
                job_seeker.portfolio_url = payload.portfolio_url;
                job_seeker.current_job_title = payload.current_job_title;
                job_seeker.years_of_experience = payload.years_of_experience;
                job_seeker.education = payload.education;
                job_seeker.skills = payload.skills;
                job_seeker.expected_salary = payload.expected_salary;
                job_seeker.preferred_location = payload.preferred_location;
                job_seeker.job_type = payload.job_type;
                job_seeker.work_type = payload.work_type;
                job_seeker.bank_account = payload.bank_account;
                job_seeker.updated_at = ic_cdk::api::time();

                ProfileResponse::Success(job_seeker.clone())
            },
            None => ProfileResponse::NotFound,
        }
    })
}