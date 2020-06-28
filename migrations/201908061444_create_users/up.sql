CREATE TABLE users (
	  id INT NOT NULL AUTO_INCREMENT
	, username VARCHAR(16) NOT NULL
	, password VARCHAR(60) NOT NULL
	, creation_date DATE NOT NULL
	, last_login DATE
	, PRIMARY KEY ( id )
);