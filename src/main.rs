extern crate cursive;
extern crate git2;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, ListView,
                     SelectView, TextView};

use git2::Repository;

fn main() {
    let diffs = get_diffs();
    for diff in diffs {
        println!("Diff for {}", diff.file_name);
        for old in diff.old {
            println!("- {}: {}", old.line_number, old.content);
        }

        for new in diff.new {
            println!("+ {}: {}", new.line_number, new.content);
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

fn show_ui() {
    let mut siv = Cursive::new();

    siv.add_layer(
        Dialog::new()
            .title("Please fill out this form")
            .button("Ok", |s| s.quit())
            .content(
                ListView::new()
                    .child("Name", EditView::new().fixed_width(10))
                    .child(
                        "Receive spam?",
                        Checkbox::new().on_change(
                            |s, checked| for name in &["email1", "email2"] {
                                s.call_on_id(name, |view: &mut EditView| {
                                    view.set_enabled(checked)
                                });
                                if checked {
                                    s.focus_id("email1").unwrap();
                                }
                            },
                        ),
                    )
                    .child(
                        "Email",
                        LinearLayout::horizontal()
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_id("email1")
                                    .fixed_width(15),
                            )
                            .child(TextView::new("@"))
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_id("email2")
                                    .fixed_width(10),
                            ),
                    )
                    .delimiter()
                    .child(
                        "Age",
                        SelectView::new()
                            .popup()
                            .item_str("0-18")
                            .item_str("19-30")
                            .item_str("31-40")
                            .item_str("41+"),
                    )
                    .with(|list| for i in 0..50 {
                        list.add_child(
                                &format!("Item {}", i),
                                EditView::new(),
                            );
                    }),
            ),
    );

    siv.run();
}
