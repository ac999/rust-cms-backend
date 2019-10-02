use reqwest::multipart;

pub fn send_mail(
	  from: String
	, to:String
	, subject: String
	, body: String
	) -> Result<(()), reqwest::Error> {
	let form = multipart::Form::new()
        .text("from", from)
        .text("to", to)
        .text("subject", subject)
        .text("text", body);

    let _res = reqwest::Client::new()
        .post("https://api.mailgun.net/v3/sandboxfc282248ae5c4e9b934bd4715c2fedf7.mailgun.org/messages")
        .basic_auth("api", Some("658d7b3328a09c4ab9eb249c258575ec-19f318b0-8c6caf5d"))
        .multipart(form)
        .send()?
        .text()?;
        Ok(())
}

// pub struct Mail {
// 	  api	: String
// 	, user	: String
// 	, pass 	: Option<String>
// 	,
// }

// impl Mail{
// 	pub fn new (
// 		  api : String
// 		, user: String
// 		, pass: Option<String>
// 	) -> Mail {
// 		Mail {api: api, user: user, pass: pass}
// 	}

// 	pub fn send(&self
// 		, from: String
// 		, to: String
// 		, subject: Option<String>
// 		, body: Option<String>
// 		) -> Result<(()),reqwest::Error> {

// 		let sub = match subject{
// 			  Some(s) => s
// 			, None => String::new()
// 		};

// 		let bod = match body{
// 			  Some(b) => b
// 			, None => String::new()
// 		};

// 		let form = multipart::Form::new()
// 			.text("from"	, from)
// 			.text("to"		, to)
// 			.text("subject"	, sub)
// 			.text("body"	, bod);

// 		/*let res = reqwest::Client::new()
// 	        .post("https://api.mailgun.net/v3/sandboxfc282248ae5c4e9b934bd4715c2fedf7.mailgun.org/messages")
// 	        .basic_auth("api", Some("658d7b3328a09c4ab9eb249c258575ec-19f318b0-8c6caf5d"))
// 	        .multipart(form)
// 	        .send()?
// 	        .text()?;*/

// 	        let user = &self.user;
// 	        let pass = &self.pass;

// 	 		let _res = reqwest::Client::new()
// 	 			.post(self.api.as_str())
// 	 			.basic_auth("api",pass.as_ref())
// 	 			.multipart(form)
// 	 			.send()?
// 	 			.text()?;

//         Ok(())
// 	}

// }