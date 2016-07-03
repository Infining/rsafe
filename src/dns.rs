use std::collections::HashMap;


#[derive(Debug)]
pub enum ConnectionError { UnableToConnect , Unauthorized , FieldsAreMissing, BadRequest, UnknownError, InternalServerError, NotFound }

#[derive(Debug, RustcEncodable)]
pub struct RegisterServiceData {
    pub long_name: String,
    pub service_name: String,
    pub service_home_dir_path: String,
    pub is_path_shared: bool
}

//Register a long name
pub fn register_long_name ( long_name : String , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

	println!("App: Begin Registering Long Name ...");

	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;

	let url_dns = "http://localhost:8100/0.5/dns".to_string();

	let long_nameencoded = ::url::percent_encoding::utf8_percent_encode ( &long_name, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );

	let url_dns_ln = url_dns + "/" + &long_nameencoded ;
	println!("url_dns_ln = {:?}", &url_dns_ln);

	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Connection".to_string(), "close".to_string());

	let body = String::new();

	//println!("sending request");
	//Send a request to launcher using "request" library
	let res = ::request::post(&url_dns_ln, &mut headers, &body.into_bytes() );
	//println!("request sent");

	//Error handling
	match res {
		Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
		Ok(res) =>
		{
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad request"); return Err(ConnectionError::BadRequest)
			}  else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }

		} // end Ok
	}; // end match

} // end fn

//Register a service
pub fn register_service ( register_service_data : RegisterServiceData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

	let token = &safe_register_resp.token ;

	let bearertoken = "bearer ".to_string()+&token ;

	println!("app: begin registering service...");

	// Encode the request as a JSON.
	let register_service_json_str = ::rustc_serialize::json::encode(&register_service_data).unwrap_or_else(|a| panic!("{:?}", a));
	//println!("App: RegisterService encoded");

	let url_dns = "http://localhost:8100/5.0/dns/";

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::post(&url_dns, &mut headers, &register_service_json_str.into_bytes() );
		//println!("request sent");

		//Error handling
    match res {
       Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
       Ok(res) => {
          if res.status_code == 200 {
              println!("200 Ok"); { return Ok(res.status_code) }
          } else {
              // Get it into a valid UTF-8 String -
              //let decrypted_response_str = String::from_utf8(res.body).ok().unwrap();
              println!( "decr = {}" , &res.body );

              if res.status_code == 401 {
                  println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
              } else if res.status_code == 400 {
                  println!("400 Bad Request");
                  return Err(ConnectionError::BadRequest)
              } else {
                  return Err(ConnectionError::UnknownError)
              }
          }
      } // end ok
    };  // end match
} // end fn
