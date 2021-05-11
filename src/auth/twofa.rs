use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};

/// Checks that a 2fa code entered by a user is valid
///
/// # Arguments
///
/// * `secret` - the secret under which the code was genereated
///
/// * `code` - the code to check
///
pub fn check_code(secret: &str, code: &str) -> bool {
    let auth = GoogleAuthenticator::new();
    auth.verify_code(secret, &code, 30, 0)
}

/// Generates a secret for the 2fa
///
pub fn generate_secret() -> String {
    let auth = GoogleAuthenticator::new();
    auth.create_secret(32)
}

/// Generates the url of QR code for a given secret
/// With the qr code, the user will be able to add to an authenticator app
/// e.g. Google Authenticator
///
/// # Arguments
///
/// * `secret` - the secret to generate the QR from
///
/// * `name` - the name of the application
///
/// * `title` - the name to set
///
pub fn generate_qr(secret: &str, name: &str, title: &str) -> String {
    let auth = GoogleAuthenticator::new();
    auth.qr_code_url(secret, name, title, 400, 400, ErrorCorrectionLevel::High)
}
