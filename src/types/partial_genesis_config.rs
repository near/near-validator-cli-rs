use color_eyre::eyre::Context;
use near_jsonrpc_client::methods::EXPERIMENTAL_genesis_config::RpcGenesisConfigError;
use num_rational::Rational32;

fn default_minimum_stake_ratio() -> Rational32 {
    Rational32::new(160, 1_000_000)
}

#[derive(Debug, serde::Deserialize)]
pub struct PartialGenesisConfig {
    #[serde(default = "default_minimum_stake_ratio")]
    pub minimum_stake_ratio: Rational32,
}

impl near_jsonrpc_client::methods::RpcHandlerResponse for PartialGenesisConfig {}

pub fn get_partial_genesis_config(
    json_rpc_client: &near_jsonrpc_client::JsonRpcClient,
) -> color_eyre::eyre::Result<PartialGenesisConfig> {
    let request = near_jsonrpc_client::methods::any::<
        Result<PartialGenesisConfig, RpcGenesisConfigError>,
    >("EXPERIMENTAL_genesis_config", serde_json::json!(null));

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(json_rpc_client.call(request))
        .wrap_err("Failed to get genesis config.")
}
