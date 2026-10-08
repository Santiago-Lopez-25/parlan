//! Error reporting system of parlan

#[derive(Debug, Clone, Copy)]
pub struct Span(pub usize, pub usize);

#[derive(Debug)]
pub struct SourceFile<'sf> {
    name: &'sf str,
    src: &'sf str,
    line_starts: Vec<usize>
}

impl<'sf> SourceFile<'sf> {
    pub fn new(name: &'sf str, src: &'sf str) -> Self {
        let mut line_starts = vec![0];
        for (i, byte) in src.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self { name, src, line_starts }
    }

    pub fn location(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.src.len());
        let line = match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };
        let line_start = self.line_starts[line];
        let col = self.src[line_start..offset].chars().count();
        (line + 1, col + 1)
    }

    pub fn get_line(&self, line: usize) -> &str {
        let start = self.line_starts[line - 1];
        let end = self.line_starts.get(line)
            .map(|&idx| if idx > 0 && self.src.as_bytes()[idx - 1] == b'\n' { idx - 1 } else { idx })
            .unwrap_or(self.src.len());
        &self.src[start..end]
    }
}

pub enum ErrorLevel {
    Error,
    #[allow(unused)]
    Warning
}

pub struct Diagnostic {
    pub level: ErrorLevel,
    pub msg: String,
    pub span: Span,
    pub label_msg: String,
}

impl Diagnostic {
    pub fn emit(&self, file: &SourceFile) {
        let color = match self.level {
            ErrorLevel::Error => "\x1b[1;31m",
            ErrorLevel::Warning => "\x1b[93m"
        };
        
        let level_str = match self.level {
            ErrorLevel::Error => "\x1b[1;31merror at\x1b[0m",
            ErrorLevel::Warning => "\x1b[93mwarning at\x1b[0m"
        };

        let (line, col) = file.location(self.span.0);
        eprintln!(
            "{} {}:{}:{}: \x1b[1;97m{}\x1b[0m", 
            level_str, 
            file.name, 
            line, 
            col, 
            self.msg
        );

        let padding = " ".repeat(line.to_string().len());
        eprintln!("{} \x1b[1;96m|\x1b[0m", padding);
        eprintln!("\x1b[1;96m{} |\x1b[0m {}", line, file.get_line(line));

        let span_len = (self.span.1 - self.span.0).max(1);
        eprintln!(
            "{} \x1b[1;96m|\x1b[0m {}{}{} {}\x1b[0m",
            padding,
            " ".repeat(col - 1),
            color,
            "^".repeat(span_len as usize),
            self.label_msg 
        );

        eprintln!("{} \x1b[1;96m|\x1b[0m \n", padding);
    }
}

/// 
#[macro_export]
macro_rules! error {
    ($span:expr, $label_msg:expr, $($args:tt)+) => {
        crate::error::Diagnostic {
            level: crate::error::ErrorLevel::Error,
            msg: format_args!($($args)+).to_string(),
            span: $span,
            label_msg: $label_msg
        }
    };
}