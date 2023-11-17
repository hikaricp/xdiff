use anyhow::Result;
use clap::Parser;

/// Diff two http requests and compare the difference of the responses
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct RunArgs {
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Overrides args. Could be used to override the query, headers and body of the request
    /// For query params, use `-e key=value`.
    /// For headers, use `-e %key=value`.
    /// For body, use `-e @key=value`.
    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
    extra_params: Vec<KeyVal>,
}

pub(crate) enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone)]
pub(crate) struct KeyVal {
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, "=");
    let key = parts.next().unwrap();
    let value = parts.next().unwrap();
    todo!()
}
