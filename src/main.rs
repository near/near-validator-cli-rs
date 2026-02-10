#![allow(clippy::large_enum_variant)]

use color_eyre::eyre::WrapErr;
use color_eyre::owo_colors::OwoColorize;

use interactive_clap::ToCliArgs;
pub use near_cli_rs::CliResult;
use near_cli_rs::Verbosity;
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod common;
mod proposals;
mod staking;
mod types;
mod update_self;
mod validators;

/// near-cli is a toolbox for interacting with NEAR protocol

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
struct Cmd {
    /// Quiet mode
    #[interactive_clap(long)]
    quiet: bool,
    /// TEACH-ME mode
    #[interactive_clap(long)]
    teach_me: bool,
    #[interactive_clap(subcommand)]
    command: self::Command,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = near_cli_rs::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(disable_back)]
/// What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter)
pub enum Command {
    #[strum_discriminants(strum(message = "validators   -   Lookup validators for given epoch"))]
    /// Lookup validators for given epoch
    Validators(self::validators::Validators),
    #[strum_discriminants(strum(
        message = "proposals    -   Show both new proposals in the current epoch as well as current validators who are implicitly proposing"
    ))]
    /// Show both new proposals in the current epoch as well as current validators who are implicitly proposing
    Proposals(self::proposals::Proposals),
    #[strum_discriminants(strum(
        message = "staking      -   For validators, there is an option to staking without deploying a staking pool smart contract (stake, unstake, view stake)"
    ))]
    /// For validators, there is an option to staking without deploying a staking pool smart contract (stake, unstake, view stake)
    Staking(self::staking::Staking),
    #[strum_discriminants(strum(message = "self-update  -   Self update near-validator"))]
    /// Self update near-validator
    SelfUpdate(self::update_self::SelfUpdateCommand),
}

fn main() -> CliResult {
    inquire::set_global_render_config(near_cli_rs::get_global_render_config());

    let config = near_cli_rs::config::Config::get_config_toml()?;

    #[cfg(not(debug_assertions))]
    let display_env_section = false;
    #[cfg(debug_assertions)]
    let display_env_section = true;
    color_eyre::config::HookBuilder::default()
        .display_env_section(display_env_section)
        .install()?;

    #[cfg(feature = "self-update")]
    let handle = std::thread::spawn(|| -> color_eyre::eyre::Result<String> {
        self::update_self::get_latest_version()
    });

    let cli = match Cmd::try_parse() {
        Ok(cli) => cli,
        Err(error) => error.exit(),
    };

    let verbosity = match (cli.quiet, cli.teach_me) {
        (true, _) => Verbosity::Quiet,
        (false, true) => Verbosity::TeachMe,
        (false, false) => Verbosity::Interactive,
    };
    near_cli_rs::setup_tracing(verbosity)?;

    let global_context = near_cli_rs::GlobalContext {
        config,
        offline: false,
        verbosity,
    };

    let cli_cmd = match <Cmd as interactive_clap::FromCli>::from_cli(
        Some(cli.clone()),
        global_context.clone(),
    ) {
        interactive_clap::ResultFromCli::Ok(cli_cmd)
        | interactive_clap::ResultFromCli::Cancel(Some(cli_cmd)) => {
            eprintln!(
                "Your console command:\n{} {}",
                std::env::args().next().as_deref().unwrap_or("./validator"),
                shell_words::join(cli_cmd.to_cli_args())
            );
            Ok(Some(cli_cmd))
        }
        interactive_clap::ResultFromCli::Cancel(None) => {
            eprintln!("Goodbye!");
            Ok(None)
        }
        interactive_clap::ResultFromCli::Back => {
            unreachable!("TopLevelCommand does not have back option")
        }
        interactive_clap::ResultFromCli::Err(optional_cli_cmd, err) => {
            if let Some(cli_cmd) = optional_cli_cmd {
                eprintln!(
                    "Your console command:\n{} {}",
                    std::env::args().next().as_deref().unwrap_or("./validator"),
                    shell_words::join(cli_cmd.to_cli_args())
                );
            }
            Err(err)
        }
    };

    #[cfg(feature = "self-update")]
    // We don't need to check the version if user has just called self-update
    if !matches!(
        cli_cmd,
        Ok(Some(CliCmd {
            command: Some(CliCommand::SelfUpdate(update_self::CliSelfUpdateCommand {})),
            ..
        }))
    ) && let Ok(Ok(latest_version)) = handle.join()
    {
        let current_version = semver::Version::parse(self_update::cargo_crate_version!())
            .wrap_err("Failed to parse current version of `near-validator`")?;

        let latest_version = semver::Version::parse(&latest_version)
            .wrap_err("Failed to parse latest version of `near-validator`")?;

        if current_version < latest_version {
            eprintln!(
                "\n`near-validator` has a new update available \x1b[2m{current_version}\x1b[0m →  \x1b[32m{latest_version}\x1b[0m"
            );
            let self_update_cli_cmd = CliCmd {
                quiet: false,
                teach_me: false,
                command: Some(CliCommand::SelfUpdate(update_self::CliSelfUpdateCommand {})),
            };
            eprintln!(
                "To update `near-validator` use: {} {}",
                std::env::args()
                    .next()
                    .as_deref()
                    .unwrap_or("./validator")
                    .yellow(),
                shell_words::join(self_update_cli_cmd.to_cli_args()).yellow()
            );
        }
    };

    cli_cmd.map(|_| ())
}
