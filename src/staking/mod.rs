use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod stake_proposal;
mod unstake_proposal;
mod view_stake;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
pub struct Staking {
    #[interactive_clap(subcommand)]
    staking_command: StakingCommand,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
/// What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter)
pub enum StakingCommand {
    #[strum_discriminants(strum(message = "view-stake          -   View validator stake"))]
    /// View validator stake
    ViewStake(self::view_stake::ViewStake),
    #[strum_discriminants(strum(
        message = "stake-proposal      -   To stake NEAR directly without a staking pool"
    ))]
    /// To stake NEAR directly without a staking pool
    StakeProposal(self::stake_proposal::StakeProposal),
    #[strum_discriminants(strum(
        message = "unstake-proposal    -   To unstake NEAR directly without a staking pool"
    ))]
    /// To unstake NEAR directly without a staking pool
    UnstakeProposal(self::unstake_proposal::UnstakeProposal),
}

/// Validator stake keys must be ed25519. nearcore only accepts a
/// ristretto-convertible ed25519 key in a `StakeAction`, and the 2.13
/// `near-crypto` bump makes `PublicKey` also parse `ml-dsa-65:` / `secp256k1:`
/// keys (those are transaction/access keys, not validator keys). Reject
/// anything but ed25519 up front so the user gets a clear input error instead
/// of a deterministic on-chain rejection.
fn ensure_ed25519_validator_key(
    public_key: &near_crypto::PublicKey,
) -> color_eyre::eyre::Result<()> {
    if !matches!(public_key.key_type(), near_crypto::KeyType::ED25519) {
        color_eyre::eyre::bail!(
            "Validator stake keys must be ed25519, but a `{}` key was provided.\n\
             Staking requires a ristretto-convertible ed25519 validator key; \
             ml-dsa-65 and secp256k1 keys are only valid as access/transaction keys.",
            public_key.key_type()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::ensure_ed25519_validator_key;
    use near_crypto::{KeyType, SecretKey};

    #[test]
    fn accepts_ed25519_validator_key() {
        let public_key = SecretKey::from_random(KeyType::ED25519).public_key();
        assert!(ensure_ed25519_validator_key(&public_key).is_ok());
    }

    #[test]
    fn rejects_non_ed25519_validator_keys() {
        for key_type in [KeyType::MLDSA65, KeyType::SECP256K1] {
            let public_key = SecretKey::from_random(key_type).public_key();
            assert!(
                ensure_ed25519_validator_key(&public_key).is_err(),
                "{key_type} key must be rejected as a validator stake key"
            );
        }
    }
}
