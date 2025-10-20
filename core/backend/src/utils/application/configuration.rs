use envconfig::Envconfig;

#[derive(Debug, Envconfig)]
pub struct ReddytConfig {
    #[envconfig(from = "RYT_ADMIN_EMAIL")]
    admin_email: String,
    #[envconfig(from = "RYT_ADMIN_PASSWORD")]
    admin_password: String
}

impl ReddytConfig {
    #[inline]
    pub fn admin_email(&self) -> &str {
        &self.admin_email
    }

    #[inline]
    pub fn admin_password(&self) -> &str {
        &self.admin_password
    }
}
