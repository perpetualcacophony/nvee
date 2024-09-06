fn assert_vars(iter: impl IntoIterator<Item = (&'static str, &'static str)>) {
    for (var, expected) in iter {
        pretty_assertions::assert_eq!(std::env::var(var).as_deref(), Ok(expected));
    }
}

#[test]
fn main() {
    nvee::dotnvee().expect("loading .nvee file should not fail");

    assert_vars([
        ("AUTHOR", "kate"),
        ("WORD", "jaunt"),
        ("TABLE_LUCKY_NUMBER", "777"),
        ("ANOTHER_TABLE_EVEN_LUCKIER", "7777"),
        ("ANOTHER_TABLE_SNAKE_EYES", "66"),
    ]);
}
