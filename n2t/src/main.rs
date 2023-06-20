use std::time::Instant;

fn main() {
    bench_alu();
    bench_native();
}

fn bench_alu() {
    use n2t::logic_gate::alu::*;
    use n2t::logic_gate::arithmetic::*;
    use n2t::logic_gate::gates::*;
    let mut control = ControlBits {
        zx: 0,
        nx: 0,
        zy: 1,
        ny: 1,
        f: 1,
        no: 0,
        zr: 0,
        ng: 0,
    };
    let mut val = vec![0; 16];

    let now = Instant::now();
    for i in 0..10000 {
        val = ALU(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            &mut control,
        );
    }
    let dur = now.elapsed();
    println!("{:?}", dur.as_micros());

    println!("{val:?}");
}

fn bench_native() {
    use n2t::native::alu::*;
    // use n2t::native::arithmetic::*;
    // use n2t::native::gates::*;
    let mut control = ControlBits {
        zx: false,
        nx: false,
        zy: true,
        ny: true,
        f: true,
        no: false,
        zr: false,
        ng: false,
    };
    let mut val = 0;

    let now = Instant::now();
    for i in 0..10000 {
        val = ALU(17, 3, &mut control);
    }
    let dur = now.elapsed();
    println!("{:?}", dur.as_micros());

    println!("{val:?}");
}
