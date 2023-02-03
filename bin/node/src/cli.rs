use std::str::FromStr;

use clap::Parser;
use sc_cli::{ChainSpec, RunCmd, RuntimeVersion, SubstrateCli};
use substrate_narwhal::Role as NarwhalRole;

use crate::{chain_spec, Error, Result};

#[derive(Debug, Parser, Clone)]
pub struct Cli {
    #[clap(flatten)]
    pub narwhal: NarwhalOpts,

    #[clap(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, Parser, Clone)]
pub struct NarwhalOpts {
    /// The role for this node. Options are:
    ///     Primary: A primary node that participates in consensus
    ///         ex: "primary"
    ///     Worker: A worker node that assists a primary
    ///         ex: "worker"
    pub role: String,
}

impl NarwhalOpts {
    pub fn parse_config(self) -> Result<NarwhalConfig> {
        Ok(NarwhalConfig {
            role: NarwhalRole::from_str(&self.role)
                .map_err(|_| Error::ParseError("failed to parse role".into()))?,
        })
    }
}

pub struct NarwhalConfig {
    /// The node role for the Narwhal system
    pub role: NarwhalRole,
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Substrate Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "support.anonymous.an".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "" | "local" => Box::new(chain_spec::local_testnet_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &node_template_runtime::VERSION
    }
}
