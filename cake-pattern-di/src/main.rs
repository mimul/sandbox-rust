use cake_pattern_di::cake_user_repository::{ProvidesRepository, Repository};
use cake_pattern_di::cake_user_service::{ProvidesService, Service, UsesService};

pub struct CakeModule {}

impl CakeModule {
    pub fn new() -> Self {
        Self{}
    }
}

impl Repository for CakeModule {}
impl Service for CakeModule {}

impl ProvidesRepository for CakeModule {
    type T = Self;

    fn user_repository(&self) -> &Self::T {
        self
    }
}

impl ProvidesService for CakeModule {
    type T = Self;

    fn user_service(&self) -> &Self::T {
        self
    }
}

fn main() {
    let user = CakeModule::new().user_service().find_user("test".to_string()).unwrap().unwrap();
    assert_eq!(user.id, "cake_pattern_id_1".to_string());
}