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

    println!(
        "\n{}\n",
        tab(
            "Token statistics for",
            path.to_str().expect("path to be valid unicode")
        )
    );
    println!("{}\n", tab("Total token count", total.to_string().as_str()));

    table.sort_by(|(_, left), (_, right)| right.cmp(left));
    table
        .iter()
        .for_each(|(kind, count)| println!("{}", tab(kind.as_ref(), count.to_string().as_str())));
}

fn tab(left: &str, right: &str) -> String {
    format!(
        "{}:{}{}",
        left,
        " ".repeat(21_usize.checked_sub(left.len()).unwrap_or(1)),
        right
    )
}
