use crate::domain::entities::{Task, TaskStatus};
use crate::domain::errors::DomainResult;
use async_trait::async_trait;
use uuid::Uuid;
/// Contrat définissant les opérations de persistence pour les tâches.
#[async_trait]
pub trait TaskRepository: Send + Sync {
    /// Crée une nouvelle tâche.
    async fn create(
        &self,
        title: String,
        description: Option<String>,
        user_id: Uuid,
    ) -> DomainResult<Task>;
    /// Trouve une tâche par son identifiant.
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Task>;
    /// Retourne toutes les tâches d’un utilisateur donné.
    async fn find_by_user_id(&self, user_id: Uuid) -> DomainResult<Vec<Task>>;
    /// Retourne toutes les tâches, toutes confondues.
    async fn find_all(&self) -> DomainResult<Vec<Task>>;
    /// Met à jour le titre, la description et le statut d’une tâche.
    async fn update(
        &self,
        id: Uuid,
        title: Option<String>,
        description: Option<Option<String>>,
        status: Option<TaskStatus>,
    ) -> DomainResult<Task>;
    /// Supprime une tâche par son identifiant.
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}
