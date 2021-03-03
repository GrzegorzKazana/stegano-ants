pub fn split_once<'a>(str: &'a str, delimiter: &str) -> Option<(&'a str, &'a str)> {
    let mut chunks = str.split(delimiter);
    let first = chunks.next();
    let second = chunks.next();

    first.zip(second)
}
