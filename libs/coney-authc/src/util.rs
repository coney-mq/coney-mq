pub fn split_zero(s: &str) -> Vec<&str> {
    s.split('\0').skip(1).collect()
}

pub fn join_zero<S: AsRef<str>>(parts: &[S]) -> String {
    let mut acc = String::new();

    for part in parts {
        acc.push('\0');
        acc.push_str(part.as_ref());
    }

    acc
}

#[test]
fn t0() {
    assert_eq!(split_zero("\0user\0password"), vec!["user", "password"])
}

#[test]
fn t1() {
    assert_eq!(split_zero(""), Vec::<&str>::new())
}

#[test]
fn t2() {
    assert_eq!(split_zero("should be empty"), Vec::<&str>::new())
}
