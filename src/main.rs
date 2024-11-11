use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Structs for API Key Management
#[derive(Clone, Debug)]
struct ApiKey {
    key: String,
    user: Option<String>,
    limited: bool,
    request_count: u32,
    last_request_time: DateTime<Utc>,
}

impl ApiKey {
    fn new() -> Self {
        ApiKey {
            key: Uuid::new_v4().to_string(),
            user: None,
            limited: false,
            request_count: 0,
            last_request_time: Utc::now(),
        }
    }

    fn can_request(&mut self, rate_limit: u32) -> bool {
        let now = Utc::now();
        let diff = now.signed_duration_since(self.last_request_time);

        if diff.num_seconds() > 60 {
            self.request_count = 0;
            self.last_request_time = now;
            self.limited = false;
        }

        if self.request_count < rate_limit {
            self.request_count += 1;
            true
        } else {
            self.limited = true;
            false
        }
    }
}

// Endpoint Structure
#[derive(Clone)]
struct Endpoint {
    path: String,
    method: String,
    access: String,
    handler: fn(HttpRequest) -> HttpResponse,
}

// Server Configuration
struct ServerConfig {
    encrypt: bool,
    api_enabled: bool,
    rate_limit: u32,
    page_404: String,
}

// EndpointManager
struct EndpointManager {
    endpoints: Mutex<Vec<Endpoint>>,
    api_keys: Mutex<HashMap<String, ApiKey>>,
    config: ServerConfig,
}

impl EndpointManager {
    fn new() -> Self {
        EndpointManager {
            endpoints: Mutex::new(Vec::new()),
            api_keys: Mutex::new(HashMap::new()),
            config: ServerConfig {
                encrypt: false,
                api_enabled: false,
                rate_limit: 1000,
                page_404: String::from("404.html"),
            },
        }
    }

    fn register_endpoint(&self, endpoint: Endpoint) {
        self.endpoints.lock().unwrap().push(endpoint);
    }

    fn get_endpoint(&self, path: &str, method: &str) -> Option<Endpoint> {
        self.endpoints
            .lock()
            .unwrap()
            .iter()
            .find(|e| e.path == path && e.method == method)
            .cloned()
    }
}

// Custom Response Structure
#[derive(Serialize)]
struct CustomResponse {
    status: u16,
    message: String,
    data: Option<serde_json::Value>,
}

impl CustomResponse {
    fn new(status: u16, message: String, data: Option<serde_json::Value>) -> Self {
        CustomResponse {
            status,
            message,
            data,
        }
    }

    fn to_http_response(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(self)
    }
}

// Request handlers
async fn handle_get(
    req: HttpRequest,
    data: web::Data<EndpointManager>,
) -> impl Responder {
    handle_request(req, data, "GET").await
}

async fn handle_post(
    req: HttpRequest,
    data: web::Data<EndpointManager>,
) -> impl Responder {
    handle_request(req, data, "POST").await
}

async fn handle_request(
    req: HttpRequest,
    endpoint_manager: web::Data<EndpointManager>,
    method: &str,
) -> impl Responder {
    let path = req.path().to_string();

    if let Some(endpoint) = endpoint_manager.get_endpoint(&path, method) {
        // Handle API key validation if needed
        if endpoint.access == "LIMIT" && endpoint_manager.config.api_enabled {
            // Implement API key validation logic here
        }

        // Call the endpoint handler
        (endpoint.handler)(req)
    } else {
        HttpResponse::NotFound().json(CustomResponse::new(
            404,
            "Endpoint not found".to_string(),
            None,
        ))
    }
}

// Example endpoint registration
fn register_example_endpoint(manager: &EndpointManager) {
    let endpoint = Endpoint {
        path: "/api/example".to_string(),
        method: "GET".to_string(),
        access: "NO_LIMIT".to_string(),
        handler: |_req| {
            HttpResponse::Ok().json(CustomResponse::new(
                200,
                "Success".to_string(),
                Some(serde_json::json!({"message": "Hello, World!"})),
            ))
        },
    };

    manager.register_endpoint(endpoint);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let endpoint_manager = web::Data::new(EndpointManager::new());

    // Register example endpoint
    register_example_endpoint(&endpoint_manager);

    HttpServer::new(move || {
        App::new()
            .app_data(endpoint_manager.clone())
            .route("/{path:.*}", web::get().to(handle_get))
            .route("/{path:.*}", web::post().to(handle_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
