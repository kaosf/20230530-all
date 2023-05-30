fn all(a: &[bool]) -> bool {
    if a.is_empty() {
        true
    } else {
        a[0] && all(&a[1..])
    }
}

#[test]
fn all_works() {
    assert_eq!(all(&[]), true);
    assert_eq!(all(&[true]), true);
    assert_eq!(all(&[false]), false);
    assert_eq!(all(&[true, true]), true);
    assert_eq!(all(&[true, false]), false);
    assert_eq!(all(&[false, true]), false);
    assert_eq!(all(&[false, false]), false);
}

fn main() {
    println!("{}", all(&[]));
    println!("{}", [].iter().all(|x| *x));
    println!("Run `cargo t`");
}
