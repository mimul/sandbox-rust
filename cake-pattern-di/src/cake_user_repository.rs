use common::user::User;
use anyhow::Result;

pub trait UsesRepository: Send + Sync + 'static {
    fn find_user(&self, id: String) -> Result<Option<User>>;
}
pub trait Repository {}

impl<Repository: Sync + Send + 'static> UsesRepository for Repository {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        Ok(Some(User {
            id: "cake_id_1".to_string(),
            status: true,
        }))
    }
}

pub trait ProvidesRepository: Send + Sync + 'static {
    type T: UsesRepository;
    fn user_repository(&self) -> &Self::T;
}
