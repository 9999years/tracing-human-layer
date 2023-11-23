use expect_test::expect;
use test_harness::Example;

#[test]
fn test_color() {
    let output = Example::name("demo").arg("--color").output().unwrap();

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
    stdout.assert_eq(&output.stdout);

    let stderr = expect![[""]];
    stderr.assert_eq(&output.stderr);
}
