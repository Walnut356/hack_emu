use n2t::hardware::logic_gate::cpu::*;
use n2t::utils::*;

#[test]
fn test_cpu() {
    let mut comp = Computer::new(vec![
        0b0000000000000000,
        0b1111110000010000,
        0b0000000000000001,
        0b1111010011010000,
        0b0000000000001010,
        0b1110001100000001,
        0b0000000000000001,
        0b1111110000010000,
        0b0000000000001100,
        0b1110101010000111,
        0b0000000000000000,
        0b1111110000010000,
        0b0000000000000010,
        0b1110001100001000,
        0b0000000000001110,
        0b1110101010000111,
    ]);
    comp.ram.data[0] = 3;
    comp.ram.data[1] = 5;

    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (0, 0, 1, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (0, 3, 2, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (1, 3, 3, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (1, -2, 4, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (10, -2, 5, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (10, -2, 6, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (1, -2, 7, 3, 5, 0,)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (1, 5, 8, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (12, 5, 9, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (12, 5, 12, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (2, 5, 13, 3, 5, 0)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (2, 5, 14, 3, 5, 5,)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (14, 5, 15, 3, 5, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (14, 5, 14, 3, 5, 5)
    );
    comp.execute(true, true);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (14, 5, 0, 3, 5, 5)
    );
    comp.ram.data[0] = 23456;
    comp.ram.data[1] = 12345;
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (14, 5, 0, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (0, 5, 1, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (0, 23456, 2, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (1, 23456, 3, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (1, 11111, 4, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (10, 11111, 5, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (10, 11111, 10, 23456, 12345, 5,)
    );

    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (0, 11111, 11, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (0, 23456, 12, 23456, 12345, 5)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (2, 23456, 13, 23456, 12345, 5,)
    );
    comp.execute(true, false);
    assert_eq!(
        (
            int_from_bitvec(&comp.a.data),
            i16::from_ne_bytes(int_from_bitvec(&comp.d.data).to_ne_bytes()),
            int_from_bitvec(&comp.pc.val.data),
            comp.ram.data[0],
            comp.ram.data[1],
            comp.ram.data[2]
        ),
        (2, 23456, 14, 23456, 12345, 23456)
    );
}
