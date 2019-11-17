use chrono::Utc;

pub mod session;
pub mod ctx;

const BRINK_DEFAULT_PORT: i16 = 7880;
const BRINK_DEFAULT_HOST: &str = "localhost";
const BRINK_DEFAULT_NAME: &str = "brinkserver";

pub struct BrinkServer {
    address: String,
    name: String,
    timestamp_start: i64,
}

pub struct BrinkServerBuilder {
    name: String,
    host: String,
    port: i16,
}

impl BrinkServer {
    pub fn new(name: String, address: String) -> BrinkServer {
        let timestamp_start = Utc::now().timestamp();

        BrinkServer { address, name, timestamp_start }
    }
}

impl BrinkServerBuilder {
    pub fn new() -> BrinkServerBuilder {
        let name = BRINK_DEFAULT_NAME.to_string();
        let host = BRINK_DEFAULT_HOST.to_string();
        let port = BRINK_DEFAULT_PORT;

        BrinkServerBuilder { name, host, port }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = host;

        self
    }

    pub fn port(mut self, port: i16) -> Self {
        self.port = port;

        self
    }

    fn build_address(&self) -> String {
        format!("{}:{}", &self.host, self.port)
    }

    pub fn build(self) -> BrinkServer {
        let address = self.build_address();

        BrinkServer::new(self.name, address)
    }
}

#[cfg(test)]
pub mod test {
    use crate::server::BrinkServerBuilder;

    #[test]
    pub fn serverbuilder_build() {
        let server = BrinkServerBuilder::new()
            .host("0.0.0.0".to_string())
            .port(6661)
            .name("brink-1".to_string())
            .build();

        assert_eq!("0.0.0.0:6661", server.address)
    }
}