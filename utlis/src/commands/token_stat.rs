use std::{collections::HashMap, fs, path::PathBuf};

use yamd::lexer::{Lexer, TokenKind};

pub fn token_stat(path: PathBuf) {
    let str = fs::read_to_string(&path).expect("Should have been able to read the file");
    let l = Lexer::new(str.as_ref());
    let stat = l.fold(HashMap::<TokenKind, usize>::new(), |mut acc, t| {
        acc.entry(t.kind)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        acc
    });

    let total: usize = stat.values().sum();

    let mut table: Vec<(String, usize)> = stat
        .iter()
        .map(|(kind, count)| (format!("{:?}", kind), *count))
        .collect();

    println!("\nToken statistics for: {:?}\n", path);
    println!("Total token count: {total}\n");

    table.sort_by(|(_, left), (_, right)| right.cmp(left));
    table
        .iter()
        .for_each(|(kind, count)| println!("{}:{}{}", kind, tab(kind.as_ref()), count));
}

fn tab(str: &str) -> String {
    " ".repeat(20_usize.checked_sub(str.len()).unwrap_or(1))
}
