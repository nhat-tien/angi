use std::{collections::HashMap, fmt::{self, Debug}};

#[derive(Clone)]
pub enum Tree<T> {
    Leaf(Option<T>),
    Branchs(HashMap<String, Tree<T>>),
}

impl<T> Tree<T>
where
    T: Clone + Debug,
{
    pub fn new() -> Self {
        Tree::default()
    }

    pub fn new_with_value(value: Option<T>) -> Self {
        Tree::Leaf(value)
    }

    pub fn get(&self, arr_of_key: Vec<&str>) -> Option<T> {
        let mut travel = self;

        if let Tree::Leaf(leaf) = travel {
            if arr_of_key.is_empty() {
                return leaf.clone();
            } else {
                return None;
            }
        };

        for key in arr_of_key {
            match travel {
                Tree::Leaf(_) => (),
                Tree::Branchs(branchs) => match branchs.get(key) {
                    Some(tree) => {
                        travel = tree;
                    }
                    None => {
                        return None;
                    }
                },
            }
        };

        if let Tree::Leaf(leaf) = travel {
            leaf.clone()
        } else {
            None
        }
    }

    pub fn insert(&mut self, arr_of_key: Vec<&str>, value: T) -> Result<(), TreeActionError> {
        let mut travel = self;
        let mut cursor = 0;
        while cursor < arr_of_key.len() {
            let key = arr_of_key[cursor];
            match travel {
                Tree::Leaf(_) => {
                    let mut hashmap = HashMap::new();
                    hashmap.insert(key.to_string(), Tree::Leaf(None));
                    *travel = Tree::Branchs(hashmap);
                }
                Tree::Branchs(branchs) => {
                    travel = branchs
                        .entry((*key).to_string())
                        .or_insert(Tree::Leaf(None));
                    cursor += 1;
                }
            };
        }

        if let Tree::Leaf(leaf) = travel {
            *leaf = Some(value.clone());
        };

        Ok(())
    }

    pub fn to_json(&self) -> String {
        self.traverse(0)
    }

    fn traverse(&self, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        let next_indent_str = "  ".repeat(indent + 1);

        match self {
            Tree::Leaf(Some(value)) => format!("{:?}", value),
            Tree::Leaf(None) => "null".to_string(),

            Tree::Branchs(map) => {
                if map.is_empty() {
                    return "{}".to_string();
                }
                let mut parts = Vec::new();

                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();

                for key in keys {
                    let value = &map[key];
                    let v = value.traverse(indent + 1);

                    parts.push(format!(
                        "{}\"{}\": {}",
                        next_indent_str, key, v
                    ));
                }

                format!(
                    "{{\n{}\n{}}}",
                    parts.join(",\n"),
                    indent_str
                )
            }
        }
    }
}

impl<T> fmt::Debug for Tree<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_pretty(f, 0)
    }
}


impl<T> Tree<T>
where
    T: fmt::Debug,
{
    fn fmt_pretty(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        let next_indent_str = "  ".repeat(indent + 1);

        match self {
            Tree::Leaf(Some(value)) => write!(f, "{:?}", value),
            Tree::Leaf(None) => write!(f, "null"),

            Tree::Branchs(map) => {
                if map.is_empty() {
                    return write!(f, "{{}}");
                }

                writeln!(f, "{{")?;

                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();

                for (i, key) in keys.iter().enumerate() {
                    let value = &map[*key];

                    write!(f, "{}\"{}\": ", next_indent_str, key)?;
                    value.fmt_pretty(f, indent + 1)?;

                    if i != keys.len() - 1 {
                        writeln!(f, ",")?;
                    } else {
                        writeln!(f)?;
                    }
                }

                write!(f, "{}}}", indent_str)
            }
        }
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree::Leaf(None)
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeActionError {
    pub error: String,
}

#[cfg(test)]
mod test {
    use crate::tree::Tree;

    #[test]
    fn test_init_tree() {
        let mut tree = Tree::new();
        tree.insert(vec!["port", "number"], "kcsjdn").expect("Test fail");
        assert_eq!(tree.get(vec!["port", "number"]), Some("kcsjdn"))
    }


    #[test]
    fn test_tree_get_the_wrong_path() {
        let mut tree = Tree::new();
        tree.insert(vec!["port", "number"], "kcsjdn").expect("Test fail");
        assert_eq!(tree.get(vec!["number"]), None)
    }

    #[test]
    fn test_tree_get_the_root() {
        let mut tree = Tree::new();
        tree.insert(vec!["port", "number"], "kcsjdn").expect("Test fail");
        assert_eq!(tree.get(vec!["port"]), None)
    }
}
