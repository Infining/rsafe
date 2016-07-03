extern crate rsafe;

use rsafe::{auth};

#[test]
fn auth_register_test() {
    let appdetails = auth::AppDetails {
        name: "appname".to_string(),
        version: "0.0.1".to_string(),
        vendor: "vendorname".to_string(),
        id: "myID".to_string(),
        permissions: vec! []
    };

    // Attempt to register with Launcher
    let safe_register_resp = auth::register(appdetails);

    match safe_register_resp {
        Err(_) => { // something went wrong : launcher is not running , user didn't allow application in launcher ...
            assert!(false);
        }
        Ok(credentials) => {
            // Check with Launcher that we are authorized
            let auth_check = auth::check(&credentials);
            assert_eq!(200, auth_check.unwrap());
        }
    };
}
