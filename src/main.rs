extern crate cursive;
extern crate git2;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, ListView,
                     SelectView, TextView};

use git2::Repository;

fn main() {
    get_diffs();
}

fn get_diffs<'a>() -> &'a std::collections::HashMap<
                        String,
                        (&'a mut std::vec::Vec<String>, &'a mut std::vec::Vec<String>)
                    > {
    let repo = Repository::open(std::path::Path::new(".")).unwrap();

    let diff = repo.diff_index_to_workdir(None, None).unwrap();

    let mut file_diffs:
        std::collections::HashMap<
            String,
            (&'a mut std::vec::Vec<String>, &'a mut std::vec::Vec<String>)
        > = std::collections::HashMap::new();

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
            let content = String::from_utf8(content_vec).unwrap();
            let trimmed_content = content.trim();

            let file_name = match diff_line.origin() {
                '+' => {
                    let path = diff_delta.new_file().path().unwrap();
                    String::from(path.to_str().unwrap())
                },
                '-' => {
                    let path = diff_delta.old_file().path().unwrap();
                    String::from(path.to_str().unwrap())
                },
                _  => String::from("???"),
            };

            if !file_diffs.contains_key(&file_name) {
                file_diffs.insert(file_name.clone(), (&mut vec![], &mut vec![]));
            }

            match diff_line.origin() {
                '+' => {
                    (file_diffs[&file_name].1).push(String::from(trimmed_content));
                },
                '-' => {
                },
                _ => {},
            }

            true
        })).unwrap();

    &mut file_diffs
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
