use std::path::PathBuf;

use swc_ecma_transforms_testing::{test, test_fixture};
use swc_ecma_visit::{as_folder};

use my_first_plugin::{TransformVisitor};

#[testing::fixture("tests/fixture/base/input.js")]
fn fixture_base(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| as_folder(TransformVisitor),
    &input,
    &output,
  );
}
