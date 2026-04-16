use ariadne::{Color,  ReportKind};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

impl Severity {
    fn get_report_kind(&self) -> ReportKind<'_> {
        match self {
            Severity::Error => ReportKind::Error,
            Severity::Warning => ReportKind::Warning,
            Severity::Note => ReportKind::Advice,
        }
    }

    fn get_color(&self) -> Color {
        match self {
            Severity::Error => Color::Red,
            Severity::Warning => Color::Yellow,
            Severity::Note => Color::Blue,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    /// Convert to a byte range for ariadne (needs 0-based indexing)
    pub fn to_range(&self, line_starts: &[usize], length: usize) -> Range<usize> {
        let line_idx = self.line.saturating_sub(1);
        let line_start = line_starts.get(line_idx).copied().unwrap_or(0);
        let start = line_start.saturating_add(self.column);
        let end = start.saturating_add(length);
        start..end
    }
}

pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub span_len: usize,  // length of the highlighted span
    pub help: Option<String>,
    pub notes: Vec<String>,
}

pub struct DiagnosticEngine {
    pub diagnostics: Vec<Diagnostic>,
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

    /// Emit all diagnostics using ariadne for beautiful output
    pub fn emit(&self, source: &str, filename: &str) {
        if self.diagnostics.is_empty() {
            return;
        }

        let line_starts = compute_line_starts(source);

        for diagnostic in &self.diagnostics {
            let span_range = diagnostic.span.to_range(&line_starts, diagnostic.span_len);

            let mut report = ariadne::Report::build(
                diagnostic.severity.get_report_kind(),
                (filename, span_range.clone()),
            )
            .with_message(&diagnostic.message)
            .with_label(
                ariadne::Label::new((filename, span_range.clone()))
                    .with_message(&diagnostic.message)
                    .with_color(diagnostic.severity.get_color()),
            );

            if let Some(help) = &diagnostic.help {
                report = report.with_note(help);
            }

            for note in &diagnostic.notes {
                report = report.with_note(note);
            }

            report
                .finish()
                .print((filename, ariadne::Source::from(source)))
                .unwrap();
        }
    }
}

impl Default for DiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute line start positions for a source string (0-based byte offsets)
fn compute_line_starts(source: &str) -> Vec<usize> {
    let mut starts = Vec::new();
    starts.push(0);

    for (i, &b) in source.as_bytes().iter().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }

    starts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_simple() {
        let source = "fn main() {\n    println!(\"Hello\");\n}";
        let mut engine = DiagnosticEngine::new();
        engine.report(Diagnostic {
            severity: Severity::Error,
            message: "test error".to_string(),
            span: Span { line: 2, column: 4 },
            span_len: 5,
            help: Some("test help".to_string()),
            notes: vec!["note1".to_string()],
        });
        engine.emit(source, "test");
    }

    #[test]
    fn test_line_starts() {
        let source = "line1\nline2\nline3";
        let starts = compute_line_starts(source);
        assert_eq!(starts, vec![0, 6, 12]);
    }

    #[test]
    fn test_span_to_range() {
        let source = "fn main() {\n    println!(\"Hello\");\n}";
        let line_starts = compute_line_starts(source);

        // Span at line 2, column 4, length 5
        println!("{:?}", line_starts);
        let span = Span { line: 2, column: 4 };
        let range = span.to_range(&line_starts, 5);
        assert_eq!(range, 16..21);
    }
}
