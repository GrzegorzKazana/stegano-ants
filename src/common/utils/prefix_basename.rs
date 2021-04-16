use std::path::Path;

/// transforms path or filename by inserting an prefix prior to basename
///
/// ```
/// assert_eq!(extend_basename("my_file.jpg", "test_"), "test_my_file.jpg");
/// assert_eq!(extend_basename("foo/my_file.jpg", "test_"), "foo/test_my_file.jpg");
/// ```
pub fn prefix_basename(name: &str, prefix: &str) -> Option<String> {
    let directory = Path::new(name).parent().unwrap_or(Path::new(""));
    let basename = Path::new(name).file_name().and_then(|a| a.to_str());

    basename
        .map(|base| format!("{}{}", prefix, base))
        .and_then(|filename| {
            directory
                .join(filename)
                .to_str()
                .map(|path| path.to_owned())
        })
}
