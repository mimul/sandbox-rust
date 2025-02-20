use crate::static_user_repository::UserRepository;
use anyhow::Result;
use common::user::User;

pub struct UserService<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(user_repository: T) -> UserService<T> {
        UserService { user_repository }
    }

    pub fn find_user(&self, id: String) -> Result<Option<User>> {
        self.user_repository.find_user(id)
    }
}

#[cfg(test)]
mod tests {
    use log::info;
    use crate::static_user_repository::{UserRepository, UserRepositoryImpl};
    use crate::static_user_service::UserService;

    #[test]
    fn test_static_find_user() {
        let user_service = UserService::new(UserRepositoryImpl::new());
        let user = user_service.user_repository.find_user(String::from("static_id_1")).unwrap().unwrap();
        info!("user.id={:?}", user.id);
        assert_eq!(user.id, "static_id_1".to_string());
    }
}