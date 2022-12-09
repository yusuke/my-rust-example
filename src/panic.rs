use std::process::{ExitCode, Termination};

#[test]
fn test_panic() {
    one();
}

fn one() {
    two();
}

fn two() {
    three();
}

fn three() {
    let t = Exit {};
    panic!("fail at three");
}


struct Exit {}

impl Termination for Exit {
    fn report(self) -> ExitCode {
        println!("exit impl termination!!!!!");
        panic!("exit impl termination!!!!!");
        ExitCode::FAILURE
    }
}

#[test]
fn test_termination() {
    let _ = create_exit();
    println!("test")
}

fn create_exit() -> Exit {
    Exit {}
}
