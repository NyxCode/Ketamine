use std::fmt::Display;

pub fn report(src: &str, start: usize, end: usize, msg: impl Display) {
    let (line_idx, char_idx) = get_line_idx(src, start);
    let line = src.lines().skip(line_idx).next().unwrap();
    let char_in_line = start - char_idx;

    let trimmed_line = line.trim_start();
    let char_in_line = char_in_line - (line.len() - trimmed_line.len());

    println!("{: >3} | {}", line_idx + 1, trimmed_line);
    let marker_offset = " ".repeat(char_in_line + 6);
    println!("{}{}", marker_offset, "^".repeat((end - start).max(1)));
    println!("      {}", msg)
}

fn get_line_idx(src: &str, pos: usize) -> (usize, usize) {
    *&src[..pos]
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '\n')
        .map(|(char_idx, _c)| char_idx)
        .enumerate()
        .map(|(linebreak_idx, char_idx)| (linebreak_idx + 1, char_idx + 1))
        .last()
        .unwrap_or((0, 0))
}

fn get_line(src: &str, pos: usize) -> (usize, &str) {
    let line_start = src[..pos].rfind("\n").map(|l| l + 1).unwrap_or(0);
    let line_end = src[pos..]
        .find("\n")
        .map(|p| p + pos)
        .unwrap_or(src.len() - 1);
    let line_num = *&src[..line_start].chars().filter(|c| *c == '\n').count();
    println!("start: {}", line_start);
    return (line_num, &src[line_start..line_end]);
}
