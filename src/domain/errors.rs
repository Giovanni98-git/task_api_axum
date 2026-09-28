use thiserror::Error;
use uuid::Uuid;
/// Erreurs métier du domaine.
/// Ces erreurs sont indépendantes de toute infrastructure.
#[derive(Debug, Error)]
pub enum DomainError {
    // Erreurs liées aux utilisateurs
    #[error("Utilisateur introuvable avec l’id : {0}")]
    UserNotFound(Uuid),
    #[error("Un utilisateur avec cet email existe déjà : {0}")]
    UserEmailAlreadyExists(String),
    #[error("Un utilisateur avec ce nom d’utilisateur existe déjà: {0}")]
    UserUsernameAlreadyExists(String),

    // Erreurs liées aux tâches
    #[error("Tâche introuvable avec l’id : {0}")]
    TaskNotFound(Uuid),
    #[error("Le titre de la tâche ne peut pas être vide")]
    TaskTitleEmpty,
    #[error("Le titre dépasse la longueur maximale de {max} caractères (reçu : {actual})")]
    TaskTitleTooLong { max: usize, actual: usize },
    // Erreur générique de persistence
    #[error("Erreur de persistence : {0}")]
    PersistenceError(String),
    // Erreur inattendue
    #[error("Erreur interne : {0}")]
    InternalError(String),
}
/// Type Result du domaine
pub type DomainResult<T> = Result<T, DomainError>;
