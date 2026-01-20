use std::sync::{Arc, Mutex};

use axum::{
    Router,
    response::Html,
    routing::get,
    Json,
};
use vm::value::{Function, List, Table};
use vm::{error::VmError, vm::VM};

type Avm = Arc<Mutex<VM>>;

#[tokio::main]
async fn main() -> Result<(), VmError> {
    let mut vm = VM::new_from_itself().map_err(|e| panic!("Cant initialize the vm {:?}",e))?;

    let port = vm.eval::<i64>("port")?;

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    let avm = Arc::new(Mutex::new(vm));


    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(avm)).await.unwrap();

    Ok(())
}

fn app(vm: Avm) -> Router {
    build_router(vm).expect("Error in build router")
}

fn build_router(vm: Avm) -> Result<Router, VmError> {
    let mut ready_vm = vm.lock().unwrap();
    let mut list_routes = ready_vm.eval::<List<Table>>("routes")?;
    list_routes.force(&mut ready_vm);
    let list_routes_iter = list_routes.iter().unwrap();

    Ok(list_routes_iter.fold(Router::new(), |router, route| {
        let path = route.get::<String>("path").unwrap();
        let function = route.get::<Function>("handler").unwrap();
        let result: Table = function.call(&mut ready_vm, ()).unwrap();

        let type_of_handler = result.get::<String>("type").unwrap();

        match type_of_handler.as_str() {
            "html" => {
                let html = result.get::<String>("html").unwrap();
                router.route(&path, make_html_handler(html))
            },
            "htmlTemplate" => {
                let path_template = result.get::<String>("path").unwrap();
                router.route(&path, make_html_template_handler(path_template))
            },
            "json" => {
                let json = result.get::<String>("body").unwrap();
                router.route(&path, make_json_handler(json))
            },
            _ => todo!()
        }
    }))
}

fn make_html_handler(html: String) -> axum::routing::MethodRouter {
    get(move || {
        async move { Html(html.clone()) }
    })
}

fn make_html_template_handler(path: String) -> axum::routing::MethodRouter {
    get(move || {
        async move { Html(path.clone()) }
    })
}

fn make_json_handler(json: String) -> axum::routing::MethodRouter {
    let v: serde_json::Value = serde_json::from_str(&json).map_err(|e| e.to_string()).unwrap();
    get(move || {
        async move { Json(v) }
    })
}
