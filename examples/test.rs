use std::thread::sleep;
use std::time::Duration;

use tracyrs::{zone, FrameGuard};

fn func1() {
    zone!("func1");
    func2();
}

fn func2() {
    zone!("func2");
    sleep(Duration::from_millis(16));
}

fn func3() {
    zone!("func3");
    sleep(Duration::from_millis(1));
}

fn main() {
    zone!("main");
    loop {
        let _frame = FrameGuard::new();
        func1();
        func2();
        for _ in 0..100 {
            func3();
        }
    }
}
