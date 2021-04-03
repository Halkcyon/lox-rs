use std::{
    cmp::Ordering,
    env,
    fmt,
    fs,
    io::{BufRead, BufReader, Write, stdin, stdout},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut lox: Lox = Default::default();

    match args.len().cmp(&1) {
        Ordering::Less => lox.run_prompt(),
        Ordering::Equal => lox.run_file(&args[0]),
        Ordering::Greater => {
            eprintln!("Usage: lox [script]");
            process::exit(64);
        },
    };
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32
}

impl Scanner {
    fn new(s: &str) -> Self {
        Self {
            source: s.to_string(),
            ..Default::default()
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenKind::Eof, "".into(), self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as _
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenKind::LParen),
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current).unwrap()
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

#[derive(Clone, Debug)]
struct Token {
    kind: TokenKind,
    lexeme: String,
    line: u32,
}

impl Token {
    fn new(kind: TokenKind, lexeme: String, line: u32) -> Self {
        Self {
            kind,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.kind, self.lexeme)
    }
}

#[derive(Clone, Debug)]
enum TokenKind {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Period,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEq,
    Eq,
    EqEq,
    Gt,
    GtEq,
    Lt,
    LtEq,

    Ident,
    String(String),
    Number(f64),

    And,
    Class,
    Else,
    False,
    Funct,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Default)]
struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_prompt(&mut self) {
        let stdin = stdin();
        let mut reader = BufReader::new(stdin);

        let mut input = String::new();
        let mut out = stdout();
        loop {
            print!("> ");
            out.flush().ok();

            reader.read_line(&mut input).ok();
            if input.is_empty() {
                break;
            }

            self.run(&input);
            self.had_error = false;

            input.clear();
        }
    }

    fn run_file<P: AsRef<Path>>(&self, path: P) {
        let source = fs::read_to_string(path).unwrap();
        self.run(&source);

        if self.had_error {
            process::exit(65);
        }
    }

    fn run(&self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }

    fn error(&mut self, line: u32, message: String) {
        self.report(line, "".into(), message);
    }

    fn report(&mut self, line: u32, where_: String, message: String) {
        println!("[line {}] Error{}: {}", line, where_, message);

        self.had_error = true;
    }
}
