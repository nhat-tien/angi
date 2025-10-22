use std::sync::{Arc, Mutex};

use axum::{extract::State, handler::Handler, http::StatusCode, response::Html, routing::get, Router};
use vm::{error::RuntimeError, value::Value, vm::VM};

type Avm = Arc<Mutex<VM>>;

#[tokio::main]
async fn main() -> Result<(), RuntimeError>{
    let vm = match VM::new() {
        Ok(vm) => vm,
        Err(err) => panic!("{}", err.message)
    };
    let avm = Arc::new(Mutex::new(vm));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();


    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(avm)).await.unwrap();

    Ok(())
}

fn app(vm: Avm) -> Router {

    Router::new()
        .route("/", get(handler))
        .with_state(vm)
}

async fn handler(
    State(vm): State<Avm>
) -> Result<Html<String>, (StatusCode, String)> {
    let mut new_vm = vm.lock().unwrap();

    let port = match new_vm.eval_table("port") {
        Ok(Value::Int(int)) => { int },
        _ => return Err(( StatusCode::INTERNAL_SERVER_ERROR, "csknjk".to_string() ))
    };

    let message = match new_vm.eval_table("response.handler.response") {
        Ok(Value::String(str)) => { str },
        _ => return Err(( StatusCode::INTERNAL_SERVER_ERROR, "csknjk".to_string() ))
    };

    let string = format!("<h1>Hello, World!</h1><p>{port}</p><strong>{message}</strong");

    Ok(Html(string))
}

fn make_handler() -> Handler {
    return || {

    }
}
