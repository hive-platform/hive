use actix_session::Session;
use actix_web::Responder;
use actix_web::web::ServiceConfig;
use crate::dtos::auth::{AuthFirstLoginRequest, AuthLoginRequest};
use crate::extractors::req_authority::ReqAuthority;
use crate::extractors::req_box::ReqBox;
use crate::extractors::req_dto::Dto;
use crate::handlers::handler::Handler;
use crate::services::auth_service::AuthService;

pub struct AuthHandler;

impl AuthHandler {

    async fn login(tool: ReqBox, body: Dto<AuthLoginRequest>, session: Session) -> impl Responder {
        AuthService::login(tool.db, body.value, session)
    }

    async fn first_login(tool: ReqBox, body: Dto<AuthFirstLoginRequest>, session: Session) -> impl Responder {
        AuthService::first_login(tool.db, tool.cache, body.value, session)
    }

    async fn logout(_tool: ReqBox, authority: ReqAuthority, _session: Session) -> impl Responder {
        AuthService::logout(authority.0, _session)
    }

}

impl Handler for AuthHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/auth/register/", actix_web::web::post().to(Self::first_login));
        cfg.route("/auth/login/", actix_web::web::post().to(Self::login));
        cfg.route("/auth/logout/", actix_web::web::post().to(Self::logout));
    }
}