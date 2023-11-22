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
    let output = example_output("demo", &["--color"]);

    let stdout = expect![[r#"
        [35mTRACE [0m[2mTrace event.[0m
        [34mDEBUG [0m[2mDebug event.[0m
        [32m• [0mInfo event. [1mfield[0m="field-value"
        [33m⚠ [0m[33mWarn event.[0m
        [31m⚠ [0m[31mError event.[0m

        [32m• [0mLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
          incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
          nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

        [32m• [0mLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
          incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
          nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

        [32m• [0mInfo event.
          [1mfield[0m="field-value"
          [1mother_field[0m="my-other-field-value"
        [32m• [0mnew
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [32m• [0mnew
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [32m• [0mnew
          [2min [0mmy-inner-span
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [35mTRACE [0m[2mTrace event.[0m
          [2min [0m[2mmy-inner-span[0m
          [2min [0m[2mcopy[0m{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0m[2mmy-span[0m{[1mpath[0m="my/cool/path.txt"}
        [34mDEBUG [0m[2mDebug event.[0m
          [2min [0m[2mmy-inner-span[0m
          [2min [0m[2mcopy[0m{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0m[2mmy-span[0m{[1mpath[0m="my/cool/path.txt"}
        [32m• [0mInfo event. [1mfield[0m="field-value"
          [2min [0mmy-inner-span
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [33m⚠ [0m[33mWarn event.[0m
          [2min [0mmy-inner-span
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [31m⚠ [0m[31mError event.[0m
          [2min [0mmy-inner-span
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}

        [32m• [0mLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
          incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
          nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
          [2min [0mmy-inner-span
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}

        [32m• [0mInfo event.
          [1mfield[0m="field-value"
          [1mother_field[0m="my-other-field-value"
          [2min [0mmy-inner-span
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [32m• [0mclose
          [2min [0mmy-inner-span
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [32m• [0mclose
          [2min [0mcopy{[1mpath[0m="my/cool/path.txt" [1mother_path[0m="my/second/path.txt"}
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
        [32m• [0mclose
          [2min [0mmy-span{[1mpath[0m="my/cool/path.txt"}
    "#]];
    stdout.assert_eq(&String::from_utf8(output.stdout).unwrap());

    let stderr = expect![[""]];
    stderr.assert_eq(&String::from_utf8(output.stderr).unwrap());
}
