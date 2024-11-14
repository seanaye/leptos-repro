use app::{shell, App};
use axum::{debug_handler, extract::{FromRef, Host, Query, State}, http::{header::SET_COOKIE, HeaderValue, StatusCode}, response::{IntoResponse, IntoResponseParts, Redirect}, routing::get, Router};
use axum_extra::extract::{self, cookie::{Cookie, SameSite}, CookieJar};
use leptos::prelude::*;
use leptos_axum::{extract, generate_route_list, redirect, LeptosRoutes, ResponseOptions};
use service::AppState;
use sqlx::{Pool, Sqlite, SqlitePool};
use thiserror::Error;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    dotenv::dotenv().ok();

    let pool: SqlitePool = sqlx::Pool::connect("test.db").await.expect("Failed to connect to database");

    let state = AppState { leptos_options, pool };

    let app = Router::new()
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let state = state.clone();
                move || {
                    dbg!(Owner::current());
                    dbg!("This should be called first");
                    provide_context(state.clone());
                }
            },
            {
                let leptos_options = state.leptos_options.clone();
                move || shell(leptos_options.clone())
            }
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
