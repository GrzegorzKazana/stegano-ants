use std::path::Path;

/// transforms path or filename by inserting an infix prior to extension
///
/// ```
/// assert_eq!(extend_basename("my_file.jpg", "_test"), "my_file_test.jpg");
/// assert_eq!(extend_basename("foo/my_file.jpg", "_test"), "foo/my_file_test.jpg");
/// ```
pub fn extend_basename(name: &str, infix: &str) -> Option<String> {
    let directory = Path::new(name).parent().unwrap_or(Path::new(""));
    let basename = Path::new(name).file_stem().and_then(|a| a.to_str());
    let extension = Path::new(name).extension().and_then(|a| a.to_str());

    basename
        .zip(extension)
        .map(|(base, ext)| format!("{}{}.{}", base, infix, ext))
        .and_then(|filename| {
            directory
                .join(filename)
                .to_str()
                .map(|path| path.to_owned())
        })
}
