use super::*;

pub fn handle_store_command(command: StoreCommands) -> Result<String, String> {
    match command {
        StoreCommands::Config {} => todo!(),
        StoreCommands::New {} => todo!(),
        StoreCommands::Put { data } => todo!(),
        StoreCommands::Find {
            unique,
            search_metadata,
        } => todo!(),
        StoreCommands::Count => todo!(),
    }
}

pub fn handle_ballot_command(command: BallotCommands) -> Result<String, String> {
    todo!("Handle ballot command not implimnet")
}

pub fn handle_circle_command(command: CircleCommands) -> Result<String, String> {
    todo!("Handle circle command not implimnet")
}
