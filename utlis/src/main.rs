use std::char;

use clap::{Parser, Subcommand};
use rand::{rngs::ThreadRng, Rng};
use yamd::lexer::TokenKind;

#[derive(Subcommand)]
enum Commands {
    /// Generate random tokens
    Random {
        /// length of a sequence
        length: usize,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Random { length } => random(length),
    }
}

fn random(length: usize) {
    let mut rng = rand::thread_rng();
    let t = TokenKind::Eol;
    match t {
        TokenKind::Terminator => 0,
        TokenKind::Eol => 1,
        TokenKind::LeftCurlyBrace => 2,
        TokenKind::RightCurlyBrace => 3,
        TokenKind::CollapsibleStart => 4,
        TokenKind::CollapsibleEnd => 5,
        TokenKind::Tilde => 6,
        TokenKind::Star => 7,
        TokenKind::Space => 8,
        TokenKind::Minus => 9,
        TokenKind::Hash => 10,
        TokenKind::GreaterThan => 11,
        TokenKind::Bang => 12,
        TokenKind::Backtick => 13,
        TokenKind::Plus => 14,
        TokenKind::LeftSquareBracket => 15,
        TokenKind::RightSquareBracket => 16,
        TokenKind::LeftParenthesis => 17,
        TokenKind::RightParenthesis => 18,
        TokenKind::Underscore => 19,
        TokenKind::Pipe => 20,
        TokenKind::Literal => 21,
    };

    for _ in 0..length {
        let token_kind = match rng.gen_range(0..=21) {
            0 => "\n\n",
            1 => "\n",
            2 => "{",
            3 => "}",
            4 => "{%",
            5 => "%}",
            6 => "~",
            7 => "*",
            8 => " ",
            9 => "-",
            10 => "#",
            11 => ">",
            12 => "!",
            13 => "`",
            14 => "+",
            15 => "[",
            16 => "]",
            17 => "(",
            18 => ")",
            19 => "_",
            20 => "|",
            _ => &random_string(rng.gen_range(10..100), &mut rng),
        };
        print!("{token_kind}")
    }
    println!()
}

fn random_char(rng: &mut ThreadRng) -> char {
    match char::from_u32(rng.gen()) {
        Some(c) => c,
        None => random_char(rng),
    }
}

fn random_string(len: usize, rng: &mut ThreadRng) -> String {
    let mut str = String::with_capacity(len);
    for _ in 0..len {
        str.push(random_char(rng));
    }
    str
}
