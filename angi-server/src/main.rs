use std::sync::{Arc, Mutex};

mod logger;
mod utils;

use axum::extract::{Path};
use axum::http::{StatusCode, header};
use axum::middleware;
use axum::response::Response;
use axum::{
    Router,
    response::Html,
    routing::get,
    Json,
};
use angi_runtime::value::{Function, List, Table, Value};
use angi_runtime::{error::VmError, vm::VM};
use angi_archive::{Extractor, StaticStore};
use colored::Colorize;
use tower_http::services::ServeDir;

use crate::utils::make_vm_handler;

type Avm = Arc<Mutex<VM>>;
type ArcStore = Arc<StaticStore>;

#[tokio::main]
async fn main() -> Result<(), VmError> {
    logger::info("Initializing the runtime...");

    let extractor = Extractor::init_from_itself().map_err(|e| {
        logger::error("Can't Initialize the extractor");
        panic!("Cant initialize the extractor {:?}",e)
    })?;

    let mut vm = VM::new_from_extractor(&extractor).map_err(|e| {
        logger::error("Can't Initialize the runtime");
        panic!("Cant initialize the vm {:?}",e)
    })?;

    logger::info("Initialize the runtime successful");

    logger::info("Start the server");

    let port = vm.eval::<i64>("port")?;

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    // let avm = Arc::new(Mutex::new(vm));
    let static_store = Arc::new(StaticStore::new(extractor));

    logger::log_startup(
        "My App",
        "0.1.0",
        port
    );

    logger::info(format!(
        "{} {}",
        "Listening on".bold(),
        listener.local_addr().unwrap()
    ));

    axum::serve(listener, app(vm, static_store)?).await.unwrap();

    Ok(())
}

#[allow(unused_variables)]
fn app(vm: VM, static_store: ArcStore) -> Result<Router, VmError> {

    // let mut ready_vm = vm.lock().unwrap();
    let mut ready_vm = vm.clone();

    let static_config = ready_vm.eval::<Table>("static").unwrap();

    // let prefix = static_config.get::<String>("prefix").unwrap();

    let dir = static_config.get::<String>("dir").unwrap();

    drop(ready_vm);

    Ok(build_router(vm.clone())
        .expect("Error in build router")
        // Static
        // .route("/static/{*path}", static_handler(static_store))
        // .nest_service(&prefix, ServeDir::new(dir))
        .fallback_service(ServeDir::new(dir))
        .layer(middleware::from_fn(logger::request_logger))
    )
}

fn build_router(mut vm: VM) -> Result<Router, VmError> {
    // let mut ready_vm = vm.lock().unwrap();
    let mut list_routes = vm.eval::<List<Table>>("routes")?;
    list_routes.force(&mut vm);
    let list_routes_iter = list_routes.iter().unwrap();

    Ok(list_routes_iter.fold(Router::new(), |router, route| {
        let path = route.get::<String>("path").unwrap();
        let function = route.get::<Function>("handler").unwrap();
        let method = route.get::<String>("method").unwrap();
        // let result: Table = function.call(&mut ready_vm, ()).unwrap();

        // let type_of_handler = result.get::<String>("type").unwrap();
        println!("Hello");
        router.route(&path, make_vm_handler(&method, function, vm.clone()))

        // match type_of_handler.as_str() {
        //     "html" => {
        //         let html = result.get::<String>("html").unwrap();
        //         router.route(&path, make_html_handler(html))
        //     },
        //     "htmlTemplate" => {
        //         let path_template = result.get::<String>("path").unwrap();
        //         let html = std::fs::read_to_string(&path_template)
        //         .unwrap_or_else(|_| "<h1>Template not found</h1>".to_string());
        //
        //         router.route(&path, make_html_handler(html))
        //     },
        //     "json" => {
        //         let mut json = result.get_value("body").unwrap();
        //         json.resolve_thunk(&mut ready_vm).unwrap();
        //         router.route(&path, make_json_handler_v2(json))
        //     },
        //     _ => todo!()
        // }
    }))
}

#[allow(dead_code)]
fn make_html_handler(html: String) -> axum::routing::MethodRouter {
    get(move || {
        async move { Html(html.clone()) }
    })
}

#[allow(dead_code)]
fn make_html_template_handler(path: String) -> axum::routing::MethodRouter {
    get(move || {
        async move { Html(path.clone()) }
    })
}

// fn make_json_handler(json: String) -> axum::routing::MethodRouter {
//     let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string()).unwrap();
//     get(move || {
//         async move { Json(v) }
//     })
// }

#[allow(dead_code)]
fn make_json_handler_v2(json: Value) -> axum::routing::MethodRouter {
    // let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string()).unwrap();
    get(move || {
        async move { Json(json) }
    })
}



#[allow(dead_code)]
fn static_handler(store: Arc<StaticStore>) -> axum::routing::MethodRouter {
    axum::routing::get(move |Path(path): Path<String>| {
        let store = store.clone();

        async move {
            let key = format!("static/{}", path);

            if let Some(bytes) = store.get(&key) {
                let mime = mime_guess::from_path(&path).first_or_octet_stream();

                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(axum::body::Body::from(bytes))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body("Not Found".into())
                    .unwrap()
            }
        }
    })
}
