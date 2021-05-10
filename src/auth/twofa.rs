use crate::db::{models::User, update_user};
use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};

pub fn check_2fa(secret: &str, code: &str) -> bool {
    let auth = GoogleAuthenticator::new();
    auth.verify_code(secret, &code, 30, 0)
}

pub fn setup_2fa(u: &mut User) -> String {
    let auth = GoogleAuthenticator::new();
    let secret = auth.create_secret(32);

    let qr_url = auth.qr_code_url(
        secret.as_str(),
        "Lab02 Authentication",
        &u.email,
        400,
        400,
        ErrorCorrectionLevel::High,
    );

    u.secret_2fa = Some(secret);
    update_user(u);

    qr_url
}
