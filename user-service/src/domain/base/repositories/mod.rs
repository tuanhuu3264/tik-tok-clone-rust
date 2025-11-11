pub mod base_queries;
pub mod base_commands;

pub use base_queries::{BaseQueries, BaseQueriesHelper, QueryGraphQLError};
pub use base_commands::{BaseCommands, BaseCommandsHelper, MutationGraphQLError};

