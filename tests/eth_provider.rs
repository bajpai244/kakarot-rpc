#![cfg(feature = "testing")]
use std::cmp::min;
use std::str::FromStr as _;

use kakarot_rpc::eth_provider::provider::EthereumProvider;
use kakarot_rpc::models::felt::Felt252Wrapper;
use kakarot_rpc::test_utils::eoa::Eoa as _;
use kakarot_rpc::test_utils::evm_contract::EvmContract;
use kakarot_rpc::test_utils::fixtures::{counter, katana, setup};
use kakarot_rpc::test_utils::mongo::{BLOCK_HASH, BLOCK_NUMBER};
use kakarot_rpc::test_utils::{evm_contract::KakarotEvmContract, sequencer::Katana};
use reth_rpc_types::{CallInput, CallRequest};
use rstest::*;

use reth_primitives::{Address, BlockNumberOrTag, Bytes, U256, U64};

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_block_number(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();

    // When
    let block_number = eth_provider.block_number().await.unwrap();

    // Then
    // The block number is 3 because this is what we set in the mocked mongo database.
    let expected = U64::from(*BLOCK_NUMBER);
    assert_eq!(block_number, expected);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_chain_id(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();

    // When
    let chain_id = eth_provider.chain_id().await.unwrap().unwrap_or_default();

    // Then
    // ASCII code for "KKRT" is 0x4b4b5254
    assert_eq!(chain_id, U64::from(0x4b4b5254));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_block_by_hash(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();

    // When
    let block = eth_provider.block_by_hash(*BLOCK_HASH, false).await.unwrap().unwrap();

    // Then
    assert_eq!(block.header.hash, Some(*BLOCK_HASH));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_block_by_number(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();

    // When
    let block = eth_provider.block_by_number(BlockNumberOrTag::Number(*BLOCK_NUMBER), false).await.unwrap().unwrap();

    // Then
    assert_eq!(block.header.number, Some(U256::from(*BLOCK_NUMBER)));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_block_transaction_count_by_hash(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();

    // When
    let count = eth_provider.block_transaction_count_by_hash(*BLOCK_HASH).await.unwrap();

    // Then
    assert_eq!(count, U64::from(3));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_block_transaction_count_by_number(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();

    // When
    let count = eth_provider.block_transaction_count_by_number(BlockNumberOrTag::Number(*BLOCK_NUMBER)).await.unwrap();

    // Then
    assert_eq!(count, U64::from(3));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_balance(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();
    let eoa = katana.eoa();

    // When
    let eoa_balance = eth_provider.balance(eoa.evm_address().unwrap(), None).await.unwrap();

    // Then
    assert!(eoa_balance > U256::ZERO);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_storage_at(#[future] counter: (Katana, KakarotEvmContract), _setup: ()) {
    // Given
    let katana = counter.0;
    let counter = counter.1;
    let eth_provider = katana.eth_provider();
    let eoa = katana.eoa();
    let counter_address: Felt252Wrapper = counter.evm_address.into();
    let counter_address = counter_address.try_into().expect("Failed to convert EVM address");

    // When
    eoa.call_evm_contract(&counter, "inc", (), 0).await.expect("Failed to increment counter");

    // Then
    let count = eth_provider.storage_at(counter_address, U256::from(0), None).await.unwrap();
    assert_eq!(U256::from(1), count);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_nonce_eoa(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();

    // When
    let nonce = eth_provider.transaction_count(Address::ZERO, None).await.unwrap();

    // Then
    // Zero address shouldn't throw 'ContractNotFound', but return zero
    assert_eq!(U256::from(0), nonce);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_nonce_contract_account(#[future] counter: (Katana, KakarotEvmContract), _setup: ()) {
    // Given
    let katana = counter.0;
    let counter = counter.1;
    let eth_provider = katana.eth_provider();
    let counter_address: Felt252Wrapper = counter.evm_address.into();
    let counter_address = counter_address.try_into().expect("Failed to convert EVM address");

    // When
    let nonce_initial = eth_provider.transaction_count(counter_address, None).await.unwrap();

    // Then
    assert_eq!(nonce_initial, U256::from(1));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_nonce(#[future] counter: (Katana, KakarotEvmContract), _setup: ()) {
    // Given
    let katana: Katana = counter.0;
    let counter = counter.1;
    let eth_provider = katana.eth_provider();
    let eoa = katana.eoa();

    let nonce_before = eth_provider.transaction_count(eoa.evm_address().unwrap(), None).await.unwrap();

    // When
    eoa.call_evm_contract(&counter, "inc", (), 0).await.expect("Failed to increment counter");

    // Then
    let nonce_after = eth_provider.transaction_count(eoa.evm_address().unwrap(), None).await.unwrap();
    assert_eq!(nonce_before + U256::from(1), nonce_after);
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_get_code(#[future] counter: (Katana, KakarotEvmContract), _setup: ()) {
    // Given
    let katana: Katana = counter.0;
    let counter = counter.1;
    let eth_provider = katana.eth_provider();
    let counter_address: Felt252Wrapper = counter.evm_address.into();
    let counter_address = counter_address.try_into().expect("Failed to convert EVM address");

    // When
    let bytecode = eth_provider.get_code(counter_address, None).await.unwrap();

    // Then
    let counter_bytecode = <KakarotEvmContract as EvmContract>::load_contract_bytecode("Counter")
        .expect("Failed to load counter bytecode");
    let expected =
        counter_bytecode.deployed_bytecode.unwrap().bytecode.unwrap().object.into_bytes().unwrap().as_ref().to_vec();
    assert_eq!(bytecode, Bytes::from(expected));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_estimate_gas(#[future] counter: (Katana, KakarotEvmContract), _setup: ()) {
    // Given
    let eoa = counter.0.eoa();
    let eth_provider = counter.0.eth_provider();
    let counter = counter.1;

    let chain_id = eth_provider.chain_id().await.unwrap().unwrap_or_default();
    let counter_address: Felt252Wrapper = counter.evm_address.into();

    let request = CallRequest {
        from: Some(eoa.evm_address().unwrap()),
        to: Some(counter_address.try_into().unwrap()),
        input: CallInput { input: None, data: Some(Bytes::from_str("0x371303c0").unwrap()) }, // selector of "function inc()"
        chain_id: Some(chain_id),
        ..Default::default()
    };

    // When
    let estimate = eth_provider.estimate_gas(request, None).await.unwrap();

    // Then
    assert!(estimate > U256::from(0));
}

#[rstest]
#[awt]
#[tokio::test(flavor = "multi_thread")]
async fn test_fee_history(#[future] katana: Katana, _setup: ()) {
    // Given
    let eth_provider = katana.eth_provider();
    let newest_block = 3;
    let block_count = 100usize;

    // When
    let fee_history =
        eth_provider.fee_history(U256::from(block_count), BlockNumberOrTag::Number(newest_block), None).await.unwrap();

    // Then
    let actual_block_count = min(block_count, newest_block as usize + 1);
    assert_eq!(fee_history.base_fee_per_gas.len(), actual_block_count + 1);
    assert_eq!(fee_history.gas_used_ratio.len(), actual_block_count);
    assert_eq!(fee_history.oldest_block, U256::ZERO);
}
