use actix_web::Responder;
use actix_web::web::{Path, ServiceConfig};
use crate::dtos::user::{CreateAccessTokenRequest, CreateUserRequest, UpdateUserRequest};
use crate::extractors::req_authority::ReqAuthority;
use crate::extractors::req_box::ReqBox;
use crate::extractors::req_dto::Dto;
use crate::extractors::req_queries::ReqPagination;
use crate::handlers::handler::Handler;
use crate::services::user_service::UserService;

pub struct UserHandler;

impl UserHandler {

    async fn list(mut tool: ReqBox, auth: ReqAuthority, pag: ReqPagination) -> impl Responder {
        UserService::list(&mut tool.db, auth.0, pag.0)
    }

    async fn create(mut tool: ReqBox, auth: ReqAuthority, body: Dto<CreateUserRequest>) -> impl Responder {
        UserService::create(&mut tool.db, auth.0, body.value)
    }

    async fn find(mut tool: ReqBox, auth: ReqAuthority, path: Path<String>) -> impl Responder {
        UserService::find(&mut tool.db, auth.0, path.into_inner())
    }

    async fn update(mut tool: ReqBox, auth: ReqAuthority, path: Path<String>,body: Dto<UpdateUserRequest>) -> impl Responder {
        UserService::update(&mut tool.db, auth.0, path.into_inner(), body.value)
    }

    async fn delete(mut tool: ReqBox, auth: ReqAuthority, path: Path<String>) -> impl Responder {
        UserService::delete(&mut tool.db, auth.0, path.into_inner())
    }

    async fn list_access_token(mut tool: ReqBox, auth: ReqAuthority, pag: ReqPagination, path: Path<String>) -> impl Responder {
        UserService::list_access_token(&mut tool.db, auth.0,pag.0, path.into_inner())
    }

    async fn create_access_token(tool: ReqBox, auth: ReqAuthority, path: Path<String>, body: Dto<CreateAccessTokenRequest>) -> impl Responder {
        UserService::create_access_token(tool, auth.0, path.into_inner(), body.value)
    }

    async fn revoke_access_token(mut tool: ReqBox, auth: ReqAuthority, path: Path<(String, String)>) -> impl Responder {
        UserService::delete_access_token(&mut tool.db, auth.0, path.clone().0, path.into_inner().1)
    }
}

impl Handler for UserHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/users/", actix_web::web::get().to(Self::list));
        cfg.route("/users/", actix_web::web::post().to(Self::create));
        cfg.route("/users/{id}/", actix_web::web::get().to(Self::find));
        cfg.route("/users/{id}/", actix_web::web::patch().to(Self::update));
        cfg.route("/users/{id}/", actix_web::web::delete().to(Self::delete));
        cfg.route("/users/{id}/access_token/", actix_web::web::get().to(Self::list_access_token));
        cfg.route("/users/{id}/access_token/", actix_web::web::post().to(Self::create_access_token));
        cfg.route("/users/{id}/access_token/{token_id}/", actix_web::web::delete().to(Self::revoke_access_token));
    }
}