use std::fmt;

use sqlx::{query, query_as};

use crate::db::{AuditLogEvent, EditAuthorization, FullSolve};
use crate::traits::Linkable;
use crate::{AppError, AppResult, AppState};

id_struct!(UserId, User);

impl UserId {
    pub fn relative_url(self) -> String {
        format!("/solver?id={}", self.0)
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: Option<String>,
    pub discord_id: OptionalDiscordId,
    pub name: Option<String>,
    pub moderator: bool,
    pub moderator_notes: String,
    pub dummy: bool,
}

#[derive(Debug, Clone)]
pub struct UserData {
    pub email: Option<String>,
    pub discord_id: OptionalDiscordId,
    pub name: Option<String>,
    pub moderator: bool,
    pub moderator_notes: String,
    pub dummy: bool,
}

#[derive(serde::Serialize, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct OptionalDiscordId(pub Option<u64>);
impl From<Option<i64>> for OptionalDiscordId {
    fn from(value: Option<i64>) -> Self {
        Self(value.map(|i| i as u64).filter(|&id| id != 0))
    }
}
impl fmt::Debug for OptionalDiscordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl User {
    pub fn to_public(&self) -> PublicUser {
        PublicUser {
            id: self.id,
            name: self.name.clone(),
        }
    }

    pub fn to_header_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id.0,
            "name": self.to_public().display_name(),
            "has_name": self.name.is_some(),
            "moderator": self.moderator,
        })
    }

    /// Returns the authorization for `self` to edit `target_user`, or `None` if
    /// not authorized.
    pub fn edit_auth(&self, target: impl EditAuthorizable) -> Option<EditAuthorization> {
        target.can_be_edited_by(self)
    }
    /// Returns the authorization for `self` to edit `target_user`, or an error
    /// if not authorized.
    pub fn try_edit_auth(
        &self,
        target: impl EditAuthorizable,
    ) -> Result<EditAuthorization, AppError> {
        self.edit_auth(target).ok_or(AppError::NotAuthorized)
    }
}

pub trait EditAuthorizable {
    fn can_be_edited_by(&self, editor: &User) -> Option<EditAuthorization>;
}
impl EditAuthorizable for UserId {
    fn can_be_edited_by(&self, editor: &User) -> Option<EditAuthorization> {
        if editor.moderator {
            Some(EditAuthorization::Moderator)
        } else if editor.id == *self {
            Some(EditAuthorization::IsSelf)
        } else {
            None
        }
    }
}
impl EditAuthorizable for FullSolve {
    fn can_be_edited_by(&self, editor: &User) -> Option<EditAuthorization> {
        if editor.moderator {
            Some(EditAuthorization::Moderator)
        } else if editor.id == self.solver.id
            // Don't allow users to edit solves that have already been accepted
            // or rejected.
            && self.fmc_verified.is_none()
            && self.speed_verified.is_none()
        {
            Some(EditAuthorization::IsSelf)
        } else {
            None
        }
    }
}
impl<T: EditAuthorizable> EditAuthorizable for &T {
    fn can_be_edited_by(&self, editor: &User) -> Option<EditAuthorization> {
        T::can_be_edited_by(self, editor)
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct PublicUser {
    pub id: UserId,
    pub name: Option<String>,
}
impl Linkable for PublicUser {
    fn relative_url(&self) -> String {
        self.id.relative_url()
    }

    fn md_text(&self) -> String {
        crate::util::md_minimal_escape(&self.display_name())
    }
}
impl PublicUser {
    pub fn display_name(&self) -> String {
        match &self.name {
            Some(name) => name.to_string(),
            None => format!("user #{}", self.id.0),
        }
    }
}

impl AppState {
    pub async fn get_opt_user_from_email(&self, email: &str) -> sqlx::Result<Option<User>> {
        query_as!(
            User,
            "SELECT * FROM UserAccount WHERE email = $1",
            Some(email)
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_opt_user_from_discord_id(
        &self,
        discord_id: u64,
    ) -> sqlx::Result<Option<User>> {
        query_as!(
            User,
            "SELECT * FROM UserAccount WHERE discord_id = $1",
            Some(discord_id as i64)
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_opt_user(&self, id: UserId) -> sqlx::Result<Option<User>> {
        query_as!(User, "SELECT * FROM UserAccount WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
    }
    pub async fn get_user(&self, id: UserId) -> Result<User, AppError> {
        self.get_opt_user(id)
            .await?
            .ok_or(AppError::UserDoesNotExist)
    }

    pub async fn get_all_users(&self) -> sqlx::Result<Vec<User>> {
        query_as!(User, "SELECT * FROM UserAccount")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_or_create_user_with_email(&self, email: String) -> AppResult<User> {
        if let Some(user) = self.get_opt_user_from_email(&email).await? {
            return Ok(user);
        }

        self.check_allow_logins()?;

        let mut transaction = self.pool.begin().await?;

        let user = query_as!(
            User,
            "INSERT INTO UserAccount (email) VALUES ($1) RETURNING *",
            email
        )
        .fetch_one(&mut *transaction)
        .await?;

        let event = AuditLogEvent::Added {
            object: None,
            fields: fields_map!(user, [id, email]),
        };
        Self::add_user_log_entry(&mut transaction, &user, user.id, event).await?;

        transaction.commit().await?;

        tracing::info!(user_id = ?user.id, ?email, "New user created.");

        Ok(user)
    }

    pub async fn get_or_create_user_with_discord_id(&self, discord_id: u64) -> AppResult<User> {
        if let Some(user) = self.get_opt_user_from_discord_id(discord_id).await? {
            return Ok(user);
        }

        self.check_allow_logins()?;

        let mut transaction = self.pool.begin().await?;

        let user = query_as!(
            User,
            "INSERT INTO UserAccount (discord_id) VALUES ($1) RETURNING *",
            discord_id as i64,
        )
        .fetch_one(&mut *transaction)
        .await?;

        let event = AuditLogEvent::Added {
            object: None,
            fields: fields_map!(user, [id, discord_id]),
        };
        Self::add_user_log_entry(&mut transaction, &user, user.id, event).await?;

        transaction.commit().await?;

        tracing::info!(user_id = ?user.id, ?discord_id, "New user created.");

        Ok(user)
    }

    pub async fn get_cli_dummy_user(&self) -> Result<User, AppError> {
        self.get_dummy_user_from_name("CLI").await
    }
    #[allow(unused)]
    pub async fn get_migration_dummy_user(&self) -> Result<User, AppError> {
        self.get_dummy_user_from_name("Migration").await
    }
    #[allow(unused)]
    pub async fn get_hsc_auto_verify_dummy_user(&self) -> Result<User, AppError> {
        self.get_dummy_user_from_name("HSC Auto-Verify").await
    }
    /// Returns the dummy user with the given name, or an error if it doesn't exist.
    async fn get_dummy_user_from_name(&self, name: &str) -> Result<User, AppError> {
        query_as!(
            User,
            "SELECT * FROM UserAccount WHERE dummy AND name = $1",
            name,
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::UserDoesNotExist)
    }

    /// Updates an existing user.
    pub async fn update_user(
        &self,
        editor: &User,
        new_data: User,
        audit_log_comment: &str,
    ) -> AppResult {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let User {
            id,
            email,
            discord_id,
            name,
            moderator,
            moderator_notes,
            dummy,
        } = new_data.clone();

        let mut transaction = self.pool.begin().await?;

        let old_data = query_as!(User, "SELECT * FROM UserAccount WHERE id = $1", id.0)
            .fetch_one(&mut *transaction)
            .await?;

        query!(
            "UPDATE UserAccount
                SET name = $1, moderator_notes = $2,
                    email = $3, discord_id = $4,
                    moderator = $5, dummy = $6
                WHERE id = $7
                RETURNING id",
            //
            name,
            moderator_notes,
            //
            email,
            discord_id.0.map(|i| i as i64),
            //
            moderator,
            dummy,
            //
            id.0,
        )
        .fetch_one(&mut *transaction)
        .await?;

        let fields = changed_fields_map!(
            old_data,
            new_data,
            [name, moderator_notes, email, discord_id, moderator, dummy],
        );
        let event = AuditLogEvent::Updated {
            object: None,
            fields,
            comment: Some(audit_log_comment.trim().to_string()).filter(|s| !s.is_empty()),
        };
        Self::add_user_log_entry(&mut transaction, editor, new_data.id, event).await?;

        transaction.commit().await?;

        tracing::info!(editor_id = ?editor.id.0, ?new_data, "Updated user");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** updated user {}. \
             See [all users](<{domain_name}/users>).",
            new_data.to_public().md_link(true),
        );
        self.send_private_discord_update(msg).await;

        Ok(())
    }

    /// Adds a new user to the database.
    pub async fn add_user(&self, editor: &User, data: UserData) -> AppResult<UserId> {
        if !editor.moderator {
            return Err(AppError::NotAuthorized);
        }

        let UserData {
            email,
            discord_id,
            name,
            moderator,
            moderator_notes,
            dummy,
        } = data.clone();

        let mut transaction = self.pool.begin().await?;

        let user_id = query!(
            "INSERT INTO UserAccount
                    (name, moderator_notes,
                     email, discord_id,
                     moderator, dummy)
                VALUES ($1, $2,
                        $3, $4,
                        $5, $6)
                RETURNING id",
            //
            name,
            moderator_notes,
            //
            email,
            discord_id.0.map(|i| i as i64),
            //
            moderator,
            dummy,
        )
        .fetch_one(&mut *transaction)
        .await?
        .id;

        let user_id = UserId(user_id);

        let fields = fields_map!(
            data,
            [name, moderator_notes, email, discord_id, moderator, dummy],
        );
        let event = AuditLogEvent::Added {
            object: None,
            fields,
        };
        Self::add_user_log_entry(&mut transaction, editor, user_id, event).await?;

        transaction.commit().await?;

        tracing::info!(editor_id = ?editor.id.0, ?user_id, ?data, "Added user");
        let editor_name = editor.to_public().display_name();
        let domain_name = &*crate::env::DOMAIN_NAME;
        let msg = format!(
            "**{editor_name}** added a new user {}. \
             See [all users](<{domain_name}/users>).",
            PublicUser { id: user_id, name }.md_link(true),
        );
        self.send_private_discord_update(msg).await;

        Ok(user_id)
    }
}
