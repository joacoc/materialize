use std::time::Duration;

use mz_frontegg_auth::app_password::AppPassword;
use once_cell::sync::Lazy;
use reqwest::{Error, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

pub static DEFAULT_ENDPOINT: Lazy<Url> =
    Lazy::new(|| "https://cloud.materialize.com".parse().unwrap());

/// Configures a `Client`.
pub struct ClientBuilder {
    endpoint: Url,
}

pub struct ClientConfig {
    frontegg_client: mz_frontegg_auth::FronteggAuthentication,
}

impl Default for ClientBuilder {
    fn default() -> ClientBuilder {
        ClientBuilder {
            endpoint: DEFAULT_ENDPOINT.clone(),
        }
    }
}

impl ClientBuilder {
    /// Overrides the default endpoint.
    pub fn endpoint(mut self, url: Url) -> ClientBuilder {
        self.endpoint = url;
        self
    }

    /// Creates a [`Client`] that incorporates the optional parameters
    /// configured on the builder and the specified required parameters.
    pub fn build(self, config: ClientConfig) -> Client {
        let inner = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap();

        Client {
            frontegg_client: config.frontegg_client,
            auth: None,
            endpoint: self.endpoint,
            inner,
        }
    }
}

pub struct Auth {
    token: String,
}

pub struct Client {
    inner: reqwest::Client,
    frontegg_client: mz_frontegg_auth::FronteggAuthentication,
    auth: Option<Auth>,
    endpoint: Url,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CloudProvider {
    pub id: String,
    pub name: String,
    pub api_url: String,
    pub cloud_provider: String,
}

pub struct CloudProviderAndRegion {
    pub cloud_provider: CloudProvider,
    pub region: Option<Region>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub environmentd_pgwire_address: String,
    pub environmentd_https_address: String,
    pub resolvable: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub environment_controller_url: String,
}

impl Client {
    //// Get a cloud provider's region's environment
    pub async fn region_environment_details(
        region: &Region,
    ) -> Result<Option<Vec<Environment>>, Error> {
        let mut region_api_url = region.environment_controller_url
            [0..region.environment_controller_url.len() - 4]
            .to_string();
        region_api_url.push_str("/api/environment");

        Ok(Some(vec![]))
        // let response = client
        //     .get(region_api_url)
        //     .authenticate(&valid_profile.frontegg_auth)
        //     .send()
        //     .await?;
        // match response.content_length() {
        //     Some(length) => {
        //         if length > 0 {
        //             Ok(Some(response.json::<Vec<Environment>>().await?))
        //         } else {
        //             Ok(None)
        //         }
        //     }
        //     None => Ok(None),
        // }
    }

    //// Get a cloud provider's regions
    pub async fn get_cloud_provider_region_details(
        client: &Client,
        cloud_provider_region: &CloudProvider,
    ) -> Result<Vec<Region>, Error> {
        let mut region_api_url = cloud_provider_region.api_url.clone();
        region_api_url.push_str("/api/environmentassignment");

        // let response = client
        //     .get(region_api_url)
        //     .authenticate(&valid_profile.frontegg_auth)
        //     .send()
        //     .await?;
        // ensure!(response.status().is_success());
        // Ok(response.json::<Vec<Region>>().await?)
        Ok(vec![])
    }

    pub async fn list_cloud_regions(
        &self,
        valid_profile: &str,
    ) -> Result<Vec<CloudProviderAndRegion>, Error> {
        // TODO: Run requests in parallel
        let mut cloud_providers_and_regions: Vec<CloudProviderAndRegion> = Vec::new();

        Ok(cloud_providers_and_regions)
        // for cloud_provider in cloud_providers {
        //     let cloud_provider_region_details =
        //         self.get_cloud_provider_region_details(client, cloud_provider, valid_profile)
        //             .await
        //             .with_context(|| "Retrieving region details.")?;
        //     match cloud_provider_region_details.get(0) {
        //         Some(region) => cloud_providers_and_regions.push(CloudProviderAndRegion {
        //             cloud_provider: cloud_provider.clone(),
        //             region: Some(region.to_owned()),
        //         }),
        //         None => cloud_providers_and_regions.push(CloudProviderAndRegion {
        //             cloud_provider: cloud_provider.clone(),
        //             region: None,
        //         }),
        //     }
        // }
        // Ok(cloud_providers_and_regions)
    }

    pub async fn get_environment(&self, region_name: &str) -> Result<Environment, Error> {
        // let environment_details = self.region_environment_details(client, region, valid_profile)
        //     .await
        //     .with_context(|| "Environment unavailable")?;
        // let environment_list = environment_details.with_context(|| "Environment unlisted")?;
        // let environment = environment_list
        //     .get(0)
        //     .with_context(|| "Missing environment")?;

        // Ok(environment.to_owned())
        Ok(Environment {
            environmentd_pgwire_address: "s".to_string(),
            environmentd_https_address: "s".to_string(),
            resolvable: true,
        })
    }

    pub async fn get_all_environments(&self) -> Result<Environment, Error> {
        // let region = self.get_provider_region(client, valid_profile, cloud_provider_region)
        //     .await
        //     .with_context(|| "Retrieving region data.")?;

        // let environment = self.get_region_environment(client, valid_profile, &region)
        //     .await
        //     .with_context(|| "Retrieving environment data")?;

        // Ok(environment)
        Ok(Environment {
            environmentd_pgwire_address: "s".to_string(),
            environmentd_https_address: "s".to_string(),
            resolvable: true,
        })
    }

    pub async fn create_environment(&self, version: Option<String>, environmentd_extra_args: Vec<String>) -> Result<Region, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body {
            #[serde(skip_serializing_if = "Option::is_none")]
            environmentd_image_ref: Option<String>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            environmentd_extra_args: Vec<String>,
        }

        let body = Body {
            environmentd_image_ref: version.map(|v| match v.split_once(':') {
                None => format!("materialize/environmentd:{v}"),
                Some((user, v)) => format!("{user}/environmentd:{v}"),
            }),
            environmentd_extra_args,
        };

        self.build_request(Method::POST, "/");
        // client
        //     .post(format!("{:}/api/environmentassignment", cloud_provider.api_url).as_str())
        //     .authenticate(&self.frontegg_client.auth.unwrap().token)
        //     .json(&body)
        //     .send()
        //     .await?
        //     .json::<Region>()
        //     .await
    }

    /// Builds a request towards the `Client`'s endpoint
    fn build_request<P>(&self, method: Method, path: P) -> RequestBuilder
    where
        P: IntoIterator,
        P::Item: AsRef<str>,
    {
        let mut url = self.endpoint.clone();
        url.path_segments_mut()
            .expect("builder validated URL can be a base")
            .clear()
            .extend(path);
        self.inner.request(method, url)
    }

    async fn request<T>(&self, method: Method, path: &str, body: Option<T>) -> Result<RequestBuilder, Error> {
        // Makes a request using the frontegg client's authentication.
        let token = "s";
        // let token = self
            // .frontegg_client
            // .auth(AppPassword {
            //     client_id: "".parse().unwrap(),
            //     secret_key: "".parse().unwrap(),
            // })
            // .await
            // .unwrap();
        let request = self.build_request(method, path).bearer_auth(token);
        Ok(request)
    }
}

// TODO: nice error type. Use `rust_frontegg` for inspiration.
#[cfg(test)]
mod tests {
    use mz_frontegg_auth::app_password::AppPassword;

    #[test]
    fn test_app_password() {
        struct TestCase {
            input: &'static str,
        }

        for tc in [
            TestCase {
                input: "mzp_7ce3c1e8ea854594ad5d785f17d1736f1947fdcef5404adb84a47347e5d30c9f",
            },
            TestCase {
                input: "mzp_fOPB6OqFRZStXXhfF9FzbxlH_c71QErbhKRzR-XTDJ8",
            },
            TestCase {
                input:
                    "mzp_0445db36-5826-41af-84f6-e09402fc6171:a0c11434-07ba-426a-b83d-cc4f192325a3",
            },
        ] {
            let app_password: AppPassword = tc.input.parse().unwrap();
            // assert_eq!(app_password.to_string(), tc.expected_output);
        }
    }
}
