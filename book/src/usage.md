# Usage 



```rust
let vm = VM::new();
vm.load(bytecode);

let port = vm.eval("port");
let message = vm.eval("respone.message");
let result = vm.eval("respone.handler", params);
 ``` 

