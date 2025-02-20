use std::sync::Arc;
use anyhow::Result;
use shaku::{module, Component, Interface};
use common::user::User;
use crate::shaku_user_repository::{UserRepository, UserRepositoryImpl};

pub trait UserService: Interface {
    fn find_user(&self, id: String) -> Result<Option<User>>;
}

#[derive(Component)]
#[shaku(interface = UserService)]
pub struct UserServiceImpl {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

impl UserService for UserServiceImpl {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        self.user_repository.find_user(id)
    }
}

module! {
    pub AppModule {
        components = [UserServiceImpl, UserRepositoryImpl],
        providers = []
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use shaku::HasComponent;
    use crate::shaku_user_service::{AppModule, UserService};

    #[test]
    fn test_shaku_find_user() {
        let module = AppModule::builder().build();
        let user_service: Arc<dyn UserService> = module.resolve();
        let user = user_service.find_user(String::from("shaku_id_1")).unwrap().unwrap();
        assert_eq!(user.id, "shaku_id_1".to_string());
    }
}
