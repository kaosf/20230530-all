#[derive(Debug)]
struct EmptyError;

fn all1(a: &[bool]) -> bool {
    if a.is_empty() {
        true
    } else {
        a[0] && all1(&a[1..])
    }
}

fn all(a: &[bool]) -> Result<bool, EmptyError> {
    if a.is_empty() {
        Err(EmptyError)
    } else {
        Ok(all1(&a))
    }
}

#[test]
fn all_works() {
    assert!(all(&[]).is_err());
    assert_eq!(all(&[true]).unwrap(), true);
    assert_eq!(all(&[false]).unwrap(), false);
    assert_eq!(all(&[true, true]).unwrap(), true);
    assert_eq!(all(&[true, false]).unwrap(), false);
    assert_eq!(all(&[false, true]).unwrap(), false);
    assert_eq!(all(&[false, false]).unwrap(), false);
}

fn main() {
    println!(
        "{}",
        match all(&[]) {
            Ok(true) => "True",
            Ok(false) => "False",
            Err(EmptyError) => "EmptyError",
        }
    );
    println!("{}", [].iter().all(|x| *x));
    println!("Run `cargo t`");
}
