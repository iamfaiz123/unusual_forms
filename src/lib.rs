pub mod get_config;
pub use crate::get_config::get_configuration;
mod service_starter;
pub use actix_web::middleware::Logger;
pub use mongodb::bson::{doc, Document};
pub use mongodb::{options::ClientOptions, Client};
pub use service_starter::run;
pub use std::net::TcpListener;
mod routes;
pub use routes::*;
mod collection;
pub use collection::*;
use log::{info, trace, warn};


