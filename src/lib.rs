pub mod api;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub enum Environment {
    UAT,
    PROD,
}

pub enum FlowType {
    Desktop,
    Mobile,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OAuthToken {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: u16,
}

pub struct ProveCredentials {
    data: HashMap<String, String>,
}

pub struct ProveClient {
    creds: ProveCredentials,
    http_client: reqwest::blocking::Client,
    env: Environment,
    oauth_token: Option<OAuthToken>,
    correlation_id: Option<String>,
    auth_token: Option<String>,
    flow_type: FlowType,
    final_target_url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct StartParams {
    //DOB must be YYYY-MM-DD or YYYY-MM or MM-DD
    pub dob: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "finalTargetUrl")]
    pub final_target_url: String,
    #[serde(rename = "flowType")]
    pub flow_type: String,
    //SSN cannot be longer than 9 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StartResponse {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(rename = "authToken")]
    pub auth_token: String,
}

#[derive(Serialize, Debug)]
pub struct ValidateParams {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(rename = "flowType")]
    pub flow_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "finalTargetUrl")]
    pub final_target_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ValidateResponse {
    #[serde(rename = "phoneNumber")]
    #[serde(default)]
    pub phone_number: String,
    pub success: bool,
    #[serde(rename = "challengeMissing")]
    pub challenge_missing: bool,
}

#[derive(Serialize, Debug)]
pub struct SubmitChallengeParams {
    #[serde(rename = "correlationId")]
    correlation_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SubmitChallengeResponse {
    pub individual: Individual,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Individual {
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    pub addresses: Vec<Address>,
    #[serde(rename = "emailAddresses")]
    pub email_addresses: Vec<String>,
    #[serde(default)]
    pub dob: String,
    #[serde(default)]
    ssn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub city: String,
    #[serde(rename = "extendedAddress")]
    #[serde(default)]
    pub extended_address: String,
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub region: String,
}

#[derive(Serialize, Debug)]
pub struct CompleteFlowParams {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub individual: Individual,
}

#[derive(Deserialize, Debug)]
pub struct CompleteFlowResponse {
    pub success: bool,
}
