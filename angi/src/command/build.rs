use std::fs::{self, File, set_permissions};
use std::io::{self, Write};
use std::path::Path;

static SERVER: &[u8] = include_bytes!(env!("RUNTIME_PATH"));

use crate::compiler::{
    code_gen::BytecodeGen,
    lexer::Lexer,
    optimization::optimization,
    parser::parse,
};

pub fn index(args: &[String]) {
    let source_file_path = &args[2];
    let dist_file_path = &args[3];

    let content = match fs::read_to_string(source_file_path) {
        Ok(content) => content,
        Err(err) => panic!("Cannot open file {err:?}"),
    };

    let mut lexer = Lexer::new(content.chars());

    let mut ast = match parse(&mut lexer) {
        Ok(ast) => ast,
        Err(err) => panic!("Err in parse {err:?}"),
    };

    let mut bytecode_genaration = BytecodeGen::new();

    optimization(&mut ast);

    let content = bytecode_genaration.get_binary(ast);

    let mut file = match File::options()
        .create(true)
        .append(true)
        .open(dist_file_path)
    {
        Ok(file) => file,
        Err(err) => panic!("Err in open file {}", err),
    };

    file.write_all(SERVER).expect("Fail to write file");
    file.write_all(&content).expect("Fail to write file");
    file.flush().expect("Fail to flush");

    let path = Path::new(&dist_file_path);
    make_file_executable(path).expect("Fail to make file executable");
}


fn make_file_executable(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        set_permissions(path, perms)?;
    }

    #[cfg(windows)]
    {
        if path.extension().is_none() {
            eprintln!("⚠️  On Windows, consider naming the file with `.exe`");
        }
    }

    Ok(())
}
