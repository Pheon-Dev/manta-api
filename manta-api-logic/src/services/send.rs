use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(feature = "persist")]
use tokio::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendRequest {
    pub receiver: String,
    pub amount: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSendRequest {
    pub receiver: Option<String>,
    pub amount: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentifiableSendRequest {
    pub id: usize,

    #[serde(flatten)]
    pub request: SendRequest,
}

impl IdentifiableSendRequest {
    pub fn new(id: usize, request: SendRequest) -> IdentifiableSendRequest {
        IdentifiableSendRequest { id, request }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl Pagination {
    pub fn new(offset: Option<usize>, limit: Option<usize>) -> Pagination {
        Pagination { offset, limit }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SendRequestStoreError {
    #[error("persist data store error")]
    FileAccessError(#[from] std::io::Error),
    #[error("serialization error")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct SendRequestStore {
    store: HashMap<usize, IdentifiableSendRequest>,
    id_generator: AtomicUsize,
}

impl SendRequestStore {
    pub fn from_hashmap(store: HashMap<usize, IdentifiableSendRequest>) -> Self {
        let id_generator = AtomicUsize::new(store.keys().max().map(|x| x + 1).unwrap_or(0));
        SendRequestStore {
            store,
            id_generator,
        }
    }

    pub fn get_send_requests(&self, pagination: Pagination) -> Vec<IdentifiableSendRequest> {
        self.store
            .values()
            .skip(pagination.offset.unwrap_or(0))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn get_send_request(&self, id: usize) -> Option<&IdentifiableSendRequest> {
        self.store.get(&id)
    }

    pub fn add_send_request(&mut self, request: SendRequest) -> IdentifiableSendRequest {
        let id = self.id_generator.fetch_add(1, Ordering::Relaxed);
        let new_request = IdentifiableSendRequest::new(id, request);
        self.store.insert(id, new_request.clone());
        new_request
    }

    pub fn remove_send_request(&mut self, id: usize) -> Option<IdentifiableSendRequest> {
        self.store.remove(&id)
    }

    pub fn update_send_request(
        &mut self,
        id: &usize,
        request: UpdateSendRequest,
    ) -> Option<&IdentifiableSendRequest> {
        // let mut send_request = self.store.get(&id).unwrap().clone();
        // if let Some(receiver) = request.receiver {
        //     send_request.request.receiver = Some(receiver);
        // }
        // if let Some(amount) = request.amount {
        //     send_request.request.amount = amount;
        // }
        // if let Some(description) = request.description {
        //     send_request.request.description = description;
        // }
        // self.store.insert(id, send_request);
        // self.store.remove(&id)
        if let Some(req) = self.store.get_mut(id) {
            if let Some(receiver) = request.receiver {
                req.request.receiver = receiver;
            }
            if let Some(amount) = request.amount {
                req.request.amount = amount;
            }
            if let Some(description) = request.description {
                req.request.description = description;
            }
            Some(req)
        } else {
            None
        }
    }

    #[cfg(feature = "persist")]
    pub async fn persist(&self) -> Result<(), SendRequestStoreError> {
        const FILENAME: &str = "req_store.json";

        let json = serde_json::to_string_pretty(
            &self
                .store
                .values()
                .collect::<Vec<&IdentifiableSendRequest>>(),
        )
        .map_err(SendRequestStoreError::SerializationError)?;
        fs::write(FILENAME, json.as_bytes())
            .await
            .map_err(SendRequestStoreError::FileAccessError)?;
        Ok(())
    }
}

impl From<SendRequestStore> for HashMap<usize, IdentifiableSendRequest> {
    fn from(value: SendRequestStore) -> Self {
        value.store
    }
}

