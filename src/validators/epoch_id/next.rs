use color_eyre::eyre::Context;
use prettytable::Table;

use near_jsonrpc_client::methods::validators::RpcValidatorRequest;
use near_primitives::types::{BlockReference, EpochReference, Finality};

use near_cli_rs::common::JsonRpcClientExt;

use crate::types::{
    partial_genesis_config::get_partial_genesis_config,
    partial_protocol_config::get_partial_protocol_config,
};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = crate::validators::network_view_at_block::NetworkViewAtBlockArgsContext)]
#[interactive_clap(output_context = NextContext)]
pub struct Next {}

#[derive(Debug, Clone)]
pub struct NextContext;

impl NextContext {
    pub fn from_previous_context(
        previous_context: crate::validators::network_view_at_block::NetworkViewAtBlockArgsContext,
        _scope: &<Next as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        display_next_validators_info(&previous_context.network_config)?;
        Ok(Self)
    }
}

fn display_next_validators_info(
    network_config: &near_cli_rs::config::NetworkConfig,
) -> crate::CliResult {
    let json_rpc_client = network_config.json_rpc_client();

    let epoch_validator_info = json_rpc_client
        .blocking_call(&RpcValidatorRequest {
            epoch_reference: EpochReference::Latest,
        })
        .wrap_err("Failed to get epoch validators information request.")?;

    let current_validators = epoch_validator_info.current_validators;
    let mut current_validators_stake: std::collections::HashMap<_, _> = current_validators
        .into_iter()
        .map(|current_epoch_validator_info| {
            (
                current_epoch_validator_info.account_id,
                current_epoch_validator_info.stake,
            )
        })
        .collect();

    let mut next_validators = epoch_validator_info.next_validators;
    next_validators.sort_by_key(|b| std::cmp::Reverse(b.stake));

    let partial_genesis_config = get_partial_genesis_config(&json_rpc_client)?;

    let partial_protocol_config =
        get_partial_protocol_config(&json_rpc_client, &BlockReference::Finality(Finality::Final))?;

    let max_number_of_seats = partial_protocol_config.num_block_producer_seats
        + partial_protocol_config
            .avg_hidden_validator_seats_per_shard
            .iter()
            .sum::<u64>();
    let seat_price = crate::common::find_seat_price(
        next_validators
            .iter()
            .map(|next_validator| next_validator.stake.as_yoctonear())
            .collect(),
        max_number_of_seats,
        partial_genesis_config.minimum_stake_ratio,
        partial_protocol_config.protocol_version,
    )?;
    eprintln!(
        "Next validators (total: {}, seat price: {}):",
        next_validators.len(),
        seat_price
    );

    let mut table = Table::new();
    table.set_titles(
        prettytable::row![Fg=>"#", "Status", "Validator Id", "Previous Stake", "Stake"],
    );

    for (index, validator) in next_validators.into_iter().enumerate() {
        let mut previous_stake = "".to_string();
        let mut status = "New".to_string();
        if let Some(stake) = current_validators_stake.remove(&validator.account_id) {
            previous_stake = stake.to_string();
            status = "Rewarded".to_string();
        };
        table.add_row(prettytable::row![
            Fg->index + 1,
            status,
            validator.account_id,
            previous_stake,
            validator.stake,
        ]);
    }
    for (account_id, previous_stake) in current_validators_stake {
        table.add_row(prettytable::row![
            "",
            "Kicked out",
            account_id,
            previous_stake,
            ""
        ]);
    }

    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
    Ok(())
}
