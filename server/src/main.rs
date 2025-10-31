use std::sync::{Arc, Mutex};

use axum::{
    Router, extract::State, http::StatusCode, response::Html, routing::get,
};
use vm::value::{List, Table};
use vm::{error::VmError, vm::VM};

type Avm = Arc<Mutex<VM>>;

#[tokio::main]
async fn main() -> Result<(), VmError> {
    let vm = VM::new_from_itself().map_err(|_| panic!("Cant initialize the vm"))?;

    let avm = Arc::new(Mutex::new(vm));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(avm)).await.unwrap();

    Ok(())
}

fn app(vm: Avm) -> Router {
    build_router(vm).expect("Error in build router")
}

async fn handler(State(vm): State<Avm>) -> Result<Html<String>, (StatusCode, String)> {
    let mut new_vm = vm.lock().unwrap();

    let port = new_vm
        .eval::<i64>("port")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "csknjk".to_string()))?;

    let message = new_vm
        .eval::<String>("response.handler.response")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "csknjk".to_string()))?;

    let string = format!(
        "<h1>Hello, World!</h1>
                        <p>{port}</p><strong>{message}</strong"
    );
    Ok(Html(string))
}
fn build_router(vm: Avm) -> Result<Router, VmError> {
    let mut ready_vm = vm.lock().unwrap();
    let list_routes = ready_vm.eval::<List<Table>>("routes")?;

    Ok(list_routes.iter(&mut ready_vm).fold(Router::new(), |router, route| {
        let path = route.get::<String>("path").unwrap();
        let message = route.get::<String>("handler").unwrap();

        router.route(&path, get(|| async { message }))
    }))
}
