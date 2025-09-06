use vm::tree::Tree;


#[test]
fn test_init_tree() {
    let mut tree = Tree::new();
    tree.insert(vec!["port", "number"], "kcsjdn").expect("Test fail");
    assert_eq!(tree.get(vec!["port", "number"]), Some("kcsjdn".into()))
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
