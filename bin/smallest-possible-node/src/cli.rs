use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub struct Cli {
    #[clap(flatten)]
    pub run: RunCmd,
}
