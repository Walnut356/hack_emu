use n2t::hardware::logic_gate::memory::*;

#[test]
fn test_bit() {
    let mut dff = DFF::new();
    let input = 0;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 1;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 1;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 1;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 1;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 1;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 1;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 1;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 1;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 0;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 1;
    let out = 1;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 0;
    let load = 1;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
    let input = 1;
    let load = 0;
    let out = 0;
    assert_eq!(dff.data, out);
    dff.cycle(input, load);
}

#[test]
    pub fn test_ram() {
        let mut ram = n2t::hardware::native::memory::RAM::new();

        let input = 0;
        let load = 0;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 0;
        let load = 0;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 0;
        let load = 1;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 0;
        let load = 1;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 4321;
        let load = 0;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 4321;
        let load = 0;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 4321;
        let load = 1;
        let address = 4321;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 4321;
        let load = 1;
        let address = 4321;
        assert_eq!(ram.data[address as usize], 4321);
        ram.cycle(input, address, load);
        let input = 4321;
        let load = 0;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 4321;
        let load = 0;
        let address = 0;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 12345;
        let load = 0;
        let address = 12345;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 12345;
        let load = 0;
        let address = 12345;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 12345;
        let load = 1;
        let address = 12345;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 12345;
        let load = 1;
        let address = 12345;
        assert_eq!(ram.data[address as usize], 12345);
        ram.cycle(input, address, load);
        let input = 12345;
        let load = 0;
        let address = 12345;
        assert_eq!(ram.data[address as usize], 12345);
        ram.cycle(input, address, load);
        let input = 12345;
        let load = 0;
        let address = 12345;
        assert_eq!(ram.data[address as usize], 12345);
        ram.cycle(input, address, load);
        let input = 12345;
        let load = 0;
        let address = 4321;
        assert_eq!(ram.data[address as usize], 4321);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 4321;
        assert_eq!(ram.data[address as usize], 4321);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 4321;
        assert_eq!(ram.data[address as usize], 4321);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 1;
        let address = 16383;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 1;
        let address = 16383;
        assert_eq!(ram.data[address as usize], 16383);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 16383;
        assert_eq!(ram.data[address as usize], 16383);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 16383;
        assert_eq!(ram.data[address as usize], 16383);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 12345;
        assert_eq!(ram.data[address as usize], 12345);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 16383;
        assert_eq!(ram.data[address as usize], 16383);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10920;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10920;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10921;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10922;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10923;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10924;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10925;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10926;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 16383;
        let load = 0;
        let address = 10927;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10920;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10920;
        assert_eq!(ram.data[address as usize], 21845);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10921;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10921;
        assert_eq!(ram.data[address as usize], 21845);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10922;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10922;
        assert_eq!(ram.data[address as usize], 21845);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10923;
        assert_eq!(ram.data[address as usize], 0);
        ram.cycle(input, address, load);
        let input = 21845;
        let load = 1;
        let address = 10923;
        assert_eq!(ram.data[address as usize], 21845);
        ram.cycle(input, address, load);
    }