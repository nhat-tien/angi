use std::{collections::HashMap};

use angi_runtime::{tree::Tree, value::{Function, Table, Value}, vm::VM};
use axum::{Json, body::Body, extract::{Path, Query, Request}, http::{HeaderMap, StatusCode}, response::{Html, IntoResponse, Response}, routing::{any, get, post}};

// use crate::Avm;

fn build_request_value(
        path: HashMap<String, String>,
        query: HashMap<String, String>,
        headers: HeaderMap,
        body: Option<serde_json::Value>,
) -> Value {
    let mut root = Tree::new();

    for (k, v) in path {
        println!("{}", k);
        root.insert(vec!["path", &k], Value::String(v)).unwrap();
    }

    for (k, v) in query {
        root.insert(vec!["query", &k], Value::String(v)).unwrap();
    }

    for (k, v) in headers.iter() {
        if let Ok(val) = v.to_str() {
            root.insert(
                vec!["headers", k.as_str()],
                Value::String(val.to_string()),
            ).unwrap();
        }
    }

    match body {
        Some(b) => {
            insert_json(&mut root, vec!["body"], b);
        }
        None => {
            root.insert(vec!["body"], Value::None).unwrap();
        }
    }

    Value::Table(Box::new(root))
}

fn insert_json(
    tree: &mut Tree<Value>,
    path: Vec<&str>,
    value: serde_json::Value,
) {
    match value {
        serde_json::Value::Null => {
            tree.insert(path, Value::None).unwrap();
        }

        serde_json::Value::Bool(b) => {
            tree.insert(path, Value::Bool(b)).unwrap();
        }

        serde_json::Value::Number(n) => {
            tree.insert(path, Value::Int(n.as_i64().unwrap_or(0))).unwrap();
        }

        serde_json::Value::String(s) => {
            tree.insert(path, Value::String(s)).unwrap();
        }

        serde_json::Value::Array(arr) => {
            for (i, v) in arr.into_iter().enumerate() {
                let mut new_path = path.clone();
                new_path.push(Box::leak(i.to_string().into_boxed_str()));
                insert_json(tree, new_path, v);
            }
        }

        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let mut new_path = path.clone();
                new_path.push(Box::leak(k.into_boxed_str()));
                insert_json(tree, new_path, v);
            }
        }
    }
}

pub fn make_vm_handler(method: &str, function: Function, avm: VM) -> axum::routing::MethodRouter {
    let handler = move |
        Path(path): Path<HashMap<String, String>>,
        Query(query): Query<HashMap<String, String>>,
        req: Request<Body>,
    | {
        let mut vm = avm.clone();
        let function = function.clone();

        async move {

            let headers = req.headers().clone();

            let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
                .await
                .unwrap_or_default();

            let body_json = serde_json::from_slice(&body_bytes).ok();

            // let mut ready_vm = vm.lock().unwrap();
            let input = build_request_value(
                path,
                query,
                headers,
                body_json,
            );

            let result = function.call::<Table, _>(&mut vm, (input,));

            println!("{:?}", result);

            match result {
                Ok(val) => table_to_response(val, vm.clone()),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("VM Error: {:?}", e),
                ).into_response(),
            }
        }
    };

    match method {
        "GET" => get(handler),
        "POST" => post(handler),
        _ => any(handler),
    }
}

// pub fn make_vm_handler(function: Function, avm: Avm) -> axum::routing::MethodRouter {
//     axum::routing::any(move |
//         path: axum::extract::Path<HashMap<String, String>>,
//         Query(query): Query<HashMap<String, String>>,
//         headers: HeaderMap,
//         Json(body): Json<serde_json::Value>,
//     | {
//         let vm = avm.clone();
//         let function = function.clone();
//
//         async move {
//             let vm_for_response = vm.clone();
//             let mut ready_vm = vm.lock().unwrap();
//
//             // let body_value = body.map(|Json(v)| v);
//
//             let input = build_request_value(
//                 path.0,
//                 query,
//                 headers,
//                 body,
//             );
//
//             let result = function.call::<Table, _>(&mut ready_vm, (input,));
//
//             match result {
//                 Ok(val) => table_to_response(val, vm_for_response),
//                 Err(e) => (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     format!("VM Error: {:?}", e),
//                 ).into_response(),
//             }
//         }
//     })
// }

// fn json_to_value(v: serde_json::Value) -> Value {
//     match v {
//         serde_json::Value::Null => Value::None,
//         serde_json::Value::Bool(b) => Value::Bool(b),
//         serde_json::Value::Number(n) => {
//             Value::Int(n.as_i64().unwrap_or(0))
//         }
//         serde_json::Value::String(s) => Value::String(s),
//         serde_json::Value::Array(arr) => {
//             Value::List(arr.into_iter().map(json_to_value).collect())
//         }
//         serde_json::Value::Object(map) => {
//             let mut tree = Tree::new();
//             for (k, v) in map {
//                 tree.insert(vec![&k], json_to_value(v));
//             }
//             Value::Table(Box::new(tree))
//         }
//     }
// }

pub fn table_to_response(table: Table, mut vm: VM) -> Response {
        // let mut ready_vm = vm.lock().unwrap();
        let type_of_handler = table.get::<String>("type").unwrap();

        match type_of_handler.as_str() {
            "html" => {
                let html = table.get::<String>("html").unwrap();
                return (StatusCode::from_u16(200).unwrap(), Html(html)).into_response();
            },
            "htmlTemplate" => {
                let path_template = table.get::<String>("path").unwrap();
                let html = std::fs::read_to_string(&path_template)
                .unwrap_or_else(|_| "<h1>Template not found</h1>".to_string());
                return (StatusCode::from_u16(200).unwrap(), Html(html)).into_response();

            },
            "json" => {
                let mut json = table.get_value("body").unwrap();
                json.resolve_thunk(&mut vm).unwrap();
                return (StatusCode::from_u16(200).unwrap(), Json(json)).into_response();
            },
            _ => todo!()
        }

}

// fn map_to_table(map: HashMap<String, String>) -> Value {
//     let mut table = Table::new();
//     for (k, v) in map {
//         table.insert(k, Value::String(v));
//     }
//     Value::Table(table)
// }
