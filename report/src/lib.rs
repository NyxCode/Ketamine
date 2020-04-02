use std::fmt::Display;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

macro_rules! report {
    ($out:expr, $src:expr, $start:expr, $end:expr, $msg:expr) => {{
            let (line_idx, char_idx) = get_line_idx($src, $start);
            let line = $src.lines().nth(line_idx).unwrap();
            let char_in_line = $start - char_idx;

            let trimmed_line = line.trim_start();
            let char_in_line = char_in_line - (line.len() - trimmed_line.len());

            writeln!($out, "{: >3} | {}", line_idx + 1, trimmed_line)?;
            let marker_offset = " ".repeat(char_in_line + 6);
            writeln!(
                $out,
                "{}{}",
                marker_offset,
                "^".repeat(($end - $start).max(1).min(trimmed_line.len()))
            )?;
            writeln!($out, "      {}", $msg)
    }};
}

pub fn report_io(out: &mut impl IoWrite, src: &str, start: usize, end: usize, msg: impl Display) -> IoResult<()> {
    report!(out, src, start, end, msg)
}

pub fn report_fmt(out: &mut impl FmtWrite, src: &str, start: usize, end: usize, msg: impl Display) -> FmtResult {
    report!(out, src, start, end, msg)
}


pub fn report_string(src: &str, start: usize, end: usize, msg: impl Display) -> String {
    let mut out = String::new();
    report_fmt(&mut out, src, start, end, msg).unwrap();
    out
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

