use mockito::{Mock, Server};

use super::api::LoginResponse;

#[cfg(test)]
pub(crate) trait TokenApiMocks {
    fn mock_login_success(&mut self) -> Mock;
    fn mock_login_failure(&mut self) -> Mock;
}

#[cfg(test)]
impl TokenApiMocks for Server {
    fn mock_login_success(&mut self) -> Mock {
        let r = LoginResponse { access: "a".to_string(), refresh: "b".to_string() };
        self.mock("POST", "/jwt/create")
            .with_status(200)
            .with_body(
                serde_json::to_string(&r).unwrap()
            )
            .create()
    }

    fn mock_login_failure(&mut self) -> Mock {
        self.mock("POST", "/jwt/create")
            .with_status(403)
            .create()
    }
}
