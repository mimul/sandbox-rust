use std::sync::Arc;
use common::user::User;
use anyhow::Result;
use crate::dynamic_user_repository::UserRepository;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn find_user(&self, id: String) -> Result<Option<User>> {
        self.user_repository.find_user(id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use log::info;
    use crate::dynamic_user_repository::UserRepositoryImpl;
    use crate::dynamic_user_service::UserService;

    #[test]
    fn test_dynamic_find_user() {
        let user_service = UserService::new(Arc::new(UserRepositoryImpl::new()));
        let user = user_service.user_repository.find_user(String::from("dynamic_id_1")).unwrap().unwrap();
        info!("user.id={:?}", user.id);
        assert_eq!(user.id, "dynamic_id_1".to_string());
    }
}