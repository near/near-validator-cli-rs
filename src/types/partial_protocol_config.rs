use color_eyre::eyre::Context;
use near_jsonrpc_client::methods::EXPERIMENTAL_protocol_config::RpcProtocolConfigError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PartialProtocolConfigView {
    pub protocol_version: near_primitives::types::ProtocolVersion,
    pub num_block_producer_seats: near_primitives::types::NumSeats,
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
