/// Sign Up Request Details 

#[derive(Debug)]
pub struct SignupRequest{
    pub email: String,
    pub password: String
}

pub enum Role {
    Admin, 
    User
} 
