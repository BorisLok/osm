#[derive(Debug, Clone)]
pub struct Environment {
    pub debug: bool,
    pub(crate) postgres_database: String,
    pub(crate) postgres_host: String,
    pub(crate) postgres_user: String,
    pub(crate) postgres_password: String,
    pub(crate) postgres_port: u16,
    pub(crate) api_key: String,
}

impl Environment {
    pub fn new() -> Self {
        let debug = dotenv::var("DEBUG")
            .expect("Can read debug from env.")
            .parse::<bool>()
            .unwrap_or(true);

        let postgres_database = dotenv::var("POSTGRES_DATABASE")
            .expect("Can read postgres_database from env.");

        let postgres_host = dotenv::var("POSTGRES_HOST")
            .expect("Can read postgres_host from env.");

        let postgres_user = dotenv::var("POSTGRES_USER")
            .expect("Can read postgres_user from env.");

        let postgres_password = dotenv::var("POSTGRES_PASSWORD")
            .expect("Can read postgres_password from env.");

        let postgres_port = dotenv::var("POSTGRES_PORT")
            .expect("Can't read postgres_port from env.")
            .parse::<u16>()
            .expect("Can parse port to u16.");

        let api_key = dotenv::var("API_KEY")
            .expect("Can read api key from env.");

        Self {
            debug,
            postgres_database,
            postgres_host,
            postgres_user,
            postgres_password,
            postgres_port,
            api_key,
        }
    }
}
