use buffapi::{
    apis::{
        self, configuration::Configuration, cratehandlersadminmoderators_api::SelfInfoError,
        cratehandlersauthadmins_api::LoginError,
    },
    models::{AdminLoginResponse, Credentials, ModeratorOrAdminInfo},
};

pub struct ApiService {
    configuration: Configuration,
}

impl ApiService {
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }

    pub async fn moderator_info(
        &mut self,
    ) -> Result<ModeratorOrAdminInfo, apis::Error<SelfInfoError>> {
        apis::cratehandlersadminmoderators_api::self_info(&self.configuration).await
    }

    pub async fn login(
        &mut self,
        login: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<AdminLoginResponse, apis::Error<LoginError>> {
        let credentials = Credentials {
            login: login.into(),
            password: password.into(),
        };

        let response =
            apis::cratehandlersauthadmins_api::login(&self.configuration, credentials).await?;
        self.configuration.api_key = Some(apis::configuration::ApiKey {
            prefix: Some("Bearer".to_owned()),
            key: response.token.clone(),
        });
        Ok(response)
    }
}
