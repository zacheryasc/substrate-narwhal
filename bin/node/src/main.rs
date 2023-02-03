//! Substrate Node Template CLI library.
// #![warn(missing_docs)]

use node_template::{new_primary, new_worker, Cli, Error, Result};
use sc_cli::SubstrateCli;
use substrate_narwhal::Role as NarwhalRole;

pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    let runner = cli.create_runner(&cli.run)?;
    let narwhal_config = cli.narwhal.parse_config()?;
    runner
        .run_node_until_exit(|config| async move {
            match narwhal_config.role {
                NarwhalRole::Primary => new_primary(config, narwhal_config),
                NarwhalRole::Worker => new_worker(config, narwhal_config),
            }
        })
        .map_err(Error::from)
}

fn main() -> Result<()> {
    run()
}
