use crate::server;

pub fn create_and_send_activation() -> String{
	let from = String::from("ZOOLX Activation <no-reply@zoolx.ro>");
	let to = String::from("andreicristian6@pm.me");
	let subject = String::from("ZOOLX Account Confirmation");
	let body = format!("Use the following link to activate account:
		https://zoolx.ro/activate/{key}", key=server::create_activation(&to));
	match server::send_mail(from,to,subject,body){
		  Ok(()) => String::from("Mail sent. Test passed.")
		, _ => String::from("Mail not sent. Test failed.")
	}
}