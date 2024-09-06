fn assert_vars(iter: impl IntoIterator<Item = (&'static str, &'static str)>) {
    for (var, expected) in iter {
        pretty_assertions::assert_eq!(std::env::var(var).as_deref(), Ok(expected));
    }
}

#[test]
fn main() {
    nvee::from_path("example.nvee").expect("loading .nvee file should not fail");

    assert_vars([
        ("EXAMPLE_AUTHOR", "kate"),
        ("EXAMPLE_WORD", "jaunt"),
        ("EXAMPLE_TABLE_LUCKY_NUMBER", "777"),
        ("EXAMPLE_ANOTHER_TABLE_EVEN_LUCKIER", "7777"),
        ("EXAMPLE_ANOTHER_TABLE_SNAKE_EYES", "66"),
    ]);
}
