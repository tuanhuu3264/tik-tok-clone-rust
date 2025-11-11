use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::errors::DomainError;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGraphQLError {
    pub message: String,
    pub path: Option<Vec<String>>,
    pub extensions: Option<Value>,
}

#[async_trait]
pub trait BaseQueries<T, ID>: Send + Sync 
where
    T: Send + Sync,
    ID: Send + Sync,
{
    async fn find_by_id_cassandra(&self, id: &ID) -> Result<Option<T>, DomainError>;

    async fn find_all_cassandra(&self) -> Result<Vec<T>, DomainError>;

    async fn find_by_filter_cassandra(
        &self,
        filter: Value,
    ) -> Result<Vec<T>, DomainError>;
}

pub struct BaseQueriesHelper;

impl BaseQueriesHelper {
    pub fn build_query(query_name: &str, fields: &[&str]) -> String {
        let fields_str = fields.join(" ");
        format!("query {{ {} {{ {} }} }}", query_name, fields_str)
    }

    pub fn build_query_with_variables(
        query_name: &str,
        fields: &[&str],
        variables: &str,
    ) -> String {
        let fields_str = fields.join(" ");
        format!(
            "query {} {{ {} {{ {} }} }}",
            variables, query_name, fields_str
        )
    }
}
