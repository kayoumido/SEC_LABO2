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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_code() {
        let secret = "I3VFM3JKMNDJCDH5BMBEEQAW6KJ6NOE3";
        let auth = GoogleAuthenticator::new();
        let code = auth.get_code(secret, 0).unwrap();

        assert_eq!(check_code(secret, &code), true);
        assert_eq!(check_code(secret, "000000"), false);
    }

    #[test]
    fn test_generate_secret() {
        let secret = generate_secret();

        assert_eq!(secret.len(), 32);
    }

    #[test]
    fn test_generate_qr() {
        let secret = "I3VFM3JKMNDJCDH5BMBEEQAW6KJ6NOE3";
        let name = "test";
        let title = "test";
        let qr_url = generate_qr(secret, name, title);

        assert_eq!(qr_url.contains(secret), true);
        assert_eq!(qr_url.contains(name), true);
        assert_eq!(qr_url.contains(title), true);
        assert_eq!(qr_url.contains("http"), true);
    }
}
