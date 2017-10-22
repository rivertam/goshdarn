extern crate cursive;
extern crate git2;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, ListView,
                     SelectView, TextView};

use git2::Repository;

fn main() {
    let repo = Repository::open(std::path::Path::new(".")).unwrap();
    let head = repo.head().unwrap();
    let name = head.name().unwrap();
    println!("{} name", name);

    let diff = repo.diff_tree_to_workdir_with_index(None, None).unwrap();
    diff.print(git2::DiffFormat::Raw, |diff_delta, maybe_hunk, diff_line| {
        let num_files = diff_delta.nfiles();
        let hunk_start = maybe_hunk.unwrap().old_start();
        println!("files: {}, {}", num_files, hunk_start);
        return true;
    }).unwrap();
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
