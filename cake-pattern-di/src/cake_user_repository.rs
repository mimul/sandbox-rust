use common::user::User;
use anyhow::Result;

pub trait UserRepository {
    fn find_user(&self, id: String) -> Result<Option<User>>;
}
pub trait Repository {}

impl<Repository> UserRepository for Repository {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        Ok(Some(User {
            id: "cake_pattern_id_1".to_string(),
            status: true,
        }))
    }
}

pub trait ProvidesRepository {
    type T: UserRepository;
    fn user_repository(&self) -> &Self::T;
}
