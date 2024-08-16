use alloc::string::String;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::FromBytes, contract_messages::MessagePayload};

use crate::{constants::EVENTS, error::NFTCoreError};

use super::Event;

pub fn emit_native_string(event: Event) {
    let payload_data = event.to_json();
    runtime::emit_message(EVENTS, &payload_data.into()).unwrap_or_revert()
}

pub fn emit_native_bytes(event: Event) {
    let payload_data = event.to_json();
    let payload = MessagePayload::from_bytes(&payload_data.as_bytes())
        .unwrap_or_revert()
        .0;
    runtime::emit_message(EVENTS, &payload).unwrap_or_revert()
}

impl Event {
    pub fn to_json(&self) -> String {
        match self {
            Event::Mint(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::Burn(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::Approval(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::ApprovalForAll(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::ApprovalRevoked(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::Transfer(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::MetadataUpdated(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::Migration(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::RevokedForAll(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
            Event::VariablesSet(e) => serde_json::to_string(e)
                .map_err(|_| NFTCoreError::FailedToConvertEventToJson)
                .unwrap_or_revert(),
        }
    }
}
