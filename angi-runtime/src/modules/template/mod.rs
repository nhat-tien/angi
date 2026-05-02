use crate::tree::Tree;
use std::{collections::HashMap, fmt::Debug};
use serde::Serialize;
use crate::value::Value;
use minijinja::value::Value as MiniValue;
use minijinja::Environment;
use std::fs;

use crate::modules::ForeignFnError;

pub fn render_fn(args: Vec<Value>) -> Result<Value, ForeignFnError> {
    if args.len() != 2 {
        return Err(ForeignFnError::Unexpected {
            message: "render expects (Table, path)".into(),
        });
    }

    let table = match &args[0] {
        Value::Table(t) => t,
        _ => {
            return Err(ForeignFnError::Unexpected {
                message: "First arg must be Table".into(),
            })
        }
    };

    let path = match &args[1] {
        Value::String(s) => s,
        _ => {
            return Err(ForeignFnError::Unexpected {
                message: "Second arg must be string".into(),
            })
        }
    };

    let template_str = fs::read_to_string(path)
        .map_err(|e| ForeignFnError::Unexpected { message: e.to_string(),})?;

    let env = Environment::new();

    let tmpl = env
        .template_from_str(&template_str)
        .map_err(|e| ForeignFnError::Unexpected { message: e.to_string(),})?;

    let ctx = tree_to_minijinja(table);

    let rendered = tmpl
        .render(ctx)
        .map_err(|e| ForeignFnError::Unexpected { message: e.to_string(),})?;


    let mut tree = Tree::new();
    tree.insert(vec!["type"], Value::String("html".into())).expect("Cant insert attribute to tree in template");
    tree.insert(vec!["html"], Value::String(rendered)).expect("Cant insert attribute to tree in template");

    Ok(Value::Table(Box::new(tree)))
}

fn tree_to_minijinja<T>(tree: &Tree<T>) -> MiniValue
where
    T: Clone + Debug + Serialize,
{
    match tree {
        Tree::Leaf(Some(v)) => MiniValue::from_serialize(v),
        Tree::Leaf(None) => MiniValue::from(()),

        Tree::Branchs(map) => {
            let mut new_map = HashMap::new();

            for (k, v) in map {
                new_map.insert(k.clone(), tree_to_minijinja(v));
            }

            MiniValue::from(new_map)
        }
    }
}

