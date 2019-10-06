use mysql as my;

use super::config;

pub struct MyPool {
	  pub pool : my::Pool
	,
}

fn set_ssl(
	  ca_file: String
	, crt_file: String
	, key_file: String
	) -> Option<(String, Option<(String, String)>)> {
	Some((ca_file
		, Some((
			  crt_file
			, key_file
			))
		))
}

fn set_opts(_config: config::Configuration) -> my::OptsBuilder {
	let mut builder=my::OptsBuilder::new();
	builder
	.ip_or_hostname(Some(&_config.mysql.db_host))
	.tcp_port(_config.mysql.db_port)
	.user(Some(&_config.mysql.db_user))
	.pass(Some(&_config.mysql.db_pass))
	.db_name(Some(&_config.mysql.db_name))
	// Change verify_peer to true for deployment.
	.verify_peer(false);
	// .ssl_opts(set_ssl(
	// 	  _config.mysql.ssl_ca
	// 	, _config.mysql.ssl_crt
	// 	, _config.mysql.ssl_key
	// 	));
	
	builder
}

pub fn establish_connection() -> my::Pool {
	
	let _config = config::load_config()
		.expect("config load error");

	my::Pool::new(
		set_opts(_config)
		).expect(&format!("Could not connect to database."))
}
