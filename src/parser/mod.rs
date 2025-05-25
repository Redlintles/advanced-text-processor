pub mod writer;
pub mod reader;
use regex::Regex;

use crate::data::{ AtpToken };

fn trim_both_sides(text: &str) -> String {
    String::from(text.trim())
}
fn trim_left_side(text: &str) -> String {
    String::from(text.trim_start())
}
fn trim_right_side(text: &str) -> String {
    String::from(text.trim_end())
}
fn add_to_beginning(text: &str, chunk: &str) -> String {
    let mut s = String::from(chunk);
    s.push_str(text);
    s
}
fn add_to_end(text: &str, chunk: &str) -> String {
    let mut s = String::from(text);
    s.push_str(chunk);
    s
}

fn delete_first(text: &str) -> String {
    let mut s = String::from(text);
    s.drain(..1);
    s
}

fn delete_last(text: &str) -> String {
    let mut s = String::from(text);

    if let Some((x, _)) = s.char_indices().rev().next() {
        s.drain(x..);
    }

    s
}

fn delete_after(text: &str, index: usize) -> String {
    let mut s = String::from(text);

    if
        let Some(byte_index) = s
            .char_indices()
            .nth(index)
            .map(|(i, _)| i)
    {
        s.drain(byte_index..);
    }

    s
}
fn delete_before(text: &str, index: usize) -> String {
    let mut s = String::from(text);

    if
        let Some(byte_index) = s
            .char_indices()
            .nth(index)
            .map(|(i, _)| i)
    {
        s.drain(..=byte_index);
    }

    s
}

fn delete_chunk(text: &str, start: usize, end: usize) -> String {
    let start_index = text
        .char_indices()
        .nth(start)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    let end_index = text
        .char_indices()
        .nth(end)
        .map(|(i, _)| i)
        .unwrap_or(text.len());

    String::from(&text[start_index..end_index])
}

fn replace_all_with(text: &str, pattern: &str, to_replace: &str) -> String {
    let re = Regex::new(pattern).unwrap();

    let result = re.replace_all(text, to_replace);

    result.to_string()
}
fn replace_first_with(text: &str, pattern: &str, to_replace: &str) -> String {
    let re = Regex::new(pattern).unwrap();

    let result = re.replace(text, to_replace);

    result.to_string()
}

pub fn parse_token(token: AtpToken, to_process: &str) -> String {
    match token {
        AtpToken::Tbs(_) => trim_both_sides(to_process),
        AtpToken::Tls(_) => trim_left_side(to_process),
        AtpToken::Trs(_) => trim_right_side(to_process),
        AtpToken::Atb(obj) => add_to_beginning(to_process, &obj.text),
        AtpToken::Ate(obj) => add_to_end(to_process, &obj.text),
        AtpToken::Dlf(_) => delete_first(to_process),
        AtpToken::Dll(_) => delete_last(to_process),
        AtpToken::Dla(obj) => delete_after(to_process, obj.index),
        AtpToken::Dlb(obj) => delete_before(to_process, obj.index),
        AtpToken::Dlc(obj) => delete_chunk(to_process, obj.start_index, obj.end_index),
        AtpToken::Raw(obj) => replace_all_with(to_process, &obj.pattern, &obj.text_to_replace),
        AtpToken::Rfw(obj) => replace_first_with(to_process, &obj.pattern, &obj.text_to_replace),
    }
}
