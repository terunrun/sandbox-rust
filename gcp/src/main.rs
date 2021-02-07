extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_bigquery2 as bigquery2;
use bigquery2::Table;
use bigquery2::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use bigquery2::Bigquery;
use hyper::net::HttpsConnector;
use oauth2::GetToken;
use std::env;

const PROJECT_ID: &str = "sandbox-terunrun-dev";
const DATASET_ID: &str = "import";
const TABLE_ID: &str = "test_table";

fn access_bigquery() {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
    // `client_secret`, among other things.
    let secret: ApplicationSecret = Default::default();
    // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                                <MemoryStorage as Default>::default(), None);
    let mut hub = Bigquery::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let mut req = Table::default();
    
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.tables().update(req, PROJECT_ID, DATASET_ID, TABLE_ID)
                .doit();
    
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}

fn main() {
    let json =  env::var("CREDENTIAL_JSON").expect("CREDENTIAL_JSON is not defined");

    // let client_secret = oauth2::service_account_key_from_file(&"/Users/terunrun/Downloads/sandbox-terunrun-dev-d8bcb6cc65fb.json".to_string()).unwrap();
    let client_secret = oauth2::service_account_key_from_file(&json.to_string()).unwrap();
    let client = hyper::Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));

    let mut access = oauth2::ServiceAccountAccess::new(client_secret, client);

    // println!("{:?}",
    //          access.token(&vec!["https://www.googleapis.com/auth/bigquery"]).unwrap());

    let client = hyper::Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
    let hub = bigquery2::Bigquery::new(client, access);

    // let result = hub.tables().list(PROJECT_ID, DATASET_ID).doit();
    // match result {
    //     Err(e) => match e {
    //         // The Error enum provides details about what exactly happened.
    //         // You can also just use its `Debug`, `Display` or `Error` traits
    //         Error::HttpError(_)
    //         |Error::MissingAPIKey
    //         |Error::MissingToken(_)
    //         |Error::Cancelled
    //         |Error::UploadSizeLimitExceeded(_, _)
    //         |Error::Failure(_)
    //         |Error::BadRequest(_)
    //         |Error::FieldClash(_)
    //         |Error::JsonDecodeError(_, _) => println!("{}", e),
    //     },
    //     Ok(res) => println!("Success: {:?}", res.1),
    // }

    let result = hub.tables().get(PROJECT_ID, DATASET_ID, TABLE_ID,).doit();
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res.1),
    }

}