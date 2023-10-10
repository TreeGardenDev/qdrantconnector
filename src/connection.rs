use qdrant_client::prelude::*;
use std::error::Error;

pub fn generate_client() -> Option<QdrantClient> {
    let client = QdrantClient::from_url("http://localhost:6334").build();

    Some(client.unwrap())

}
