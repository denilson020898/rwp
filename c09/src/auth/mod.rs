pub mod jwt;
use actix_web::dev::ServiceRequest;
mod processes;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => processes::check_password(token),
        Err(message) => Err(message),
    }
}

#[cfg(test)]
mod process_token_test {
    use super::jwt::JwtToken;
    use super::process_token;
    use actix_web::test::TestRequest;

    #[test]
    fn no_token_process_token() {
        let mock_request = TestRequest::with_header("test", "test").to_srv_request();
        match process_token(&mock_request) {
            Err(msg) => assert_eq!("there is no token", msg),
            _ => panic!("No token in request header should fail"),
        }
    }

    #[test]
    fn incorrect_token() {
        let mock_request = TestRequest::with_header("user-token", "test").to_srv_request();
        match process_token(&mock_request) {
            Err(msg) => assert_eq!("Could not decode", msg),
            _ => panic!("Incorrect token should error"),
        }
    }

    #[test]
    fn correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let mock_request = TestRequest::with_header("user-token", encoded_token).to_srv_request();
        match process_token(&mock_request) {
            Ok(token) => assert_eq!("passed", token),
            _ => panic!("encoded token should pass"),
        }
    }
}
