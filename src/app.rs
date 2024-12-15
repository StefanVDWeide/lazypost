use serde_json::Value;
use std::{collections::HashMap, fmt};

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Url,
}

#[derive(Clone)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            RequestMethod::GET => "GET",
            RequestMethod::POST => "POST",
            RequestMethod::PUT => "PUT",
            RequestMethod::DELETE => "DELETE",
            RequestMethod::PATCH => "PATCH",
        };
        write!(f, "{}", method_str)
    }
}

pub struct Request {
    pub url: String,
    pub method: RequestMethod,
    pub response: String,
}

pub struct App {
    pub url_input: String,
    pub request_method_input: RequestMethod,
    pub requests: Vec<Request>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            url_input: String::new(),
            request_method_input: RequestMethod::GET,
            requests: Vec::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_request_values(&mut self, response: String) {
        let request = Request {
            url: self.url_input.clone(),
            method: self.request_method_input.clone(),
            response,
        };
        self.requests.push(request);
        self.url_input = String::new();
        self.currently_editing = None;
    }

    pub async fn make_request(&mut self) {
        let resp = match reqwest::get(&self.url_input).await {
            Ok(resp) => resp,
            Err(error) => panic!("An error occurred when making the request: {error:?}"),
        };

        match resp.status() {
            reqwest::StatusCode::OK => {
                let json_value: Value = resp
                    .json()
                    .await
                    .expect("An error occurred when retrieving the JSON from the response");

                // Convert the JSON value to a formatted string
                let json_string = serde_json::to_string_pretty(&json_value)
                    .expect("Failed to convert JSON to string");

                self.save_request_values(json_string);
            }
            _ => panic!("A non-200 status code was returned"),
        }
    }

    // pub fn toggle_editing(&mut self) {
    //     if let Some(edit_mode) = &self.currently_editing {
    //         match edit_mode {
    //             CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
    //             CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
    //         };
    //     } else {
    //         self.currently_editing = Some(CurrentlyEditing::Key);
    //     }
    // }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let placeholder_pairs = HashMap::from([
            ("Mercury", 0.4),
            ("Venus", 0.7),
            ("Earth", 1.0),
            ("Mars", 1.5),
        ]);
        let output = serde_json::to_string(&placeholder_pairs)?;
        println!("{}", output);
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
