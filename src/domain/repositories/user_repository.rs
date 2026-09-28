use crate::domain::entities::User;
use crate::domain::errors::DomainResult;
use async_trait::async_trait;
use uuid::Uuid;
/// Contrat définissant les opérations de persistence pour les utilisateurs.
/// Cette interface est définie dans le DOMAINE.
/// Son implémentation concrète (PostgreSQL) est dans l’INFRASTRUCTURE.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Crée un nouvel utilisateur et retourne l’entité complète.
    async fn create(&self, username: String, email: String) -> DomainResult<User>;
    /// Trouve un utilisateur par son identifiant unique.
    async fn find_by_id(&self, id: Uuid) -> DomainResult<User>;
    /// Trouve un utilisateur par son email.
    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>>;
    /// Retourne la liste de tous les utilisateurs.
    async fn find_all(&self) -> DomainResult<Vec<User>>;
}
