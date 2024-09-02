use std::env;

#[test]
fn dotnvee() {
    nvee::dotnvee().expect("loading .nvee file should not fail");

    pretty_assertions::assert_eq!(env::var("AUTHOR").as_deref(), Ok("kate"));
    pretty_assertions::assert_eq!(env::var("TABLE_LUCKY_NUMBER").as_deref(), Ok("777"));
    pretty_assertions::assert_eq!(
        env::var("ANOTHER_TABLE_EVEN_LUCKIER").as_deref(),
        Ok("7777")
    );
}
