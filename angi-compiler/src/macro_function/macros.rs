#[macro_export]
macro_rules! register_macros {
    ($($name:expr => $func:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert($name.to_string(), $func as MacroFn);
        )*
        map
    }};
}
