#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

pub struct Span {
    pub line: usize,
    pub column: usize,
}

pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub help: Option<String>,
}

pub struct DiagnosticEngine {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticEngine {

    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn report(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_error(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }

    pub fn emit(&self, source: &str) {
        let lines: Vec<&str> = source.lines().collect();

        for d in &self.diagnostics {

            let line = d.span.line;
            let column = d.span.column;

            println!("{:?}: {}", d.severity, d.message);
            println!(" --> {}:{}", line, column);
            println!("  |");

            if let Some(code_line) = lines.get(line - 1) {
                println!("{:>3} | {}", line, code_line);

                let mut pointer = String::new();
                for _ in 0..(column + 3) {
                    pointer.push(' ');
                }
                pointer.push('^');

                println!("  |{}", pointer);
            }

            if let Some(help) = &d.help {
                println!("  = help: {}", help);
            }

            println!();
        }
    }

}
