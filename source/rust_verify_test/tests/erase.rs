#![feature(rustc_private)]
#[macro_use]
mod common;
use common::*;

test_verify_one_file! {
    #[test] test_regression_78 verus_code! {
        use vstd::prelude::*;

        proof fn f1<T>(t: Ghost<Option<T>>) {
            let x = t.view()->0;
        }

        spec fn f2() -> bool {
            let x: Option<usize> = Option::None;
            x is None
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_regression_70 verus_code! {
        fn m(v: &mut u64) { }

        fn main() {
            let v = 6;
            m(&mut v);
        }
    } => Err(err) => assert_rust_error_msg(err, "cannot borrow `v` as mutable, as it is not declared as mutable")
}

test_verify_one_file! {
    #[test] ghost_stub_cfg_issue3157 verus_code! {
        mod m {
            pub fn helper() -> u8 {
                7
            }
        }

        use m::*;

        #[cfg(any())]
        proof fn helper() {
        }

        #[cfg_attr(all(), cfg(any()))]
        proof fn helper() {
        }

        #[test]
        proof fn helper() {
        }

        proof fn helper() {
            #![cfg(any())]
        }

        #[cfg(any())]
        spec const helper: u8 = 1;

        fn test() -> u8 {
            helper()
        }

        fn test_nested() -> u8 {
            #[cfg(any())]
            proof fn helper() {
            }

            helper()
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] cfg_attr_verifier_ghost_fn_stub_issue3157 verus_code! {
        #[cfg_attr(verus_keep_ghost, verifier::external_body)]
        proof fn lemma() {
        }
    } => Ok(())
}
