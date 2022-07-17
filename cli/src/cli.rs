use chrono::prelude::*;
use chrono::Utc;
use clap::{Parser, Subcommand};
use libp2p::Multiaddr;
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    /// Subcommands to access specific components
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Join and manage your participation in the circle of facilitators
    Circle {
        #[clap(subcommand)]
        command: CircleCommands,
    },
    /// Vote, count, add and verify votes and voters
    Ballot {
        #[clap(subcommand)]
        command: BallotCommands,
    },
    /// Manually manupilate the chains and blocks stored on offline
    Store {
        #[clap(subcommand)]
        command: StoreCommands,
    },
}

#[derive(Subcommand)]
pub enum CircleCommands {
    /// Create a new circle
    New {
        /// Define the name of the new circle to be announced for other node to connect
        name: String,
        /// Define the date of the starting of votes, in the format YYYY-MM-DDThh:mm:ssZ
        #[clap(value_parser)]
        start_date: DateTime<Utc>,
        /// Define the date of the end of votes, in the format YYYY-MM-DDThh:mm:ssZ
        #[clap(value_parser)]
        end_date: DateTime<Utc>,
    },
    /// Manage avalible options to be voted for in the circle
    Option {
        #[clap(subcommand)]
        command: CircleOptionsCommands,
    },
    /// Join an existing circle
    Join {
        /// Define the name of the new circle to be joined by the current node
        name: String,
        /// Define a list of known nodes in circle to connect to
        #[clap(short, long, value_parser, value_name = "Addresses")]
        know_nodes: Option<Vec<Multiaddr>>,
    },
    /// Configure and manage discovery of circles
    Discovery {
        /// Set the source for fetching circles
        #[clap(short, long, value_parser, value_name = "URL")]
        publishing_url: Url,
        /// Set the source for fetching circles
        #[clap(short, long, value_parser, value_name = "URL")]
        discovery_source: Url,
    },
}

#[derive(Subcommand)]
pub enum CircleOptionsCommands {
    /// Add an option for the voters
    Add {
        /// Displayed name of the option
        name: String,
    },
    /// Remove an option for the voters
    Remove {
        /// Remove option using it's unique identifier
        id: String,
    },
    /// Edit an option for the voters
    Edit {
        /// Edit an option based on it's unique identifiers
        id: String,
        name: String,
    },
}

#[derive(Subcommand)]
pub enum BallotCommands {
    /// Register for to vote for your options
    Resister {
        /// Enter a unique identifer that will be hashed to maintain your privacy
        #[clap(short, long, value_parser, value_name = "Unique Identifier")]
        unique: String,
        /// Enter the circle you wish to register in
        #[clap(short, long, value_parser, value_name = "Circle")]
        circle: String,
    },
    /// Get options if your in a circle or provide the circle you wish to query
    GetOptions {
        /// Circle to query from
        #[clap(short, long, value_parser, value_name = "Circle")]
        circle: String,
    },
    /// Cast a vote in the circle your part of or wish to participate in
    CastVote {
        /// Circle you wish to participate in
        #[clap(short, long, value_parser, value_name = "Circle")]
        circle: String,
        /// Enter a unique identifer that will be hashed to maintain your privacy
        #[clap(short, long, value_parser, value_name = "Unique Identifier")]
        unique: String,
        /// Option number number
        #[clap(short, long, value_parser, value_name = "Unique Identifier")]
        choice: i32,
    },
    CheckVote {
        /// Enter a unique identifer that will be hashed to maintain your privacy
        #[clap(short, long, value_parser, value_name = "Unique Identifier")]
        unique: String,
        /// Enter the circle you wish to register in
        #[clap(short, long, value_parser, value_name = "Circle")]
        circle: String,
    },
}

#[derive(Subcommand)]
pub enum StoreCommands {
    /// Define the configurations for the current store store you wish
    Config {},
    /// Create a new store using the current config context
    New {},
    /// Put information into the current store
    Put {
        /// Data to be provided as a path to a file or a string
        data: String,
    },
    /// Find if an entry is in the the store
    Find {
        /// Find if with the given unique peace is present
        unique: String,
        /// Set useful metadata to help optimize search
        search_metadata: String,
    },
    /// Get a count of the number of entries
    Count,
}

fn datable(s: &str) -> Result<String, String> {
    todo!("")
}
