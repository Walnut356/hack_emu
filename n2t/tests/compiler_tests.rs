//! Tests for chapter 11

use std::{
    fs::File,
    io::Read,
    iter::zip,
    path::{Path, PathBuf},
};

use n2t::software::compiler::JackCompiler;

pub fn test_data_path(file_path: &str) -> PathBuf {
    match std::env::var("ENV_ROOT_DIR") {
        Ok(path) => Path::new(&path).join(file_path),
        Err(_) => Path::new(&std::env::current_dir().unwrap())
            .join("../")
            .join(file_path),
    }
}

#[test]
fn test_seven() {
    let paths = [(
        "./test_files/ch 11/Seven/Main.jack",
        "./test_files/ch 11/Seven/Main.vm",
        "./test_files/ch 11/Seven/MainTarget.vm",
    )];

    for (jack, vm, target) in paths {
        let path = test_data_path(jack);
        let _vm = JackCompiler::compile(&path);

        let vm_path = test_data_path(vm);
        let mut vm_out = File::open(vm_path).unwrap();
        let mut output_text = String::new();
        vm_out.read_to_string(&mut output_text).unwrap();

        let target_path = test_data_path(target);
        println!("{:?}", target_path);
        let mut target_out = File::open(target_path).unwrap();
        let mut target_text = String::new();
        target_out.read_to_string(&mut target_text).unwrap();

        assert_eq!(
            output_text.lines().count(),
            target_text.lines().count(),
            "files are not the same length"
        );

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
            assert_eq!(a, b);
        }
    }
}

#[test]
/// this test case was manually tested via the software suite due to their compiler outputing
/// semantically different but logically identical code. After checking the output matched the
/// expected (i.e. bit-vec of RAM[8000] in RAM[8001..=8016]), I manually set the test file to be
/// equal to my output to catch regressions.
fn test_convertbin() {
    let paths = [(
        "./test_files/ch 11/ConvertToBin/Main.jack",
        "./test_files/ch 11/ConvertToBin/Main.vm",
        "./test_files/ch 11/ConvertToBin/MainTarget.vm",
    )];

    for (jack, vm, target) in paths {
        let path = test_data_path(jack);
        let _vm = JackCompiler::compile(&path);

        let vm_path = test_data_path(vm);
        let mut vm_out = File::open(vm_path).unwrap();
        let mut output_text = String::new();
        vm_out.read_to_string(&mut output_text).unwrap();

        let target_path = test_data_path(target);
        println!("{:?}", target_path);
        let mut target_out = File::open(target_path).unwrap();
        let mut target_text = String::new();
        target_out.read_to_string(&mut target_text).unwrap();

        assert_eq!(
            output_text.lines().count(),
            target_text.lines().count(),
            "files are not the same length"
        );

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
            // don't fail on dumb label naming conventions
            if (a.starts_with("if-goto") && b.starts_with("if-goto"))
                || (a.starts_with("goto") && b.starts_with("goto"))
                || (a.starts_with("label") && b.starts_with("label"))
            {
                continue;
            }
            assert_eq!(a, b);
        }
    }
}

#[test]
/// this test case was manually tested via the software suite due to their compiler outputing
/// semantically different but logically identical code. After checking the output matched the
/// expected (i.e. game ran correctly), I manually set the test file to be equal to my output to
/// catch regressions.
fn test_square() {
    let paths = [
        (
            "./test_files/ch 11/Square/Main.jack",
            "./test_files/ch 11/Square/Main.vm",
            "./test_files/ch 11/Square/MainTarget.vm",
        ),
        (
            "./test_files/ch 11/Square/Square.jack",
            "./test_files/ch 11/Square/Square.vm",
            "./test_files/ch 11/Square/SquareTarget.vm",
        ),
        (
            "./test_files/ch 11/Square/SquareGame.jack",
            "./test_files/ch 11/Square/SquareGame.vm",
            "./test_files/ch 11/Square/SquareGameTarget.vm",
        ),
    ];

    for (jack, vm, target) in paths {
        let path = test_data_path(jack);
        let _vm = JackCompiler::compile(&path);

        let vm_path = test_data_path(vm);
        let mut vm_out = File::open(vm_path).unwrap();
        let mut output_text = String::new();
        vm_out.read_to_string(&mut output_text).unwrap();

        let target_path = test_data_path(target);

        let mut target_out = File::open(target_path).unwrap();
        let mut target_text = String::new();
        target_out.read_to_string(&mut target_text).unwrap();

        assert_eq!(
            output_text.lines().count(),
            target_text.lines().count(),
            "files are not the same length"
        );

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
        // don't fail on dumb label naming conventions
        if (a.starts_with("if-goto") && b.starts_with("if-goto"))
            || (a.starts_with("goto") && b.starts_with("goto"))
            || (a.starts_with("label") && b.starts_with("label"))
        {
            continue;
        }
        assert_eq!(a, b);
        }
    }
}

#[test]
fn test_average() {
    let paths = [(
        "./test_files/ch 11/Average/Main.jack",
        "./test_files/ch 11/Average/Main.vm",
        "./test_files/ch 11/Average/MainTarget.vm",
    )];

    for (jack, vm, target) in paths {
        let path = test_data_path(jack);
        let _vm = JackCompiler::compile(&path);

        let vm_path = test_data_path(vm);
        let mut vm_out = File::open(vm_path).unwrap();
        let mut output_text = String::new();
        vm_out.read_to_string(&mut output_text).unwrap();

        let target_path = test_data_path(target);
        let mut target_out = File::open(target_path).unwrap();
        let mut target_text = String::new();
        target_out.read_to_string(&mut target_text).unwrap();

        assert_eq!(
            output_text.lines().count(),
            target_text.lines().count(),
            "files are not the same length"
        );

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
            // don't fail on dumb label naming conventions
            if (a.starts_with("if-goto") && b.starts_with("if-goto"))
                || (a.starts_with("goto") && b.starts_with("goto"))
                || (a.starts_with("label") && b.starts_with("label"))
            {
                continue;
            }
            assert_eq!(a, b);
        }
    }
}

#[test]
/// this test case was manually tested via the software suite due to their compiler outputing
/// semantically different but logically identical code. After checking the output matched the
/// expected (i.e. game ran correctly), I manually set the test file to be equal to my output to
/// catch regressions.
fn test_pong() {
    let paths = [
        (
            "./test_files/ch 11/Pong/Main.jack",
            "./test_files/ch 11/Pong/Main.vm",
            "./test_files/ch 11/Pong/MainTarget.vm",
        ),
        (
            "./test_files/ch 11/Pong/Ball.jack",
            "./test_files/ch 11/Pong/Ball.vm",
            "./test_files/ch 11/Pong/BallTarget.vm",
        ),
        (
            "./test_files/ch 11/Pong/Bat.jack",
            "./test_files/ch 11/Pong/Bat.vm",
            "./test_files/ch 11/Pong/BatTarget.vm",
        ),
        (
            "./test_files/ch 11/Pong/PongGame.jack",
            "./test_files/ch 11/Pong/PongGame.vm",
            "./test_files/ch 11/Pong/PongGameTarget.vm",
        ),
    ];

    for (jack, vm, target) in paths {
        let path = test_data_path(jack);
        let _vm = JackCompiler::compile(&path);

        let vm_path = test_data_path(vm);
        let mut vm_out = File::open(vm_path).unwrap();
        let mut output_text = String::new();
        vm_out.read_to_string(&mut output_text).unwrap();

        let target_path = test_data_path(target);
        println!("{:?}", target_path);
        let mut target_out = File::open(target_path).unwrap();
        let mut target_text = String::new();
        target_out.read_to_string(&mut target_text).unwrap();

        assert_eq!(
            output_text.lines().count(),
            target_text.lines().count(),
            "files are not the same length"
        );

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
            // don't fail on dumb label naming conventions
            if (a.starts_with("if-goto") && b.starts_with("if-goto"))
                || (a.starts_with("goto") && b.starts_with("goto"))
                || (a.starts_with("label") && b.starts_with("label"))
            {
                continue;
            }
            assert_eq!(a, b);
        }
    }
}

#[test]
fn test_complexarrays() {
    let paths = [(
        "./test_files/ch 11/ComplexArrays/Main.jack",
        "./test_files/ch 11/ComplexArrays/Main.vm",
        "./test_files/ch 11/ComplexArrays/MainTarget.vm",
    )];

    for (jack, vm, target) in paths {
        let path = test_data_path(jack);
        let _vm = JackCompiler::compile(&path);

        let vm_path = test_data_path(vm);
        let mut vm_out = File::open(vm_path).unwrap();
        let mut output_text = String::new();
        vm_out.read_to_string(&mut output_text).unwrap();

        let target_path = test_data_path(target);
        println!("{:?}", target_path);
        let mut target_out = File::open(target_path).unwrap();
        let mut target_text = String::new();
        target_out.read_to_string(&mut target_text).unwrap();

        assert_eq!(
            output_text.lines().count(),
            target_text.lines().count(),
            "files are not the same length"
        );

        for (a, b) in zip(output_text.lines(), target_text.lines()) {
            // don't fail on dumb label naming conventions
            if (a.starts_with("if-goto") && b.starts_with("if-goto"))
                || (a.starts_with("goto") && b.starts_with("goto"))
                || (a.starts_with("label") && b.starts_with("label"))
            {
                continue;
            }
            assert_eq!(a, b);
        }
    }
}
