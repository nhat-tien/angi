use std::collections::HashMap;


#[derive(Default, Debug)]
pub struct Tree {
    pub value: Option<String>,
    pub childrens: HashMap<String, Tree>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            value: None,
            childrens: HashMap::new(),
        }
    }

    pub fn get(&self, arr_of_key: Vec<&str>) -> Option<String> {
        let mut travel = self;

        for key in arr_of_key {
            match travel.childrens.get(key) {
                Some(tree) => {
                    travel = tree;
                }
                None => return None,
            };
        }
        travel.value.clone()
    }

    pub fn insert(
        &mut self,
        arr_of_key: Vec<&str>,
        value: &str,
    ) -> Result<(), TreeActionError> {

        let mut travel = self;
        let last_index_of_arr_of_key = arr_of_key.len() - 1;

        for (i, key) in arr_of_key.iter().enumerate() {
            travel = travel.childrens.entry((*key).to_string()).or_insert(Tree {
                value: None,
                childrens: HashMap::new()
            });
            if i == last_index_of_arr_of_key {
                travel.value = Some(value.to_string());
            };
        };

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeActionError {
    pub error: String,
}
