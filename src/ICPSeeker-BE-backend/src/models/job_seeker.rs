use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum JobType {
    FullTime,
    PartTime,
    Contract,
    Internship,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum WorkType {
    Remote,
    Hybrid,
    Onsite,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum EducationLevel {
    HighSchool,
    Diploma,
    Bachelor,
    Master,
    Doctorate,
    Other,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BankAccount {
    pub bank_name: String,
    pub account_number: String,
    pub account_holder_name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Education {
    pub level: EducationLevel,
    pub institution: String,
    pub major: String,
    pub graduation_year: u32,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct JobSeeker {
    pub principal_id: Principal,
    
    // Required Basic Information
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub cv_data: Vec<u8>, 
    
    // Optional Professional Information
    pub linkedin_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub current_job_title: Option<String>,
    pub years_of_experience: Option<u32>,
    pub education: Option<Education>,
    pub skills: Option<Vec<String>>,
    
    // Optional Preferences
    pub expected_salary: Option<f64>,
    pub preferred_location: Option<String>,
    pub job_type: Option<JobType>,
    pub work_type: Option<WorkType>,
    
    // Optional Financial Information
    pub bank_account: Option<BankAccount>,
    
    // Profile Status
    pub is_profile_complete: bool,
    pub created_at: u64,
    pub updated_at: u64,
}


#[derive(CandidType, Deserialize)]
pub struct CreateProfilePayload {
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub cv_data: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateProfilePayload {
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

#[derive(CandidType, Deserialize)]
pub enum ProfileResponse {
    Success(JobSeeker),
    NotFound,
    AlreadyExists,
    ValidationError(String),
    DatabaseError(String),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ProfileStatus {
    pub principal_id: Principal,
    pub is_profile_complete: bool,
    pub created_at: u64,
}