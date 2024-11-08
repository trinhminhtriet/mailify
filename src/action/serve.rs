use mailify::service::server::{Configuration, Server};

#[derive(clap::Parser)]
pub(crate) struct Action {
    /// Path to the configuration toml file, default to /etc/mailify/mailify.toml.
    #[clap(
        short,
        long,
        default_value = "/etc/mailify/mailify.toml",
        env = "CATAPULTE_CONFIG"
    )]
    pub config_path: String,
}

impl Action {
    pub(crate) async fn execute(self) {
        let config = Configuration::from_path(&self.config_path);
        Server::from_config(config).run().await
    }
}
