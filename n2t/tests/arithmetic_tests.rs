use n2t::hardware::logic_gate::arithmetic::*;

#[test]
fn test_half_adder() {
    assert_eq!(half_adder(0, 0), Add { sum: 0, carry: 0 });
    assert_eq!(half_adder(0, 1), Add { sum: 1, carry: 0 });
    assert_eq!(half_adder(1, 0), Add { sum: 1, carry: 0 });
    assert_eq!(half_adder(1, 1), Add { sum: 0, carry: 1 });
}

#[test]
fn test_full_adder() {
    assert_eq!(full_adder(0, 0, 0), Add { sum: 0, carry: 0 });
    assert_eq!(full_adder(0, 0, 1), Add { sum: 1, carry: 0 });
    assert_eq!(full_adder(0, 1, 0), Add { sum: 1, carry: 0 });
    assert_eq!(full_adder(0, 1, 1), Add { sum: 0, carry: 1 });
    assert_eq!(full_adder(1, 0, 0), Add { sum: 1, carry: 0 });
    assert_eq!(full_adder(1, 0, 1), Add { sum: 0, carry: 1 });
    assert_eq!(full_adder(1, 1, 0), Add { sum: 0, carry: 1 });
    assert_eq!(full_adder(1, 1, 1), Add { sum: 1, carry: 1 });
}

#[test]
fn test_adder() {
    assert_eq!(
        adder(
            &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        adder(
            &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        ),
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        adder(
            &vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            &vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        ),
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0]
    );
    assert_eq!(
        adder(
            &vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            &vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
        ),
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        adder(
            &vec![0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1],
            &vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0]
        ),
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1]
    );
    assert_eq!(
        adder(
            &vec![0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0],
            &vec![1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0]
        ),
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]
    );
}
#[test]
fn test_incrementer() {
    assert_eq!(
        incrementer(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    );
    assert_eq!(
        incrementer(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        incrementer(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1]),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0]
    );
    assert_eq!(
        incrementer(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1]),
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0]
    );
}

#[test]
fn test_is_equal() {
    assert_eq!(
        is_zero(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        1
    );
    assert_eq!(
        is_zero(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
        0
    );
    assert_eq!(
        is_zero(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        0
    );
}
