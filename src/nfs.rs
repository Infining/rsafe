use std::collections::HashMap;

#[derive(Debug, RustcEncodable)]
pub struct CreateDirData {
	pub dir_path: String,
	pub is_private: bool,
	pub metadata: String,
	pub is_versioned: bool,
	pub is_path_shared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct ReadDirData {
	pub dir_path: String,
	pub is_path_shared: bool,
}

#[derive(Debug, RustcDecodable)]
struct GetDirResponse {
	id: String,
	data: GetDirResponseData,
}

#[derive(Debug, RustcDecodable)]
pub struct GetDirResponseData {
	pub info: DirInfo,
	pub files: Vec<FileInfo>,
	pub sub_directories: Vec<DirInfo>,
}

#[derive(Debug, RustcDecodable)]
pub struct DirInfo {
	pub name: String,
	pub created_on: i64,
	pub modified_on: i64,
	pub is_private: bool,
	pub is_versioned: bool,
	pub metadata: String,
}

#[derive(Debug, RustcDecodable)]
pub struct FileInfo {
	pub name: String,
	pub created_on: i64,
	pub modified_on: i64,
	pub metadata: String,

}

#[derive(Debug, RustcEncodable)]
pub struct CreateFileData {
	pub file_path: String,
	pub is_private: bool,
	pub metadata: String,
	pub is_versioned: bool,
	pub is_path_shared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct MoveFileData {
	pub src_path: String,
	pub dest_path: String,
	pub retain_source: bool,
	pub is_src_path_shared: bool,
	pub is_dest_path_shared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct MoveDirData {
	pub src_path: String,
	pub dest_path: String,
	pub retain_source: bool,
	pub is_src_path_shared: bool,
	pub is_dest_path_shared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct WriteFileData {
	pub file_path: String,
	pub is_path_shared: bool,
	pub file_content: String,
	pub offset: i64,
}

#[derive(Debug, RustcEncodable)]
pub struct ReadFileData {
	pub file_path: String,
	pub is_path_shared: bool,
	pub offset: i64,
	pub length: i64
}

#[derive(Debug, RustcEncodable)]
pub struct DeleteFileData {
	pub file_path: String,
	pub is_path_shared: bool,
}

// fn get_base64_config() -> ::rustc_serialize::base64::Config {
// 	::rustc_serialize::base64::Config {
// 		char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
// 		newline    : ::rustc_serialize::base64::Newline::LF,
// 		pad        : true,
// 		line_length: None,
// 	}
// }

#[derive(Debug, RustcDecodable)]
pub struct FileReadInfo {
	pub filename: String,
	pub filesize: i64,
	pub filecreatedtime: i64,
	pub filemodifiedtime: i64,
	pub filemetadata: String,
	pub filebody: String,
}

#[derive(Debug)]
pub enum ConnectionError { UnableToConnect , Unauthorized , FieldsAreMissing, BadRequest, UnknownError, InternalServerError, NotFound }

/* TODO
 *
 * 	 read and write file with offset
 *
 * 	 modify file info
 *
 *   modify dir info
 *
 *   move dir test    ----- 400
 *
 */

// create a directory
pub fn create_dir ( create_dir_data : CreateDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

		let token = &safe_register_resp.token ;

		let bearertoken = "Bearer ".to_string()+&token ;

		println!("App: Begin creating directory...");

		// Encode the request as a JSON.
		let create_dir_json_str = ::rustc_serialize::json::encode(&create_dir_data).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: CreateDir encoded");

		// Get raw bytes to be encrypted.
		let create_dir_bytes = create_dir_json_str.into_bytes();

		let url_nfs = "http://localhost:8100/5.0/nfs/directory";

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::post(&url_nfs, &mut headers, &create_dir_bytes );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// move a directory
pub fn move_dir( move_dir_data : MoveDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

		let token = &safe_register_resp.token ;

		let bearertoken = "Bearer ".to_string()+&token ;

		println!("App: Begin Moving Dir...");

		// Encode the request as a JSON.
		let move_dir_json_str = ::rustc_serialize::json::encode(&move_dir_data).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: MoveDir encoded");

		// Get raw bytes to be encrypted.
		let move_dir_bytes = move_dir_json_str.into_bytes();

		let url_nfs_dir = "http://localhost:8100/nfs/movedir".to_string();

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::post(&url_nfs_dir, &mut headers, &move_dir_bytes );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// read a directory
pub fn read_dir ( read_dir_data : ReadDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< GetDirResponseData, ConnectionError > {

		println!("App: Begin reading directory...");

		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;

		// path Parameters
		let requested_dir = read_dir_data.dir_path ;
		let dir_path = ::url::percent_encoding::utf8_percent_encode ( &requested_dir, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = read_dir_data.is_path_shared;

		//println!("dirPath = {}",&dir_path);

		// URL to send our 'ls' request to

		let url_nfs = "http://localhost:8100/5.0/nfs/directory".to_string();
		let url_nfs_ls = url_nfs + "/" + &dir_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_ls = {}",&url_nfs_ls);

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::get(&url_nfs_ls, &mut headers );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); {

		//println!("App: GetDir Response JSON: {:?}", decrypted_response_json_str);

		// Decode the JSON into expected response structure - in this case a directory response as
		// stated in the RFC.
		let get_dir_response: GetDirResponseData = ::rustc_serialize::json::decode(&res.body)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		//println!("App: GetDir Response decoded.");

		return Ok(get_dir_response) }

		} else { return Err(ConnectionError::UnknownError) }
	}

};	//match end
}	//fn end

// delete a directory
pub fn delete_dir ( delete_dir_data : ReadDirData, safe_register_resp : &super::auth::SafeRegisterResp  ) -> Result< u16 , ConnectionError > {

		println!("App: Begin deleting directory...");

		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;

		// path Parameters
		let requested_dir = delete_dir_data.dir_path ;
		let dir_path = ::url::percent_encoding::utf8_percent_encode ( &requested_dir, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = delete_dir_data.is_path_shared;

		println!("dirPath = {}",&dir_path);

		// URL to send our 'ls' request to
		let url_nfs = "http://localhost:8100/5.0/nfs/directory".to_string();
		let url_nfs_del = url_nfs + "/" + &dir_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_ls = {}",&url_nfs_del);

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::delete(&url_nfs_del, &mut headers );

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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok Directory was deleted");		{ return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};

} // fn end

// create an empty file
pub fn create_file( create_file_data : CreateFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

		let token = &safe_register_resp.token ;

		let bearertoken = "Bearer ".to_string()+&token ;

		println!("App: Begin creating file...");

		// Encode the request as a JSON.
		let create_file_json_str = ::rustc_serialize::json::encode(&create_file_data).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: CreateFile encoded");

		// Get raw bytes to be encrypted.
		let create_file_bytes = create_file_json_str.into_bytes();

		//println!( "encr = {}", &create_file_json_encrypted_b64 );

		let url_nfs_file = "http://localhost:8100/5.0/nfs/file".to_string();

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::post(&url_nfs_file, &mut headers, &create_file_bytes );

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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// move a file
pub fn move_file( move_file_data : MoveFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

		let token = &safe_register_resp.token;

		let bearertoken = "Bearer ".to_string()+&token ;

		println!("App: Begin moving file...");

		// Encode the request as a JSON.
		let move_file_json_str = ::rustc_serialize::json::encode(&move_file_data).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: MoveFile encoded");

		// Get raw bytes to be encrypted.
		let move_file_bytes = move_file_json_str.into_bytes();

		//println!( "encr = {}", &move_file_json_encrypted_b64 );

		let url_nfs_file = "http://localhost:8100/5.0/nfs/movefile".to_string();

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::post(&url_nfs_file, &mut headers, &move_file_bytes );

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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// write to a file
pub fn write_file ( write_file_data : WriteFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

		let token = &safe_register_resp.token;

		let bearertoken = "Bearer ".to_string()+&token;

		let file_content = write_file_data.file_content;

		println!("App: Begin writing to file...");

		// Encode the request as a JSON.
		let write_file_json_str = ::rustc_serialize::json::encode(&file_content).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: WriteFile encoded");

		// Get raw bytes to be encrypted.
		let write_file_bytes = write_file_json_str.into_bytes();

		// path Parameters
		let requested_file = write_file_data.file_path ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = write_file_data.is_path_shared;
		let offset = write_file_data.offset ; // seems to be unsupported

		//println!("dirPath = {}",&dir_path);

		// URL to send our 'ls' request to

		let url_nfs = "http://localhost:8100/5.0/nfs/directory".to_string();
		let url_nfs_write = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string() + "?offset:=" + &offset.to_string() ;
		//println!("url_nfs_ls = {}",&url_nfs_write);

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::put(&url_nfs_write, &mut headers, &write_file_bytes );

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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// read a file
pub fn read_file ( read_file_data : ReadFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< FileReadInfo , ConnectionError > {

		println!("App: Begin reading file...");

		/*
		 *
		 *
		 *    TODO   PANIC on inexistant file
		 *
		 *
		 */

		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;

		// path Parameters
		let requested_file = read_file_data.file_path ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = read_file_data.is_path_shared;

		let offset = read_file_data.offset ; //  seems to be unsupported
		let length = read_file_data.length ; //  seems to be unsupported

		// URL to send our 'ls' request to
		// http://localhost:8100/0.4/nfs/file/:filePath/:isPathShared?offset=:offset&length=:length

		let url_nfs = "http://localhost:8100/5.0/nfs/directory".to_string();

		let url_nfs1 = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string() ;

		let mut url_nfs_read = url_nfs1.clone() ;

		// append length and offset if needed
		if  length > 0 && offset > 0 {
		 url_nfs_read = url_nfs1 +  "?offset=:" + &&offset.to_string() + "&length=:" + &&length.to_string() ; }
		else if  length == 0 && offset > 0  {
		 url_nfs_read = url_nfs1 +  "?offset=:" + &&offset.to_string(); }
		else if  length > 0 && offset == 0  {
		 url_nfs_read = url_nfs1 +  "?length=:" + &&length.to_string() ; };

		println!("url_nfs_read = {}",&url_nfs_read);

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::get( &url_nfs_read, &mut headers );

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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); {

		// Decode the JSON into expected response structure

		/*
		 *
		 * 		TODO
		 *
		 *     panics on empty file  ----  EOF
		 *
		 */

		let read_file_resp_body = ::rustc_serialize::json::decode(&res.body)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		//println!("App: GetFile Response decoded.");

		//get headers
		let headers = res.headers;

		//println!( "get file headers = {:?}", headers);

		let mut file_size = "";
		let mut file_name = "";
		let mut file_created_time = "";
		let mut file_modified_time = "";
		let mut file_metadata = "";


		match headers.get("file-size") {
			Some ( val ) => { file_size = val; },
			_ => { file_size = "0"; }
		}

		match headers.get("file-name") {
			Some ( val ) => { file_name = val; },
			_ => { file_name = "None"; }
		}

		match headers.get("file-created-time") {
			Some ( val ) => { file_created_time = val; },
			_ => { file_created_time = "0"; }
		}

		match headers.get("file-modified-time") {
			Some ( val ) => { file_modified_time = val; },
			_ => { file_modified_time = "0"; }
		}

		match headers.get("file-metadata") {
			Some ( val ) => { file_metadata = val; },
			_ => { file_metadata = "None"; }
		}

		let file_info = FileReadInfo {
			filename: file_name.to_string(),
			filesize: file_size.parse().ok().expect("Wanted a number"),
			filecreatedtime: file_created_time.parse().ok().expect("Wanted a number"),
			filemodifiedtime: file_modified_time.parse().ok().expect("Wanted a number"),
			filemetadata: file_metadata.to_string(),
			filebody: read_file_resp_body,
		};

		return Ok( file_info ); }

		} else { return Err(ConnectionError::UnknownError) } // if end
	}
};//match end
} //fn end

// delete a file
pub fn delete_file ( delete_file_data : DeleteFileData, safe_register_resp : &super::auth::SafeRegisterResp  ) -> Result< u16 , ConnectionError > {

		println!("App: Begin deleting file...");

		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;

		// path Parameters
		let requested_file = delete_file_data.file_path ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = delete_file_data.is_path_shared;

		//println!("filePath = {}",&file_path);

		// URL to send our 'ls' request to

		let url_nfs = "http://localhost:8100/5.0/nfs/directory".to_string();
		let url_nfs_del = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_ls = {}",&url_nfs_del);

		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());

		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::delete(&url_nfs_del, &mut headers );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}
