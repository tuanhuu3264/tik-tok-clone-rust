use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::errors::DomainError;

#[derive(Debug, Serialize, Deserialize)]
pub struct MutationGraphQLError {
    pub message: String,
    pub path: Option<Vec<String>>,
    pub extensions: Option<Value>,
}

#[async_trait]
pub trait BaseCommands<T, ID>: Send + Sync
where
    T: Send + Sync,
    ID: Send + Sync,
{
    async fn insert_postgres(&self, entity: &T) -> Result<(), DomainError>;

    async fn update_postgres(&self, id: &ID, entity: &T) -> Result<(), DomainError>;

    async fn delete_postgres(&self, id: &ID) -> Result<(), DomainError>;

    async fn batch_insert_postgres(&self, entities: &[T]) -> Result<(), DomainError>;
}

pub struct BaseCommandsHelper;

impl BaseCommandsHelper {
    pub fn build_mutation(mutation_name: &str, fields: &[&str]) -> String {
        let fields_str = fields.join(" ");
        format!("mutation {{ {} {{ {} }} }}", mutation_name, fields_str)
    }

    pub fn build_mutation_with_variables(
        mutation_name: &str,
        fields: &[&str],
        variables: &str,
    ) -> String {
        let fields_str = fields.join(" ");
        format!(
            "mutation {} {{ {} {{ {} }} }}",
            variables, mutation_name, fields_str
        )
    }

    pub fn build_variables(vars: &[(&str, &str)]) -> Value {
        let mut map = serde_json::Map::new();
        for (key, value) in vars {
            map.insert(key.to_string(), Value::String(value.to_string()));
        }
        Value::Object(map)
    }
}
