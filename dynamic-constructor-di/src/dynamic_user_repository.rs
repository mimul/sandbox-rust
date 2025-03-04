use common::user::User;
use anyhow::Result;

pub trait UserRepository: Send + Sync + 'static {
    fn find_user(&self, id: String) -> Result<Option<User>>;
}

pub struct UserRepositoryImpl {
}

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self {  }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        Ok(Some(User {
            id: "dynamic_id_1".to_string(),
            status: true,
        }))
    }
}