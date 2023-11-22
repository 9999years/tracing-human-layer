use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

use escargot::format::Message;
use expect_test::expect;

fn example_path(name: &str) -> PathBuf {
    let messages = escargot::CargoBuild::new().example(name).exec().unwrap();
    for message in messages {
        if let Message::CompilerArtifact(artifact) = message.unwrap().decode().unwrap() {
            if artifact.target.name != name
                || !artifact.target.kind.contains(&Cow::Borrowed("example"))
            {
                continue;
            }
            return artifact.executable.unwrap().into_owned();
        }
    }
    panic!("No example output binary found");
}

fn example_output(name: &str, args: &[&str]) -> Output {
    let example = example_path(name);
    let output = Command::new(example)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    assert!(output.status.success());

    output
}

#[test]
fn my_test() {
    let output = example_output("demo", &[]);

    let stdout = expect![[r#"
        TRACE Trace event.
        DEBUG Debug event.
        • Info event. field="field-value"
        ⚠ Warn event.
        ⚠ Error event.

        • Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
          incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
          nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

        • Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
          incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
          nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

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

        • Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
          incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
          nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
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
    stdout.assert_eq(&String::from_utf8(output.stdout).unwrap());

    let stderr = expect![[""]];
    stderr.assert_eq(&String::from_utf8(output.stderr).unwrap());
}
