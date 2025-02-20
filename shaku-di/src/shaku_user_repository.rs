use anyhow::Result;
use shaku::{Component, Interface};
use common::user::User;

pub trait UserRepository: Interface {
    fn find_user(&self, id: String) -> Result<Option<User>>;
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl {
}

impl UserRepository for UserRepositoryImpl {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        Ok(Some(User {
            id: "shaku_id_1".to_string(),
            status: true,
        }))
    }
}