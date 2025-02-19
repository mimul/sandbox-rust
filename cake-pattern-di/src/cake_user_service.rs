use common::user::User;
use crate::cake_user_repository::{ProvidesRepository, UserRepository};
use anyhow::Result;

pub trait UsesService {
    fn find_user(&self, id: String) -> Result<Option<User>>;
}

pub trait Service: ProvidesRepository {}

impl<T: Service> UsesService for T {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        self.user_repository().find_user(id)
    }
}

pub trait ProvidesService {
    type T: UsesService;
    fn user_service(&self) -> &Self::T;
}