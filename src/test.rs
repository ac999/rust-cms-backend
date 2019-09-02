use crate::server;
use crate::mail;

pub fn create_and_send_activation() -> String{
	let from = String::from("ZOOLX Activation <no-reply@zoolx.ro>");
	let to = String::from("andreicristian6@pm.me");
	let subject = String::from("ZOOLX Account Confirmation");
	let body = format!("Use the following link to activate account:
		https://zoolx.ro/activate/{key}", key=server::create_activation(&to));
	match mail::send_mail(from,to,subject,body){
		  Ok(()) => String::from("Mail sent. Test passed.")
		, _ => String::from("Mail not sent. Test failed.")
	}
}

pub fn create_and_send_recovery() -> String{
	let from = String::from("ZOOLX Recovery <no-reply@zoolx.ro>");
	let to = String::from("andreicristian6@pm.me");
	let subject = String::from("ZOOLX Account Recovery");
	let body = format!("Use the following link to recover account:
		https://zoolx.ro/recover/{key}", key=server::create_activation(&to));
	match mail::send_mail(from,to,subject,body){
		  Ok(()) => String::from("Mail sent. Test passed.")
		, _ => String::from("Mail not sent. Test failed.")
	}
}