use angi::compiler::compile;
use vm::{value::{List, Table}, vm::VM};


#[test]
fn compiler_test_get_value_from_table() {
    let bytecode = compile(r#"
{
    port = 3030;
    routes = [
       {
           path = "/";
           handler = "Message 1";
       },
       {
           path = "/Hello";
           handler = "Message Hello";
       }
    ];
}
    "#.into()).unwrap();

    let mut vm = VM::new_from_bytes(bytecode).unwrap();

    let port = vm.eval::<i64>("port").unwrap();
    assert_eq!(port, 3030);

    let mut routes = vm.eval::<List<Table>>("routes").unwrap();

    routes.force(&mut vm);

    let first = routes.get(0).unwrap();

    assert_eq!(first.get::<String>("path"), Some(String::from("/")));
    assert_eq!(first.get::<String>("handler"), Some(String::from("Message 1")));


    let second = routes.get(1).unwrap();

    assert_eq!(second.get::<String>("path"), Some(String::from("/Hello")));
    assert_eq!(second.get::<String>("handler"), Some(String::from("Message Hello")));
}

