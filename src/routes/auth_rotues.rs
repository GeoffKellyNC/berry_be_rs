use actix_web::web;

use crate::controllers;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/login")
                    .route(web::post().to(controllers::auth::login::login_twitch)),
            )
            // .service(
            //     web::resource("/logout")
            //         .route(web::post().to(controllers::auth::logout::logout)),
            // ),
    );
}
