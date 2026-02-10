use color_eyre::eyre::Context;
use prettytable::Table;

use near_jsonrpc_client::methods::validators::RpcValidatorRequest;
use near_primitives::types::{BlockId, BlockReference, EpochReference, Finality};

use near_cli_rs::common::JsonRpcClientExt;

use crate::types::{
    partial_genesis_config::get_partial_genesis_config,
    partial_protocol_config::get_partial_protocol_config,
};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::network_view_at_block::NetworkViewAtBlockArgsContext)]
#[interactive_clap(output_context = AtBlockHeightContext)]
pub struct AtBlockHeight {
    /// Type the block height:
    block_height: near_primitives::types::BlockHeight,
}

#[derive(Debug, Clone)]
pub struct AtBlockHeightContext;

impl AtBlockHeightContext {
    pub fn from_previous_context(
        previous_context: super::network_view_at_block::NetworkViewAtBlockArgsContext,
        scope: &<AtBlockHeight as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let epoch_reference = EpochReference::BlockId(BlockId::Height(scope.block_height));
        display_current_validators_info(epoch_reference, &previous_context.network_config)?;
        Ok(Self)
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = super::network_view_at_block::NetworkViewAtBlockArgsContext)]
#[interactive_clap(output_context = AtBlockHashContext)]
pub struct AtBlockHash {
    /// Type the block hash:
    block_hash: near_cli_rs::types::crypto_hash::CryptoHash,
}

#[derive(Debug, Clone)]
pub struct AtBlockHashContext;

impl AtBlockHashContext {
    pub fn from_previous_context(
        previous_context: super::network_view_at_block::NetworkViewAtBlockArgsContext,
        scope: &<AtBlockHash as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let epoch_reference = EpochReference::BlockId(BlockId::Hash(scope.block_hash.into()));
        display_current_validators_info(epoch_reference, &previous_context.network_config)?;
        Ok(Self)
    }
}

pub fn display_current_validators_info(
    epoch_reference: near_primitives::types::EpochReference,
    network_config: &near_cli_rs::config::NetworkConfig,
) -> crate::CliResult {
    let block_reference = match &epoch_reference {
        EpochReference::Latest => BlockReference::Finality(Finality::Final),
        EpochReference::BlockId(block_id) => BlockReference::BlockId(block_id.clone()),
        _ => {
            return Err(color_eyre::eyre::ErrReport::msg(
                "BlockReference: incorrect value entered",
            ));
        }
    };

    let json_rpc_client = network_config.json_rpc_client();

    let mut current_validators = json_rpc_client
        .blocking_call(&RpcValidatorRequest { epoch_reference })
        .wrap_err("Failed to get epoch validators information request.")?
        .current_validators;
    current_validators.sort_by_key(|b| std::cmp::Reverse(b.stake));

    let partial_genesis_config = get_partial_genesis_config(&json_rpc_client)?;

    let partial_protocol_config = get_partial_protocol_config(&json_rpc_client, &block_reference)?;

    let max_number_of_seats = partial_protocol_config.num_block_producer_seats
        + partial_protocol_config
            .avg_hidden_validator_seats_per_shard
            .iter()
            .sum::<u64>();
    eprintln!(
        "Validators (total: {}, seat price: {})",
        current_validators.len(),
        crate::common::find_seat_price(
            current_validators
                .iter()
                .map(|current_validator| current_validator.stake.as_yoctonear())
                .collect(),
            max_number_of_seats,
            partial_genesis_config.minimum_stake_ratio,
            partial_protocol_config.protocol_version
        )?
    );

    let mut table = Table::new();
    table.set_titles(prettytable::row![Fg=>"#", "Validator Id", "Stake", "Online", "Blocks produced", "Blocks expected", "Chunks produced", "Chunks expected", "Endorsements produced", "Endorsements expected"]);

    for (index, validator) in current_validators.into_iter().enumerate() {
        let online = if validator.num_expected_blocks == 0 && validator.num_expected_chunks == 0 {
            if validator.num_expected_endorsements == 0 {
                "Not yet available".to_string()
            } else {
                format!(
                    "{:>6.2} %",
                    (validator.num_produced_endorsements * 100) as f64
                        / (validator.num_expected_endorsements) as f64
                )
            }
        } else if validator.num_expected_blocks == 0 {
            format!(
                "{:>6.2} %",
                (validator.num_produced_chunks * 100) as f64 / validator.num_expected_chunks as f64
            )
        } else if validator.num_expected_chunks == 0 {
            format!(
                "{:>6.2} %",
                (validator.num_produced_blocks * 100) as f64 / validator.num_expected_blocks as f64
            )
        } else {
            format!(
                "{:>6.2} %",
                (validator.num_produced_blocks as f64 / validator.num_expected_blocks as f64).min(
                    validator.num_produced_chunks as f64 / validator.num_expected_chunks as f64
                ) * 100_f64
            )
        };
        table.add_row(prettytable::row![
            Fg->index + 1,
            validator.account_id,
            validator.stake,
            online,
            validator.num_produced_blocks,
            validator.num_expected_blocks,
            validator.num_produced_chunks,
            validator.num_expected_chunks,
            validator.num_produced_endorsements,
            validator.num_expected_endorsements
        ]);
    }
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
    Ok(())
}
