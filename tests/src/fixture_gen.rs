use casper_engine_test_support::{
    ExecuteRequestBuilder, LmdbWasmTestBuilder, DEFAULT_ACCOUNT_ADDR, PRODUCTION_RUN_GENESIS_REQUEST
};
use casper_fixtures::generate_fixture;
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs 
};


use contract::{constants::{ARG_COLLECTION_NAME, ARG_TOKEN_META_DATA, ARG_TOKEN_OWNER}, modalities::OwnershipMode};
use utility::{constants::{ARG_NFT_CONTRACT_HASH, CONTRACT_NAME, MINT_SESSION_WASM, NFT_CONTRACT_WASM, NFT_TEST_COLLECTION, TEST_PRETTY_CEP78_METADATA}, installer_request_builder::NFTMetadataKind};

mod utility;

pub(crate) fn get_nft_contract_hash(
    builder: &LmdbWasmTestBuilder,
) -> ContractHash {
    let nft_hash_addr = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have this entry in named keys")
        .into_hash()
        .expect("must get hash_addr");

    ContractHash::new(nft_hash_addr)
}


fn main() {
    generate_fixture("cep78_1.5.1-ee1.5.6-minted", PRODUCTION_RUN_GENESIS_REQUEST.clone(), |builder|{
        let install_request_builder =
            utility::installer_request_builder::InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
                .with_ownership_mode(OwnershipMode::Transferable)
                .with_nft_metadata_kind(NFTMetadataKind::CEP78)
                .with_total_token_supply(2u64);
        builder
            .exec(install_request_builder.build())
            .expect_success()
            .commit();
    
        let nft_contract_key: Key = get_nft_contract_hash(&builder).into();
        let token_owner: Key = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    
        let mint_session_call = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            MINT_SESSION_WASM,
            runtime_args! {
                ARG_NFT_CONTRACT_HASH => nft_contract_key,
                ARG_TOKEN_OWNER => token_owner,
                ARG_TOKEN_META_DATA => TEST_PRETTY_CEP78_METADATA,
                ARG_COLLECTION_NAME => NFT_TEST_COLLECTION.to_string()
            },
        )
        .build();
    
        builder.exec(mint_session_call).expect_success().commit();
    }).unwrap();
}
