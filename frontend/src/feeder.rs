use super::yew::format::{Nothing, Json};
use super::yew::services::fetch::{FetchService, FetchHandle, Request, Response};
use super::yew::html::Callback;

use super::{Channel, Item, Category};
use super::BASE_URL;

pub struct FeederService {
    web: FetchService,
}

impl FeederService {
    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn channels(&mut self, callback: Callback<Result<Vec<Channel>, ()>>) -> FetchHandle {
        let url = format!("{}/channels", BASE_URL);
        let handler = move |response: Response<Json<Result<Vec<Channel>, ()>>>| {
            let (_, Json(data)) = response.into_parts();
            callback.emit(data)
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }
    pub fn items(&mut self, id: i32, callback: Callback<Result<Vec<Item>, ()>>) -> FetchHandle {
        let url = format!("{}/items/{}", BASE_URL, id);
        let handler = move |response: Response<Json<Result<Vec<Item>, ()>>>| {
            let (_, Json(data)) = response.into_parts();
            callback.emit(data)
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }
}
