use async_graphql::{Schema, EmptySubscription};
use crate::presentation::graphql::resolvers::{Query, Mutation};
use crate::di::AppContext;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(context: AppContext) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(context)
        .finish()
}

