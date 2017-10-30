extern crate tui;
extern crate termion;
extern crate git2;

use git2::Repository;
use std::io;
use termion::event;
use termion::input::TermRead;

fn main() {
    let diffs = get_diffs();

    let stdin = io::stdin();

    let backend = tui::backend::TermionBackend::new().unwrap();
    let mut terminal = tui::Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    draw(&mut terminal, &diffs);

    let mut term_size = terminal.size().unwrap();
    for c in stdin.keys() {
        let size = terminal.size().unwrap();
        if term_size != size {
            terminal.resize(size).unwrap();
            term_size = size;
        }

        draw(&mut terminal, &diffs);
        let evt = c.unwrap();
        if evt == event::Key::Char('q') {
            break;
        }
    }
}

struct Line {
    line_number: u32,
    content: String,
}

impl Clone for Line {
    fn clone(&self) -> Line {
        Line {
            line_number: self.line_number,
            content: self.content.clone(),
        }
    }
}

struct Diff {
    file_name: String,
    old: std::vec::Vec<Line>,
    new: std::vec::Vec<Line>,
}

impl Clone for Diff {
    fn clone(&self) -> Diff {
        Diff {
            file_name: self.file_name.clone(),
            old: self.old.clone(),
            new: self.new.clone(),
        }
    }
}

fn get_diffs() -> std::vec::Vec<Diff> {
    let repo = Repository::open(std::path::Path::new(".")).unwrap();

    let diff = repo.diff_index_to_workdir(None, None).unwrap();

    let mut diffs: std::collections::HashMap<String, Diff> = std::collections::HashMap::new();

    diff.foreach(
        &mut |_, _| { true },
        None,
        None,
        Some(&mut |diff_delta, _maybe_hunk, diff_line| {
            let mut old_line = 0;
            let mut new_line = 0;

            match diff_line.old_lineno() {
                Some(old_no) => old_line = old_no,
                None => {},
            };
            match diff_line.new_lineno() {
                Some(new_no) => new_line = new_no,
                None => {},
            };

            let content_vec = diff_line.content().to_vec();
            let content = String::from_utf8(content_vec).unwrap().to_owned();
            let trimmed_content = content.trim().to_string();

            let file_name = match diff_line.origin() {
                '+' => {
                    let path = diff_delta.new_file().path().unwrap();
                    String::from(path.to_str().unwrap())
                },
                '-' => {
                    let path = diff_delta.old_file().path().unwrap();
                    String::from(path.to_str().unwrap())
                },
                _  => { return true; },
            };

            let d = diffs.entry(file_name.clone()).or_insert(Diff { file_name: file_name.clone(), new: vec![], old: vec![] });
            match diff_line.origin() {
                '+' => {
                    d.new.push(Line { line_number: new_line, content: trimmed_content });
                },
                '-' => {
                    d.old.push(Line { line_number: old_line, content: trimmed_content });
                },
                _ => {},
            }

            true
        })).unwrap();

    diffs.iter().map(|(_, diff)| Diff {
        file_name: diff.file_name.clone(),
        old: diff.old.clone(),
        new: diff.new.clone(),
    }).collect()
}

fn draw(t: &mut tui::Terminal<tui::backend::TermionBackend>, diffs: &std::vec::Vec<Diff>) {
    use tui::widgets::*;
    use tui::layout::*;

    let size = t.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(90), Size::Percent(10)])
        .render(t, &size, |t, chunks| {
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[0], |t, chunks| {
                    Block::default()
                        .title("Block")
                        .borders(border::ALL)
                        .render(t, &chunks[0]);
                    Block::default()
                        .title("Block")
                        .borders(border::ALL)
                        .render(t, &chunks[1]);

                });

            Block::default()
                .title("Files")
                .borders(border::ALL)
                .render(t, &chunks[1]);
        });


    t.draw().unwrap();
}
