use mysql as my;

use crate::config;

fn get_ssl(
	_config: config::ConfigStruct
	) -> Option<(String, Option<(String, String)>)> {
	Some((_config.get_ssl_ca()
		, Some((
			  _config.get_ssl_crt()
			, _config.get_ssl_key()
			))
		))
}

fn get_opts(_config: config::ConfigStruct) -> my::OptsBuilder{
	let mut builder=my::OptsBuilder::new();
	builder
	.ip_or_hostname(Some(&_config.get_db_host()))
	.tcp_port(_config.get_db_port())
	// .user(Some(&_config.get_db_user()))
	// .pass(Some(&_config.get_db_pass()))
	.db_name(Some(&_config.get_db_name()))
	.verify_peer(false)
	.ssl_opts(get_ssl(_config));
	builder
}

pub fn establish_connection(_config: config::ConfigStruct) -> my::Pool {
	
	my::Pool::new(
		get_opts(_config)
		).expect(&format!("Could not connect to database."))
}