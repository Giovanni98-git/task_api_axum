use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// Représentation d’un utilisateur dans le domaine.
/// Cette struct est la source de vérité pour toutes les règles métier
/// liées aux utilisateurs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,

    pub username: String,

    pub email: String,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Constructeur canonique.

    /// Utilisé par le repository pour reconstruire une entité depuis la DB.

    pub fn new(
        id: Uuid,

        username: String,

        email: String,

        created_at: DateTime<Utc>,

        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            created_at,
            updated_at,
        }
    }
}
