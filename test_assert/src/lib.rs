pub fn add_two(a: i32) -> i32 {
    a + 2
}

// test module
// 在module声明上方加#[cfg(test)]即可变成test module
#[cfg(test)]
mod tests {
    use super::*;
    
    // test函数，在函数上方加#[test]即可
    #[test]// 发生恐慌时，测试失败
    // #[should_panic]应该发生恐慌：发生恐慌时，测试通过
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    // running 2 tests
    // test tests::it_adds_two ... ok
    // test tests::another ... FAILED
    // ---- tests::another stdout ----
    // thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:19:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


    // failures:
    //     tests::another

    // test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // error: test failed, to rerun pass '--lib'
}