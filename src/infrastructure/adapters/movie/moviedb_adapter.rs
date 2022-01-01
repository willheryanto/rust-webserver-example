use crate::settings::{ExternalAPISettings, get_settings};
use crate::modules::movie::domain::entities::MovieEntity;
use crate::modules::movie::database::port::MovieRepositoryPort;

use reqwest::{Client, Method, RequestBuilder};
use secrecy::ExposeSecret;
use async_trait::async_trait;
use std::io::{Error, ErrorKind};

pub struct MoviedbAdapter {
    client: Client,
    settings: ExternalAPISettings,
}

impl MoviedbAdapter {
    pub fn new() -> Self {
        let settings = get_settings().expect("Failed to get settings");

        MoviedbAdapter {
            client: Client::new(),
            settings: settings.moviedb,
        }
    }

    fn request(&self, method: Method, sub_url: String) -> RequestBuilder {
        self.client
            .request(method, self.construct_url(sub_url))
            .query(&[("api_key", self.settings.api_key.expose_secret().to_string())])
    }

    fn construct_url(&self, sub_url: String) -> String {
        format!("{}{}", self.settings.base_url, sub_url)
    }

    async fn parse_response(&self, res: reqwest::Response) -> Vec<MovieEntity> {
        let body = res.text().await.expect("Failed to get body");
        let json: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse json");
        let results = json["results"].as_array().expect("Failed to get results");
        let mut movies = Vec::new();
        'outer: for result in results {
            for key in vec!["release_date", "overview", "poster_path"] {
                if result[key].is_null() {
                    continue 'outer;
                }
            }
            let id = result["id"].as_u64().expect("Failed to get id");
            let title = result["title"].as_str().expect("Failed to get title");

            let poster_path = result["poster_path"].as_str().expect("Failed to get poster_path");
            let full_poster_path = format!(
                "{}{}",
                "https://image.tmdb.org/t/p/w500",
                poster_path,
            );
            let overview = result["overview"].as_str().expect("Failed to get overview");
            let release_date = result["release_date"].as_str().expect("Failed to get release_date");
            if release_date.is_empty() {
                continue;
            }
            let release_date = chrono::NaiveDate::parse_from_str(release_date, "%Y-%m-%d").expect("Failed to parse release_date");
            let vote_average = result["vote_average"].as_f64().expect("Failed to get vote_average");

            movies.push(MovieEntity {
                id,
                title: title.to_string(),
                image_url: full_poster_path.to_string(),
                overview: overview.to_string(),
                release_date,
                vote_average,
            })
        }
        movies
    }
}

#[async_trait]
impl MovieRepositoryPort for MoviedbAdapter {
    async fn find_by_title(&self, query: String) -> Result<Vec<MovieEntity>, std::io::Error> {
        let response = self.request(Method::GET, "/search/movie".into())
            .query(&[("query", query), ("language".into(), "en-US".into())])
            .send()
            .await
            .expect("Failed to send request");

        if response.status().is_success() {
            Ok(self.parse_response(response).await)
        } else {
            Err(Error::new(ErrorKind::Other, format!("Failed to get movies: {}", response.status())))
        }
    }
}
