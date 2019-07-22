struct register{
    email: String,
    username: String,
    password: String,
    repeat-password: String,
}

struct login{
    username: String,
    password: String,
}

struct recovery{
    email: String,
}

struct user{
    id: String,
    email: String,
    username: String,
    password: String,
    token: Option(String),
    active: bool,
    register_date: String,
    last_login: String
}
