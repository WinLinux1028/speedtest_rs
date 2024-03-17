#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::{alloc::Layout, collections::HashMap, env};

use axum::{
    extract::{DefaultBodyLimit, Query, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing, Router,
};
use rand::RngCore;
use tokio::net::TcpListener;

type Error = Box<dyn std::error::Error>;

static SIZE_LIMIT: usize = 100 * 1024 * 1024;

#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next();

    let app = Router::new()
        .route("/", routing::get(download))
        .route("/", routing::post(upload))
        .layer(DefaultBodyLimit::max(SIZE_LIMIT));

    let listener = TcpListener::bind(args.next().unwrap()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn download(query: Query<HashMap<String, String>>) -> Response {
    match download_(query).await {
        Ok(o) => o,
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}

async fn download_(Query(query): Query<HashMap<String, String>>) -> Result<Response, Error> {
    let size: usize = query.get("size").ok_or("")?.parse()?;
    if size > SIZE_LIMIT {
        return Err("".into());
    }

    let mut buf = just_alloc(size)?;
    rand::thread_rng().fill_bytes(&mut buf);

    let mut response = buf.into_response();

    let headers = response.headers_mut();
    headers.insert("Cache-Control", "no-store".parse()?);

    Ok(response)
}

async fn upload(req: Request) -> Response {
    match upload_(req).await {
        Ok(o) => o,
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}

async fn upload_(req: Request) -> Result<Response, Error> {
    axum::body::to_bytes(req.into_body(), SIZE_LIMIT).await?;
    let mut response = ().into_response();

    let headers = response.headers_mut();
    headers.insert("Cache-Control", "no-store".parse()?);

    Ok(response)
}

fn just_alloc(size: usize) -> Result<Vec<u8>, Error> {
    unsafe {
        let buf = std::alloc::alloc(Layout::array::<u8>(size)?);
        let buf = Vec::from_raw_parts(buf, size, size);
        Ok(buf)
    }
}
