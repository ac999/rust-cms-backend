reg=./request/register-user.json
log=./request/login-user.json
rec=./request/recover-user.json

header=--header "Content-Type: application/json"
request=--request POST
addr=127.0.0.1:8080

register:
	curl $(header) $(request) --data @$(reg) $(addr)/register

login:
	curl $(header) $(request) --data @$(log) $(addr)/login

recover:
	curl $(header) $(request) --data @$(rec) $(addr)/recover

test:
	while true; do\
		curl $(header) $(request) --data @$(reg) $(addr)/register; \
	done
