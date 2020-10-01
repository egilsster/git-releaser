pub fn commit(commit_message: &str) {
    println!("git commit -am \"{}\"", commit_message);
}

pub fn tag(text: &str) {
    println!("git tag: {}", text);
}

/// Pushes everything committed to the main upstream
/// i.e. just `git push`
pub fn push() {
    println!("git push");
}

/// Pushes the new tag to the main upstream
/// i.e. `git push origin TAG`
pub fn push_tag(tag_ver: &str) {
    println!("git push origin {}", tag_ver);
}

// Abort if there are changes in the directory, maybe ask to abort or stash and un-stash after
pub fn ensure_no_changes() {
    println!("ensuring no changes")
}
