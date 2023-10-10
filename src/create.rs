//Module For Create Functons - May break out into specific submodules later
//
use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig,
};
use serde_json::json;

pub struct Collection {
    pub name: String,
    pub fields: Vec<Field>,
    pub indexes: Vec<Index>,
    pub options: CollectionOptions,
}
impl Collection{
    fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
            indexes: Vec::new(),
            options: CollectionOptions::default(),
        }
    }




}


#[tokio::main]
pub async fn create_collection(client: &QdrantClient, collection_name: &str)  {
    client
        .create_collection(&CreateCollection {
            collection_name: collection_name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 10,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await;
}

pub struct Field {
    name: String,
    field_type: FieldType,
    options: FieldOptions,
}

pub struct Index {
    name: String,
    index_type: IndexType,
    options: IndexOptions,
}

pub struct CollectionOptions {
    vector_size: usize,
    distance: Distance,
    payload_enabled: bool,
    auto_id: bool,
    indexing_enabled: bool,
    index_replicas: usize,
    max_vectors_count: usize,
    hnsw_m: usize,
    hnsw_ef_construct: usize,
    hnsw_ef_search: usize,
    payload_size: usize,
}
impl CollectionOptions {
    fn default() -> Self {
        Self {
            vector_size: 10,
            distance: Distance::Cosine,
            payload_enabled: false,
            auto_id: false,
            indexing_enabled: false,
            index_replicas: 0,
            max_vectors_count: 0,
            hnsw_m: 16,
            hnsw_ef_construct: 200,
            hnsw_ef_search: 100,
            payload_size: 0,
        }
    }
}

struct FieldOptions {
    is_indexed: bool,
    is_vector: bool,
    is_payload: bool,
    is_autogenerated: bool,
    is_stored: bool,
    is_dense: bool,
    dimension: usize,
    index: String,
}

struct IndexOptions {
    m: usize,
    ef_construct: usize,
    ef_search: usize,
}

struct FieldType {
    name: String,
    options: FieldTypeOptions,
}
struct FieldTypeOptions {
    is_indexed: bool,
    is_vector: bool,
    is_payload: bool,
    is_autogenerated: bool,
    is_stored: bool,
    is_dense: bool,
    dimension: usize,
    index: String,
}
struct IndexType {
    name: String,
    options: IndexOptions,
}
