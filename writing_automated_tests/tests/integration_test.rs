/// 集成测试
/// 可通过 cargo test --test integration_test 来只执行 tests/integration_test.rs 旗下的测试
use writing_automated_tests;

mod common;

#[test]
#[ignore = "我乐意"]
fn it_adds_one() {
    common::setup();
    assert_eq!(2, writing_automated_tests::add_group::add_one(1));
}
