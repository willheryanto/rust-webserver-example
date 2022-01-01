//use crate::settings::ExternalAPISettings;

//use reqwest::{Client, Method, RequestBuilder};
//use secrecy::ExposeSecret;

//trait BaseExternalAPIClient {
    //pub fn new(settings: ExternalAPISettings) -> Self {
        //ExternalAPIClient {
            //client: reqwest::Client::new(),
            //settings,
        //}
    //}

    //pub fn base_request(&self, method: Method, sub_url: String) -> RequestBuilder {
        //let url = format!("{}{}", self.settings.base_url, sub_url);
        //self.client
            //.request(method, url)
            //.query(&[("api_key", self.settings.api_key.expose_secret().to_string())])
    //}

    //fn construct_url(&self, sub_url: String) -> String {
        //format!("{}{}", self.settings.base_url, sub_url)
    //}
//}

//struct BaseExternalAPIClientA {
    //client: Client,
    //settings: ExternalAPISettings,
//}

//trait ExternalAPIClient {
    ////type BaseExternalAPIClientA;

    //fn new(settings: ExternalAPISettings) -> BaseExternalAPIClientA {
        //BaseExternalAPIClientA {
            //client: reqwest::Client::new(),
            //settings,
        //}
    //}

    //fn url(&self, sub_url: String) -> String ;
//}

//impl <T> ExternalAPIClient for T
//where
    //T: ExternalAPIClient,
//{
    //type Client = Client;
    //type ExternalAPISettings = ExternalAPISettings;

    //fn url(&self, sub_url: String) -> String {
        //self.construct_url(sub_url)
    //}
//}

//impl <T> BaseExternalAPIClient for T
//where
    //T: ExternalAPIClient,
//{
    //fn new(settings: ExternalAPISettings) -> Self {
        //T::new()
    //}

    //fn construct_url(&self, sub_url: String) -> String {
        //format!("{}{}", self.base_url(), sub_url)
    //}

    //fn base_request(&self, method: Method, sub_url: String) -> RequestBuilder {
        //self.request(method, self.construct_url(sub_url))
    //}
//}

//pub struct ExternalAPIClient {
    //client: Client,
    //settings: ExternalAPISettings,
//}

//impl ExternalAPIClient {
    //pub fn new(settings: ExternalAPISettings) -> Self {
        //ExternalAPIClient {
            //client: reqwest::Client::new(),
            //settings,
        //}
    //}

    //pub fn base_request(&self, method: Method, sub_url: String) -> RequestBuilder {
        //let url = format!("{}{}", self.settings.base_url, sub_url);
        //self.client
            //.request(method, url)
            //.query(&[("api_key", self.settings.api_key.expose_secret().to_string())])
    //}

    //fn construct_url(&self, sub_url: String) -> String {
        //format!("{}{}", self.settings.base_url, sub_url)
    //}
//}
