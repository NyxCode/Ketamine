use std::fmt::Display;

pub fn report(src: &str, start: usize, end: usize, msg: impl Display) {
    let (line_idx, char_idx) = get_line_idx(src, start);
    let line = src.lines().nth(line_idx).unwrap();
    let char_in_line = start - char_idx;

    let trimmed_line = line.trim_start();
    let char_in_line = char_in_line - (line.len() - trimmed_line.len());

    println!("{: >3} | {}", line_idx + 1, trimmed_line);
    let marker_offset = " ".repeat(char_in_line + 6);
    println!("{}{}", marker_offset, "^".repeat((end - start).max(1)));
    println!("      {}", msg)
}

fn get_line_idx(src: &str, pos: usize) -> (usize, usize) {
    src[..pos]
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .map(|(char_idx, _c)| char_idx)
        .enumerate()
        .map(|(linebreak_idx, char_idx)| (linebreak_idx + 1, char_idx + 1))
        .last()
        .unwrap_or((0, 0))
}
