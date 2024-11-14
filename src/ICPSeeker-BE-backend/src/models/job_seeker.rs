use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuthInfo {
    pub principal_id: Principal,
    pub session_created_at: u64,
    pub last_active: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
}


#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct JobSeekerIC {
    pub principal_id: Principal,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub auth_info: AuthInfo,
    pub is_profile_complete: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct JobSeekerPG {
    pub principal_id: Principal,
    pub cv_data: Vec<u8>,
    pub linkedin_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub current_job_title: Option<String>,
    pub years_of_experience: Option<u32>,
    pub education: Option<Education>,
    pub skills: Option<Vec<String>>,
    pub expected_salary: Option<f64>,
    pub preferred_location: Option<String>,
    pub job_type: Option<JobType>,
    pub work_type: Option<WorkType>,
    pub bank_account: Option<BankAccount>,
}


pub fn validate_phone(phone: &str) -> ValidationResult {
    let is_valid = phone.chars().all(|c| c.is_numeric() || c == '+' || c == '-')
        && phone.len() >= 10
        && phone.len() <= 15;

    ValidationResult {
        is_valid,
        error_message: if is_valid {
            None
        } else {
            Some("Invalid phone number format".to_string())
        },
    }
}

pub fn validate_email(email: &str) -> ValidationResult {
    let is_valid = email.contains('@') && email.contains('.');

    ValidationResult {
        is_valid,
        error_message: if is_valid {
            None
        } else {
            Some("Invalid email format".to_string())
        },
    }
}

pub fn validate_url(url: &str) -> ValidationResult {
    let is_valid = url.starts_with("http://") || url.starts_with("https://");

    ValidationResult {
        is_valid,
        error_message: if is_valid {
            None
        } else {
            Some("Invalid URL format".to_string())
        },
    }
}