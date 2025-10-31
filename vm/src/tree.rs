use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
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
