#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = near_cli_rs::GlobalContext)]
#[interactive_clap(output_context = UnstakeProposalContext)]
pub struct UnstakeProposal {
    #[interactive_clap(skip_default_input_arg)]
    /// What is the validator account ID?
    validator: near_cli_rs::types::account_id::AccountId,
    /// Validator key which will be used to sign transactions on behalf of signer_id:
    public_key: near_cli_rs::types::public_key::PublicKey,
    #[interactive_clap(named_arg)]
    /// Select network
    network_config: near_cli_rs::network_for_transaction::NetworkForTransactionArgs,
}

#[derive(Debug, Clone)]
pub struct UnstakeProposalContext {
    global_context: near_cli_rs::GlobalContext,
    validator: near_primitives::types::AccountId,
    public_key: near_crypto::PublicKey,
}

impl UnstakeProposalContext {
    pub fn from_previous_context(
        previous_context: near_cli_rs::GlobalContext,
        scope: &<UnstakeProposal as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Self {
            global_context: previous_context,
            validator: scope.validator.clone().into(),
            public_key: scope.public_key.clone().into(),
        })
    }
}

impl From<UnstakeProposalContext> for near_cli_rs::commands::ActionContext {
    fn from(item: UnstakeProposalContext) -> Self {
        let get_prepopulated_transaction_after_getting_network_callback: near_cli_rs::commands::GetPrepopulatedTransactionAfterGettingNetworkCallback =
            {
                let validator = item.validator.clone();
                std::sync::Arc::new(move |_network_config| {
                Ok(near_cli_rs::commands::PrepopulatedTransaction {
                    signer_id: validator.clone(),
                    receiver_id: validator.clone(),
                    actions: vec![near_primitives::transaction::Action::Stake(
                        Box::new(near_primitives::transaction::StakeAction {
                            stake: near_token::NearToken::from_yoctonear(0),
                            public_key: item.public_key.clone(),
                        }),
                    )],
                })
            })};
        Self {
            global_context: item.global_context,
            interacting_with_account_ids: vec![item.validator],
            get_prepopulated_transaction_after_getting_network_callback,
            on_before_signing_callback: std::sync::Arc::new(
                |_prepolulated_unsinged_transaction, _network_config| Ok(()),
            ),
            on_before_sending_transaction_callback: std::sync::Arc::new(
                |_signed_transaction, _network_config| Ok(String::new()),
            ),
            on_after_sending_transaction_callback: std::sync::Arc::new(
                |_outcome_view, _network_config| Ok(()),
            ),
            on_sending_delegate_action_callback: None,
            sign_as_delegate_action: false,
        }
    }
}

impl UnstakeProposal {
    pub fn input_validator(
        context: &near_cli_rs::GlobalContext,
    ) -> color_eyre::eyre::Result<Option<near_cli_rs::types::account_id::AccountId>> {
        near_cli_rs::common::input_signer_account_id_from_used_account_list(
            &context.config.credentials_home_dir,
            "What is the validator account ID?",
        )
    }
}
