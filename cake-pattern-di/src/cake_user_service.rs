use common::user::User;
use crate::cake_user_repository::{ProvidesRepository, UsesRepository};
use anyhow::Result;

pub trait UsesService: Send + Sync + 'static {
    fn find_user(&self, id: String) -> Result<Option<User>>;
}

pub trait Service: ProvidesRepository {}

impl<T: Service> UsesService for T {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        self.user_repository().find_user(id)
    }
}

pub trait ProvidesService: Send + Sync + 'static {
    type T: UsesService;
    fn user_service(&self) -> &Self::T;
}