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
#[interactive_clap(input_context = near_cli_rs::GlobalContext)]
#[interactive_clap(output_context = ProposalsContext)]
pub struct Proposals {
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: near_cli_rs::network::Network,
}

#[derive(Clone)]
pub struct ProposalsContext(near_cli_rs::network::NetworkContext);

impl ProposalsContext {
    pub fn from_previous_context(
        previous_context: near_cli_rs::GlobalContext,
        _scope: &<Proposals as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let on_after_getting_network_callback: near_cli_rs::network::OnAfterGettingNetworkCallback =
            std::sync::Arc::new(display_proposals_info);
        Ok(Self(near_cli_rs::network::NetworkContext {
            config: previous_context.config,
            interacting_with_account_ids: vec![],
            on_after_getting_network_callback,
        }))
    }
}

impl From<ProposalsContext> for near_cli_rs::network::NetworkContext {
    fn from(item: ProposalsContext) -> Self {
        item.0
    }
}

pub fn display_proposals_info(
    network_config: &near_cli_rs::config::NetworkConfig,
) -> crate::CliResult {
    let json_rpc_client = network_config.json_rpc_client();

    let epoch_validator_info = json_rpc_client
        .blocking_call(&RpcValidatorRequest {
            epoch_reference: EpochReference::Latest,
        })
        .wrap_err("Failed to get epoch validators information request.")?;

    let current_proposals = epoch_validator_info.current_proposals;
    let current_proposals_stake: std::collections::HashMap<_, _> = current_proposals
        .clone()
        .into_iter()
        .map(|validator_stake_view| {
            let (account_id, stake) = validator_stake_view
                .into_validator_stake()
                .account_and_stake();
            (account_id, stake)
        })
        .collect();

    let current_validators = epoch_validator_info.current_validators;
    let current_validators_stake: std::collections::HashMap<_, _> = current_validators
        .into_iter()
        .map(|current_epoch_validator_info| {
            (
                current_epoch_validator_info.account_id,
                current_epoch_validator_info.stake,
            )
        })
        .collect();

    let next_validators = epoch_validator_info.next_validators;
    let mut next_validators_stake: std::collections::HashMap<_, _> = next_validators
        .into_iter()
        .map(|next_epoch_validator_info| {
            (
                next_epoch_validator_info.account_id,
                next_epoch_validator_info.stake,
            )
        })
        .collect();

    next_validators_stake.extend(current_proposals_stake.clone());

    let mut combine_validators_and_proposals: std::collections::HashMap<
        near_primitives::types::AccountId,
        ProposalsTable,
    > = std::collections::HashMap::new();
    for (account_id, stake) in next_validators_stake {
        if let Some(new_stake) = current_proposals_stake.get(&account_id) {
            let proposals_table = ProposalsTable {
                account_id: account_id.clone(),
                status: "Proposal(Accepted)".to_string(),
                stake,
                new_stake: Some(*new_stake),
            };
            combine_validators_and_proposals.insert(account_id, proposals_table)
        } else {
            let proposals_table = ProposalsTable {
                account_id: account_id.clone(),
                status: "Rollover".to_string(),
                stake,
                new_stake: None,
            };
            combine_validators_and_proposals.insert(account_id, proposals_table)
        };
    }
    let mut combine_validators_and_proposals_table: Vec<ProposalsTable> =
        combine_validators_and_proposals.into_values().collect();
    combine_validators_and_proposals_table.sort_by_key(|b| std::cmp::Reverse(b.stake));

    let partial_genesis_config = get_partial_genesis_config(&json_rpc_client)?;

    let partial_protocol_config =
        get_partial_protocol_config(&json_rpc_client, &BlockReference::Finality(Finality::Final))?;

    let max_number_of_seats = partial_protocol_config.num_block_producer_seats
        + partial_protocol_config
            .avg_hidden_validator_seats_per_shard
            .iter()
            .sum::<u64>();

    let expected_seat_price = crate::common::find_seat_price(
        combine_validators_and_proposals_table
            .iter()
            .map(|proposal| proposal.stake.as_yoctonear())
            .collect(),
        max_number_of_seats,
        partial_genesis_config.minimum_stake_ratio,
        partial_protocol_config.protocol_version,
    )?;

    let passing_proposals = combine_validators_and_proposals_table
        .iter()
        .map(|proposals| match proposals.new_stake {
            Some(new_stake) => new_stake,
            None => proposals.stake,
        })
        .filter(|stake| stake >= &expected_seat_price)
        .count();

    eprintln!(
        "Proposals for the epoch after next (new: {}, passing: {}, expected seat price = {})",
        current_proposals.len(),
        passing_proposals,
        expected_seat_price
    );

    let mut table = Table::new();
    table.set_titles(prettytable::row![Fg=>"#", "Status", "Validator Id", "Stake", "New Stake"]);

    for (index, proposals) in combine_validators_and_proposals_table
        .into_iter()
        .enumerate()
    {
        let (new_stake, status) = match proposals.new_stake {
            Some(new_stake) => {
                let status = if new_stake <= expected_seat_price {
                    "Proposal(Declined)".to_string()
                } else {
                    proposals.status
                };
                (new_stake.to_string(), status)
            }
            None => {
                let status = if proposals.stake <= expected_seat_price {
                    "Kicked out".to_string()
                } else {
                    proposals.status
                };

                ("".to_string(), status)
            }
        };
        let stake = match current_validators_stake.get(&proposals.account_id) {
            Some(stake) => stake.to_string(),
            None => "".to_string(),
        };

        table.add_row(prettytable::row![
            Fg->index + 1,
            status,
            proposals.account_id,
            stake,
            new_stake
        ]);
    }
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProposalsTable {
    pub account_id: near_primitives::types::AccountId,
    pub status: String,
    pub stake: near_token::NearToken,
    pub new_stake: Option<near_token::NearToken>,
}
