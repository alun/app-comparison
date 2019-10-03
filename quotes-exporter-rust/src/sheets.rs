extern crate google_sheets4 as sheets4;
extern crate yup_oauth2 as oauth2;

use oauth2::{ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage};
use sheets4::Sheets;
use sheets4::ValueRange;
use std::fs;

#[derive(Debug)]
pub struct Doc {
    pub id: String,
    pub sheet: String,
}

impl Default for Doc {
    fn default() -> Doc {
        Doc {
            id: "".to_string(),
            sheet: "Sheet1".to_string(),
        }
    }
}

impl From<&str> for Doc {
    fn from(s: &str) -> Doc {
        let parts: Vec<&str> = s.split(":").collect();
        Doc {
            id: parts[0].to_string(),
            sheet: parts[1].to_string(),
        }
    }
}

impl Doc {
    pub fn write(&self, range: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let secret = get_secret();
        let range = &format!("{}!{}", self.sheet, range);
        let auth = Authenticator::new(
            &secret,
            DefaultAuthenticatorDelegate,
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            DiskTokenStorage::new(&"./auth/token.json".to_string())
                .or(Err("Token storage not found"))?,
            None,
        );
        let hub = Sheets::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            auth,
        );
        let req = ValueRange {
            range: Some(range.to_string()),
            values: Some(vec![vec![value.to_string()]]),
            ..ValueRange::default()
        };
        hub.spreadsheets()
            .values_update(req, &self.id, range)
            .add_scope(sheets4::Scope::Spreadsheet)
            .value_input_option("USER_ENTERED")
            .doit()?;

        Ok(())
    }
}

fn get_secret() -> ApplicationSecret {
    let client_secret = fs::read_to_string("./auth/token.txt").or(Err("No secret provided")).unwrap();
    let client_secret = client_secret.trim().to_string();
    let app_secret = ApplicationSecret {
        client_id: "477576497251-epv7v4f6rqjh2i9q46q37mgbbpg7drql.apps.googleusercontent.com"
            .to_string(),
        auth_uri: String::from("https://accounts.google.com/o/oauth2/auth"),
        auth_provider_x509_cert_url: Some(String::from(
            "https://www.googleapis.com/oauth2/v1/certs",
        )),
        project_id: Some("crypto-bay".to_string()),
        client_secret: client_secret,
        client_email: None,
        client_x509_cert_url: None,
        redirect_uris: vec![
            String::from("urn:ietf:wg:oauth:2.0:oob"),
            String::from("http://localhost"),
        ],
        token_uri: String::from("https://oauth2.googleapis.com/token"),
    };
    app_secret
}
