pub struct Urls {
    pub ad_service_ad_link: String,
}

impl Urls {
    pub fn new() -> Result<Self, dotenvy::Error> {
        let ad_service_base = dotenvy::var("SERVICE_ADS_URL")?;
        let ad_service_ad_link_path = dotenvy::var("SERVICE_ADS_AD_LINK_PATH")?;

        Ok(Self {
            ad_service_ad_link: ad_service_base + &ad_service_ad_link_path,
        })
    }
}
