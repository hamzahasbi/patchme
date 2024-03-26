use difference::{Changeset, Difference};
use std::fs;
use std::io::{self, Write};

fn main() {
    // Read file paths from user input
    let original_file = read_input("Enter path to the original file: ");
    let modified_file = read_input("Enter path to the modified file: ");

    // Read original and modified files
    let original_content = fs::read_to_string(&original_file).expect("Failed to read original file");
    let modified_content = fs::read_to_string(&modified_file).expect("Failed to read modified file");

    // Calculate differences between original and modified content
    let changeset = Changeset::new(&original_content, &modified_content, "\n");
    let patch = generate_patch(&original_file, &modified_file, changeset.diffs);

    // Write patch to file
    fs::write("patch.diff", patch).expect("Failed to write patch file");

    println!("Patch generated successfully: patch.diff");
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn generate_patch(original_file: &str, modified_file: &str, diffs: Vec<Difference>) -> String {
    let mut patch = String::new();

    // Header
    patch.push_str(&format!("--- {}\n", original_file));
    patch.push_str(&format!("+++ {}\n", modified_file));

    let mut current_line_number = 1;
    let mut added_lines = 0;
    let mut removed_lines = 0;

    for diff in diffs {
        match diff {
            Difference::Same(s) => {
                current_line_number += s.lines().count();
                added_lines = 0;
                removed_lines = 0;
            }
            Difference::Add(s) => {
                if added_lines == 0 {
                    patch.push_str(&format!("@@ -{},{} +{},{} @@\n", current_line_number - 1, removed_lines, current_line_number, s.lines().count()));
                }
                patch.push_str(&format!("+{}\n", s));
                added_lines += s.lines().count();
            }
            Difference::Rem(s) => {
                if removed_lines == 0 {
                    patch.push_str(&format!("@@ -{},{} +{},{} @@\n", current_line_number, s.lines().count(), current_line_number - 1, added_lines));
                }
                patch.push_str(&format!("-{}\n", s));
                removed_lines += s.lines().count();
            }
        }
    }

    patch
}
