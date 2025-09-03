use expect_test::expect;
use test_harness::Example;

#[test]
fn test_wrap_output() {
    let output = Example::name("demo").arg("--no-wrap").output().unwrap();

    let stderr = expect![[r#"
        TRACE Trace event.
        DEBUG Debug event.
        • Info event. field="field-value"
        ⚠ Warn event.
        ⚠ Error event.
        • Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        • Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        • Info event. field="field-value" other_field="my-other-field-value"
        • new
          in my-span{path="my/cool/path.txt"}
        • new
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • new
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        TRACE Trace event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        DEBUG Debug event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • Info event. field="field-value"
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        ⚠ Warn event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        ⚠ Error event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • Info event. field="field-value" other_field="my-other-field-value"
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • close
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • close
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • close
          in my-span{path="my/cool/path.txt"}
    "#]];
    stderr.assert_eq(&output.stderr);
}

#[test]
fn test_wrap_width_output() {
    let output = Example::name("demo")
        .args(["--width", "40"])
        .output()
        .unwrap();

    let stderr = expect![[r#"
        TRACE Trace event.
        DEBUG Debug event.
        • Info event. field="field-value"
        ⚠ Warn event.
        ⚠ Error event.

        • Lorem ipsum dolor sit amet,
          consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et
          dolore magna aliqua. Ut enim ad minim
          veniam, quis nostrud exercitation
          ullamco laboris nisi ut aliquip ex ea
          commodo consequat.

        • Lorem ipsum dolor sit amet,
          consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et
          dolore magna aliqua. Ut enim ad minim
          veniam, quis nostrud exercitation
          ullamco laboris nisi ut aliquip ex ea
          commodo consequat.

        • Info event.
          field="field-value"
          other_field="my-other-field-value"
        • new
          in my-span{path="my/cool/path.txt"}
        • new
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • new
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        TRACE Trace event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        DEBUG Debug event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • Info event. field="field-value"
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        ⚠ Warn event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        ⚠ Error event.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}

        • Lorem ipsum dolor sit amet,
          consectetur adipiscing elit, sed do
          eiusmod tempor incididunt ut labore et
          dolore magna aliqua. Ut enim ad minim
          veniam, quis nostrud exercitation
          ullamco laboris nisi ut aliquip ex ea
          commodo consequat.
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}

        • Info event.
          field="field-value"
          other_field="my-other-field-value"
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • close
          in my-inner-span
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • close
          in copy{path="my/cool/path.txt" other_path="my/second/path.txt"}
          in my-span{path="my/cool/path.txt"}
        • close
          in my-span{path="my/cool/path.txt"}
    "#]];
    stderr.assert_eq(&output.stderr);
}
