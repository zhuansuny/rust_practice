use std::{env, fs, collections::HashMap};

use http::{httprequest::HttpRequest, httpresponse::Httpresponse};
use serde::{Deserialize, Serialize};
use serde_json::{self};

pub trait Handler {
    fn handle(req: &HttpRequest) -> Httpresponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> Httpresponse<'static> {
        Httpresponse::new("404", None, Self::load_file("404.html"))
    }
}
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> Httpresponse<'static> {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => Httpresponse::new("200", None, Self::load_file("index.html")),
            "health" => Httpresponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map = HashMap::new();
                    if path.ends_with("css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with("js") {
                        map.insert("Content-Type", "text/javascript");
                    } else if path.ends_with("html") {
                        map.insert("Content-Type", "text/html");
                    } 
                    Httpresponse::new("200", Some(map), Some(contents))
                }
                None => Httpresponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json()-> Vec<OrderStatus>{
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path,"orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders :Vec<OrderStatus> = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> Httpresponse<'static> {
       let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[2] {
            "orders" => {
                let orders = Self::load_json();
                let json_string = serde_json::to_string(&orders).unwrap();
                Httpresponse::new("200", None, Some(json_string))
            }
            _ => Httpresponse::new("404", None, Self::load_file("404.html")),
        }
    }
}
