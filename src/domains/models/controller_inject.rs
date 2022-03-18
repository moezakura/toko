use crate::repositories;

pub struct IControllerInject {
    pub auth_repo: repositories::auth::AuthRepository,
}
