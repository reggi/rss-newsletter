use validator::validate_email;

pub fn clean_email(email: &str) -> Result<String, String> {
    let e = email.trim().to_lowercase();
    if validate_email(&e) {
        Ok(e)
    } else {
        Err("Invalid email address".to_string())
    }
}