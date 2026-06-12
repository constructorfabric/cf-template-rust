use {{ crate_name }}_sdk::Pokemon;
use toolkit::async_trait;
use toolkit_db::secure::DBRunner;
use toolkit_odata::{ODataQuery, Page};
use toolkit_security::AccessScope;
use uuid::Uuid;

use crate::domain::error::DomainError;

/// Repository trait for Pokemon persistence operations.
#[async_trait]
pub trait PokemonRepository: Send + Sync {
    /// Find a pokemon by ID within the given security scope.
    async fn get<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<Option<Pokemon>, DomainError>;

    /// List pokemon with cursor-based pagination and OData filtering.
    async fn list_page<C: DBRunner>(
        &self,
        runner: &C,
        scope: &AccessScope,
        query: &ODataQuery,
    ) -> Result<Page<Pokemon>, DomainError>;
}
