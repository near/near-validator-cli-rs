use color_eyre::eyre::Context;
use near_jsonrpc_client::methods::EXPERIMENTAL_protocol_config::RpcProtocolConfigError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PartialProtocolConfigView {
    pub protocol_version: near_primitives::types::ProtocolVersion,
    pub num_block_producer_seats: near_primitives::types::NumSeats,
    // Removed from `ProtocolConfigView` in nearcore 2.12 (protocol 84) — see nearcore
    // commit 923d5b316b ("chore: clean up deprecated fields in EpochConfig", PR #15425).
    // Defaulting to an empty Vec is correct: modern protocols have no hidden validator
    // seats, so the sum used in `block_id::display_current_validators_info` is 0.
    #[serde(default)]
    pub avg_hidden_validator_seats_per_shard: Vec<near_primitives::types::NumSeats>,
}

impl near_jsonrpc_client::methods::RpcHandlerResponse for PartialProtocolConfigView {}

pub fn get_partial_protocol_config(
    json_rpc_client: &near_jsonrpc_client::JsonRpcClient,
    block_reference: &near_primitives::types::BlockReference,
) -> color_eyre::eyre::Result<PartialProtocolConfigView> {
    let request = near_jsonrpc_client::methods::any::<
        Result<PartialProtocolConfigView, RpcProtocolConfigError>,
    >(
        "EXPERIMENTAL_protocol_config",
        serde_json::to_value(block_reference)?,
    );

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(json_rpc_client.call(request))
        .wrap_err("Failed to get protocol config.")
}
