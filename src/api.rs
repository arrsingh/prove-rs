//use reqwest::blocking::Client;
use crate::CompleteFlowParams;
use crate::CompleteFlowResponse;
use crate::Environment;
use crate::FlowType;
use crate::Individual;
use crate::OAuthToken;
use crate::ProveClient;
use crate::ProveCredentials;
use crate::StartParams;
use crate::StartResponse;
use crate::SubmitChallengeParams;
use crate::SubmitChallengeResponse;
use crate::ValidateParams;
use crate::ValidateResponse;
use reqwest::header::AUTHORIZATION;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
const OAUTH_TOKEN_URL_UAT: &str = "https://platform.uat.proveapis.com/token";
const OAUTH_TOKEN_URL_PROD: &str = "https://platform.proveapis.com/token";
const START_FLOW_URL_UAT: &str = "https://platform.uat.proveapis.com/v3/start";
const START_FLOW_URL_PROD: &str = "https://platform.proveapis.com/v3/start";
const VALIDATE_PHONE_URL_UAT: &str = "https://platform.uat.proveapis.com/v3/validate";
const VALIDATE_PHONE_URL_PROD: &str = "https://platform.proveapis.com/v3/validate";
const SUBMIT_CHALLENGE_URL_UAT: &str = "https://platform.uat.proveapis.com/v3/challenge";
const SUBMIT_CHALLENGE_URL_PROD: &str = "https://platform.proveapis.com/v3/challenge";
const CLIENT_ID: &str = "client_id";
const CLIENT_SECRET: &str = "client_secret";
const GRANT_TYPE: &str = "grant_type";
const GRANT_TYPE_VALUE: &str = "client_credentials";

impl Environment {
    pub fn get_oath_token_url(&self) -> &str {
        match self {
            Environment::UAT => OAUTH_TOKEN_URL_UAT,
            Environment::PROD => OAUTH_TOKEN_URL_PROD,
        }
    }

    pub fn get_start_flow_url(&self) -> &str {
        match self {
            Environment::UAT => START_FLOW_URL_UAT,
            Environment::PROD => START_FLOW_URL_PROD,
        }
    }

    pub fn get_validate_phone_url(&self) -> &str {
        match self {
            Environment::UAT => VALIDATE_PHONE_URL_UAT,
            Environment::PROD => VALIDATE_PHONE_URL_PROD,
        }
    }

    pub fn get_submit_challenge_url(&self) -> &str {
        match self {
            Environment::UAT => SUBMIT_CHALLENGE_URL_UAT,
            Environment::PROD => SUBMIT_CHALLENGE_URL_PROD,
        }
    }
}

impl ProveCredentials {
    pub fn new(client_id: &str, client_secret: &str) -> ProveCredentials {
        let mut data = HashMap::new();
        data.insert(CLIENT_ID.to_string(), client_id.to_string());
        data.insert(CLIENT_SECRET.to_string(), client_secret.to_string());
        data.insert(GRANT_TYPE.to_string(), GRANT_TYPE_VALUE.to_string());

        ProveCredentials { data }
    }

    pub fn form_data(&self) -> &HashMap<String, String> {
        return &self.data;
    }
}

impl OAuthToken {
    pub fn as_auth_header_val(&self) -> String {
        let mut auth_header: String = "Bearer ".to_owned();
        auth_header.push_str(&self.access_token);
        return auth_header;
    }
}

impl ProveClient {
    pub fn new(
        env: Environment,
        creds: ProveCredentials,
        flow_type: FlowType,
        final_target_url: Option<String>,
    ) -> ProveClient {
        ProveClient {
            creds,
            env,
            http_client: reqwest::blocking::Client::new(),
            flow_type,
            final_target_url,
            oauth_token: None,
            auth_token: None,
            correlation_id: None,
        }
    }

    pub fn flow_type(&self) -> String {
        match self.flow_type {
            FlowType::Desktop => return "desktop".to_string(),
            FlowType::Mobile => return "mobile".to_string(),
        }
    }

    pub fn get_final_target_url(&self) -> Option<String> {
        match self.flow_type {
            FlowType::Desktop => return self.final_target_url.clone(),
            FlowType::Mobile => return None,
        }
    }

    pub fn correlation_id(&self) -> &Option<String> {
        return &self.correlation_id;
    }

    pub fn auth_token(&self) -> &Option<String> {
        return &self.auth_token;
    }

    pub fn oauth_token(&self) -> &Option<OAuthToken> {
        return &self.oauth_token;
    }

    pub fn get_auth_header_val(&self) -> String {
        match &self.oauth_token {
            None => {
                panic!("No OAuth Token");
            }
            Some(oat) => {
                return oat.as_auth_header_val();
            }
        }
    }

    pub fn get_oauth_token(&mut self) -> Result<OAuthToken, String> {
        let req = self
            .http_client
            .post(self.env.get_oath_token_url())
            .form(self.creds.form_data())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/x-www-form-url-encoded")
            .build();
        match req {
            Ok(r) => {
                let response;
                match self.http_client.execute(r) {
                    Err(e) => return Err(e.to_string()),
                    Ok(r) => response = r,
                }
                if !response.status().is_success() {
                    return Err(response.text().unwrap());
                }
                match response.json() {
                    Err(e) => return Err(e.to_string()),
                    Ok(json) => {
                        let oauth_token: OAuthToken = json;
                        self.oauth_token = Some(oauth_token.clone());
                        return Ok(oauth_token);
                    }
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    pub fn start_flow(&mut self, req_params: &StartParams) -> Result<StartResponse, String> {
        if self.oauth_token.is_none() {
            return Err(
                "OAuth Token is invalid or missing. Please call get_oauth_token() first"
                    .to_string(),
            );
        }
        let json;
        match serde_json::to_string(req_params) {
            Err(e) => {
                return Err(e.to_string());
            }
            Ok(json_string) => {
                json = json_string;
            }
        }

        let req = self
            .http_client
            .post(self.env.get_start_flow_url())
            .body(json)
            .header(AUTHORIZATION, self.get_auth_header_val())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .build();

        match req {
            Ok(r) => {
                let response;
                match self.http_client.execute(r) {
                    Err(e) => return Err(e.to_string()),
                    Ok(r) => response = r,
                }
                if !response.status().is_success() {
                    return Err(response.text().unwrap());
                }
                match response.json() {
                    Err(e) => return Err(e.to_string()),
                    Ok(json) => {
                        let r_json: StartResponse = json;
                        self.auth_token = Some(r_json.auth_token.clone());
                        self.correlation_id = Some(r_json.correlation_id.clone());
                        return Ok(r_json);
                    }
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    pub fn validate_phone(&mut self) -> Result<ValidateResponse, String> {
        if self.correlation_id.is_none() {
            return Err("correlation_id is invalid or missing. Call start_flow() first to obtain a correllation_id".to_string());
        }

        let req_params = ValidateParams {
            correlation_id: self.correlation_id.clone().unwrap(),
            flow_type: self.flow_type(),
            final_target_url: self.get_final_target_url(),
        };

        return self.exec_http_request::<ValidateParams, ValidateResponse>(&req_params);
    }

    pub fn submit_challenge(&mut self) -> Result<SubmitChallengeResponse, String> {
        if self.correlation_id.is_none() {
            return Err("correlation_id is invalid or missing. Call start_flow() first to obtain a correllation_id".to_string());
        }

        let req_params = SubmitChallengeParams {
            correlation_id: self.correlation_id.clone().unwrap(),
        };

        return self
            .exec_http_request::<SubmitChallengeParams, SubmitChallengeResponse>(&req_params);
    }

    pub fn complete_flow(&self, individual: Individual) -> Result<CompleteFlowResponse, String> {
        if self.correlation_id.is_none() {
            return Err("correlation_id is invalid or missing. Call start_flow() first to obtain a correllation_id".to_string());
        }

        let req_params = CompleteFlowParams {
            correlation_id: self.correlation_id.clone().unwrap(),
            individual,
        };

        return self.exec_http_request::<CompleteFlowParams, CompleteFlowResponse>(&req_params);
    }

    fn exec_http_request<R: Serialize, V: DeserializeOwned>(
        &self,
        req_params: &R,
    ) -> Result<V, String> {
        let json;
        match serde_json::to_string(&req_params) {
            Err(e) => {
                return Err(e.to_string());
            }
            Ok(json_string) => {
                json = json_string;
            }
        }

        let req = self
            .http_client
            .post(self.env.get_validate_phone_url())
            .body(json)
            .header(AUTHORIZATION, self.get_auth_header_val())
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .build();
        match req {
            Ok(r) => {
                let response;
                match self.http_client.execute(r) {
                    Err(e) => return Err(e.to_string()),
                    Ok(r) => response = r,
                }
                if !response.status().is_success() {
                    let err_txt = response.text().unwrap();
                    return Err(err_txt);
                }
                match response.json() {
                    Err(e) => return Err(e.to_string()),
                    Ok(json) => {
                        let r_json: V = json;
                        return Ok(r_json);
                    }
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{Environment, ProveCredentials};

    use super::ProveClient;
    use crate::StartParams;
    //Set the clientId and client secret before running tests.
    //Remember to remove them before committing the code
    const CLIENT_ID: &str = "";
    const CLIENT_SECRET: &str = "";

    #[test]
    fn test_get_oath_token() {
        let creds = ProveCredentials::new(CLIENT_ID, CLIENT_SECRET);
        let mut pc = ProveClient::new(
            Environment::UAT,
            creds,
            crate::FlowType::Desktop,
            Some("http://example.com/finalTargetUrl".to_string()),
        );
        let response = pc.get_oauth_token().unwrap();
        assert!(!response.access_token.is_empty());
        assert!(!response.token_type.is_empty());
        assert_eq!(response.token_type, "Bearer");
        assert!(pc.oauth_token.is_some());
        let oauth_token = pc.oauth_token.unwrap();
        assert_eq!(oauth_token.access_token, response.access_token);
        assert_eq!(oauth_token.token_type, response.token_type);
    }

    /*
    Test User:
    Phone: 2001004000
    Name: Martina	Goodram
    Address: 28965 Homewood Plaza Little Rock AR	72204
    DOB: 7/26/1995
    SSN: 490959347
    Email: mgoodram0@nasa.gov
    **/
    fn get_test_user() -> StartParams {
        return StartParams {
            dob: "1995-07-26".to_string(),
            email_address: "mgoodram0@nasa.gov".to_string(),
            final_target_url: "http://example.com/finalTargetUrl".to_string(),
            flow_type: "desktop".to_string(),
            ssn: Some("490959347".to_string()),
            phone_number: Some("2001004000".to_string()),
            ip_address: "1.2.3.4".to_string(),
        };
    }

    #[test]
    fn test_start() {
        let creds = ProveCredentials::new(CLIENT_ID, CLIENT_SECRET);
        let mut pc = ProveClient::new(
            Environment::UAT,
            creds,
            crate::FlowType::Desktop,
            Some("http://example.com/finalTargetUrl".to_string()),
        );
        pc.get_oauth_token().unwrap();
        let start_params = get_test_user();
        let response = pc.start_flow(&start_params).unwrap();
        assert!(!response.correlation_id.is_empty());
        assert!(!response.auth_token.is_empty());
        assert!(pc.correlation_id.is_some());
        let cid = pc.correlation_id.unwrap();
        assert_eq!(response.correlation_id, cid);
    }

    #[test]
    fn test_validate() {
        let creds = ProveCredentials::new(CLIENT_ID, CLIENT_SECRET);
        let mut pc = ProveClient::new(
            Environment::UAT,
            creds,
            crate::FlowType::Desktop,
            Some("http://example.com/finalTargetUrl".to_string()),
        );
        pc.get_oauth_token().unwrap();
        let start_params = get_test_user();
        pc.start_flow(&start_params).unwrap();
        let response = pc.validate_phone().unwrap();
        assert_eq!(response.success, true);
        assert_eq!(response.phone_number, "2001004000");
    }
}
