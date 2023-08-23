use diesel::PgConnection;
use crate::dtos::user::{CreateUserRequest, UpdateUserRequest};
use crate::entities::authority::Authority;
use crate::entities::pagination::Pagination;
use crate::entities::user::{User, UserList};
use crate::exceptions::api::ApiException;
use crate::repositories::user_repo::{UserFindOneClause, UserRepository};
use crate::templates::template::{Template, TemplateList};

pub struct UserService;

impl UserService {
    pub fn list(db: &mut PgConnection, auth: Authority, pag: Pagination) -> Result<TemplateList<UserList>, ApiException> {
        Ok(TemplateList::new(Some(auth), UserRepository::list(db, pag.clone())?, pag))
    }

    pub fn create(db: &mut PgConnection, auth: Authority, body: CreateUserRequest) -> Result<Template<User>, ApiException> {
        let user = UserRepository::create(db, body.transform_repo(Some(auth.get_user().clone().id)))?;
        Ok(Template::new(Some(auth), Some(user)))
    }

    pub fn find(db: &mut PgConnection, auth: Authority, id: String) -> Result<Template<User>, ApiException> {
        Ok(Template::new(Some(auth), Some(UserRepository::find(db, UserFindOneClause::Id(id))?)))
    }

    pub fn update(db: &mut PgConnection, auth: Authority, id: String, body: UpdateUserRequest) -> Result<Template<User>, ApiException> {
        let user = UserRepository::update(db, id, body.transform_repo(Some(auth.get_user().clone().id)))?;
        Ok(Template::new(Some(auth), Some(user)))
    }

    pub fn delete(db: &mut PgConnection, auth: Authority, id: String) -> Result<Template<User>, ApiException> {
        let user = UserRepository::delete(db, id, Some(auth.get_user().clone().id))?;
        Ok(Template::new(Some(auth), Some(user)))
    }
}