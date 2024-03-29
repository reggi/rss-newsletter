use validator::ValidateEmail;

pub fn clean_email(email: &str) -> Result<String, String> {
    let e = email.trim().to_lowercase();
    if ValidateEmail::validate_email(&e) {
        Ok(e)
    } else {
        Err("Invalid email address".to_string())
    }
}
