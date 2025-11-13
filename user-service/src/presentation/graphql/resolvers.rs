use async_graphql::{Context, Object, Result as GraphQLResult};
use crate::di::AppContext;
use crate::application::use_cases::auth::{RegisterUseCase, LoginUseCase, LogoutUseCase};
use crate::application::use_cases::profile::GetProfileUseCase;
use crate::domain::users::entities::user::User;
use crate::domain::value_objects::{Email, Username, UserId};
use uuid::Uuid;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn health(&self) -> &str {
        "OK"
    }

    async fn get_profile(
        &self,
        ctx: &Context<'_>,
        user_id: String,
    ) -> GraphQLResult<ProfileResponse> {
        let context = ctx.data::<AppContext>()?;
        
        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid user ID: {}", e)))?;
        
        let user_id_vo = UserId::from_uuid(user_uuid);
        
        let get_profile_use_case = GetProfileUseCase::new(context.profile_repository.clone());

        let result = get_profile_use_case.execute(user_id_vo).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(ProfileResponse {
            user_id: result.profile.user_id.as_uuid().to_string(),
            display_name: result.profile.display_name,
            bio: result.profile.bio,
            avatar_url: result.profile.avatar_url,
            created_at: result.profile.created_at.to_rfc3339(),
            updated_at: result.profile.updated_at.to_rfc3339(),
            status_id: result.profile.status_id.as_uuid().to_string(),
        })
    }
}

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
    ) -> GraphQLResult<RegisterResponse> {
        let context = ctx.data::<AppContext>()?;
        
        let register_use_case = RegisterUseCase::new(
            context.user_repository.clone(),
            context.profile_repository.clone(),
        );

        let result = register_use_case.execute(username, email, password).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(RegisterResponse {
            success: true,
            message: "User registered successfully".to_string(),
            user: UserResponse {
                id: result.user.id.as_uuid().to_string(),
                username: result.user.username.as_str().to_string(),
                email: result.user.email.as_str().to_string(),
                created_at: result.user.created_at.to_rfc3339(),
                updated_at: result.user.updated_at.to_rfc3339(),
            },
            tokens: AuthTokens {
                access_token: result.access_token,
                refresh_token: result.refresh_token,
                access_token_expires_at: result.access_token_expires_at.timestamp(),
                refresh_token_expires_at: result.refresh_token_expires_at.timestamp(),
            },
        })
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> GraphQLResult<LoginResponse> {
        let context = ctx.data::<AppContext>()?;
        
        let login_use_case = LoginUseCase::new(context.user_repository.clone());

        let result = login_use_case.execute(email, password).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
            user: UserResponse {
                id: result.user.id.as_uuid().to_string(),
                username: result.user.username.as_str().to_string(),
                email: result.user.email.as_str().to_string(),
                created_at: result.user.created_at.to_rfc3339(),
                updated_at: result.user.updated_at.to_rfc3339(),
            },
            tokens: AuthTokens {
                access_token: result.access_token,
                refresh_token: result.refresh_token,
                access_token_expires_at: result.access_token_expires_at.timestamp(),
                refresh_token_expires_at: result.refresh_token_expires_at.timestamp(),
            },
        })
    }

    async fn logout(
        &self,
        ctx: &Context<'_>,
        access_token: String,
    ) -> GraphQLResult<LogoutResponse> {
        let context = ctx.data::<AppContext>()?;
        
        let logout_use_case = LogoutUseCase::new(context.redis_cache.clone());

        logout_use_case.execute(access_token).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(LogoutResponse {
            success: true,
            message: "Logout successful".to_string(),
        })
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user: UserResponse,
    pub tokens: AuthTokens,
}

#[derive(async_graphql::SimpleObject)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user: UserResponse,
    pub tokens: AuthTokens,
}

#[derive(async_graphql::SimpleObject)]
pub struct LogoutResponse {
    pub success: bool,
    pub message: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_at: i64,
    pub refresh_token_expires_at: i64,
}

#[derive(async_graphql::SimpleObject)]
pub struct ProfileResponse {
    pub user_id: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub status_id: String,
}

