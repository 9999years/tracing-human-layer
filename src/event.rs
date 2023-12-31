use std::fmt;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tracing::field::Field;
use tracing::field::Visit;
use tracing::Level;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::Scope;

use crate::textwrap::TextWrapOptionsExt;
use crate::SpanInfo;

use super::HumanFields;
use super::Style;

#[derive(Debug)]
pub struct HumanEvent {
    pub last_event_was_long: AtomicBool,
    style: Style,
    /// Spans, in root-to-current (outside-in) order.
    spans: Vec<SpanInfo>,
    pub fields: HumanFields,
}

impl HumanEvent {
    pub fn new<S>(
        level: Level,
        last_event_was_long: AtomicBool,
        scope: Option<Scope<'_, S>>,
        color: bool,
    ) -> Self
    where
        S: tracing::Subscriber,
        S: for<'lookup> LookupSpan<'lookup>,
    {
        Self {
            last_event_was_long,
            style: Style::new(level, color),
            fields: HumanFields::new_event(),
            spans: scope
                .map(|scope| SpanInfo::from_scope(scope))
                .unwrap_or_default(),
        }
    }
}

impl Visit for HumanEvent {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.fields
            .record_field(field.name().to_owned(), format!("{value:?}"))
    }
}

impl fmt::Display for HumanEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent_colored = self.style.indent_colored();

        let options = crate::textwrap_options()
            .initial_indent(&indent_colored)
            .subsequent_indent(self.style.subsequent_indent);

        let mut message = self.fields.message.clone().unwrap_or_default();

        // If there's only one field, and it fits on the same line as the message, put it on the
        // same line. Otherwise, we use the 'long format' with each field on a separate line.
        let short_format = self.fields.use_short_format(options.width);

        if short_format {
            for (name, value) in &self.fields.fields {
                message.push_str(&format!(" {}", self.style.style_field(name, value)));
            }
        }

        // Next, color the message _before_ wrapping it. If you wrap before coloring,
        // `textwrap` prepends the `initial_indent` to the first line. The `initial_indent` is
        // colored, so it has a reset sequence at the end, and the message ends up uncolored.
        let message_colored = self.style.style_message(&message);

        let lines = options.wrap(&message_colored);

        // If there's more than one line of message, add a blank line before and after the message.
        // This doesn't account for fields, but I think that's fine?
        let add_blank_lines = lines.len() > 1;
        // Store `add_blank_lines` and fetch the previous value:
        let last_event_was_long = self
            .last_event_was_long
            .swap(add_blank_lines, Ordering::SeqCst);
        if add_blank_lines && !last_event_was_long {
            writeln!(f)?;
        };

        // Write the actual message, line by line.
        for line in &lines {
            writeln!(f, "{line}")?;
        }

        // Add fields, one per line, at the end.
        if !short_format {
            for (name, value) in &self.fields.fields {
                writeln!(
                    f,
                    "{}{}",
                    self.style.subsequent_indent,
                    self.style.style_field(name, value)
                )?;
            }
        }

        // Add spans, one per line, at the end.
        // TODO: Short format for spans?
        for span in self.spans.iter().rev() {
            writeln!(
                f,
                "{}{}",
                self.style.subsequent_indent,
                self.style.style_span(span),
            )?;
        }

        // If there's more than one line of output, add a blank line before and after the message.
        if add_blank_lines {
            writeln!(f)?;
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use expect_test::expect;
    use expect_test::Expect;
    use indoc::indoc;

    // /!\   /!\   /!\   /!\   /!\   /!\   /!\   /!\
    //
    // NOTE: The tests here have non-printing characters for ANSI terminal escapes in them.
    //
    // Be sure to configure your editor to display them!
    //
    // /!\   /!\   /!\   /!\   /!\   /!\   /!\   /!\

    fn check(actual: HumanEvent, expected: Expect) {
        owo_colors::set_override(true);
        let actual = actual.to_string();
        expected.assert_eq(&actual);
    }

    #[test]
    fn test_simple() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::INFO, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some(
                        "Checking access to Mercury repositories on GitHub over SSH".to_owned(),
                    ),
                    fields: Default::default(),
                },
                spans: vec![],
            },
            expect![[r#"
                [32m• [0mChecking access to Mercury repositories on GitHub over SSH
            "#]],
        );
    }

    #[test]
    fn test_short_format() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::INFO, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some("User `nix.conf` is already OK".to_owned()),
                    fields: vec![(
                        "path".to_owned(),
                        "/Users/wiggles/.config/nix/nix.conf".to_owned(),
                    )],
                },
                spans: vec![],
            },
            expect![[r#"
                [32m• [0mUser `nix.conf` is already OK [1mpath[0m=/Users/wiggles/.config/nix/nix.conf
            "#]],
        );
    }

    #[test]
    fn test_short_format_long_field() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::INFO, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some("User `nix.conf` is already OK".to_owned()),
                    fields: vec![(
                        "path".to_owned(),
                        // this field is too long to fit on one line, so we use the long format
                        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
                            .to_owned(),
                    )],
                },
                spans: vec![],
            },
            expect![[r#"
                [32m• [0mUser `nix.conf` is already OK
                  [1mpath[0m=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
            "#]],
        );
    }

    #[test]
    fn test_long_format() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::INFO, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some("User `nix.conf` is already OK".to_owned()),
                    // Multiple fields means we use the long format.
                    fields: vec![
                        ("path".to_owned(), "~/.config/nix/nix.conf".to_owned()),
                        ("user".to_owned(), "puppy".to_owned()),
                    ],
                },
                spans: vec![],
            },
            expect![[r#"
                [32m• [0mUser `nix.conf` is already OK
                  [1mpath[0m=~/.config/nix/nix.conf
                  [1muser[0m=puppy
            "#]],
        );
    }

    #[test]
    fn test_long_warning() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::WARN, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some(
                        indoc!(
                            "
                            `nix doctor` found potential issues with your Nix installation:
                            Running checks against store uri: daemon
                            [FAIL] Multiple versions of nix found in PATH:
                              /nix/store/lr32i0bdarx1iqsch4sy24jj1jkfw9vf-nix-2.11.0/bin
                              /nix/store/s1j8d1x2jlfkb2ckncal8a700hid746p-nix-2.11.0/bin

                            [PASS] All profiles are gcroots.
                            [PASS] Client protocol matches store protocol.
                            "
                        )
                        .to_owned(),
                    ),
                    fields: vec![],
                },
                spans: vec![],
            },
            expect![[r#"

                [33m⚠ [0m[33m`nix doctor` found potential issues with your Nix installation:
                  Running checks against store uri: daemon
                  [FAIL] Multiple versions of nix found in PATH:
                    /nix/store/lr32i0bdarx1iqsch4sy24jj1jkfw9vf-nix-2.11.0/bin
                    /nix/store/s1j8d1x2jlfkb2ckncal8a700hid746p-nix-2.11.0/bin

                  [PASS] All profiles are gcroots.
                  [PASS] Client protocol matches store protocol.
                  [0m

            "#]],
        );
    }

    #[test]
    fn test_long_warning_last_was_long() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::WARN, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some(
                        indoc!(
                            "
                            `nix doctor` found potential issues with your Nix installation:
                            Running checks against store uri: daemon
                            [FAIL] Multiple versions of nix found in PATH:
                              /nix/store/lr32i0bdarx1iqsch4sy24jj1jkfw9vf-nix-2.11.0/bin
                              /nix/store/s1j8d1x2jlfkb2ckncal8a700hid746p-nix-2.11.0/bin

                            [PASS] All profiles are gcroots.
                            [PASS] Client protocol matches store protocol.
                            "
                        )
                        .to_owned(),
                    ),
                    fields: vec![],
                },
                spans: vec![],
            },
            expect![[r#"

                [33m⚠ [0m[33m`nix doctor` found potential issues with your Nix installation:
                  Running checks against store uri: daemon
                  [FAIL] Multiple versions of nix found in PATH:
                    /nix/store/lr32i0bdarx1iqsch4sy24jj1jkfw9vf-nix-2.11.0/bin
                    /nix/store/s1j8d1x2jlfkb2ckncal8a700hid746p-nix-2.11.0/bin

                  [PASS] All profiles are gcroots.
                  [PASS] Client protocol matches store protocol.
                  [0m

            "#]],
        );
    }

    #[test]
    fn test_trace() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::TRACE, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some("Fine-grained tracing info".to_owned()),
                    fields: vec![("favorite_doggy_sound".to_owned(), "awooooooo".to_owned())],
                },
                spans: vec![],
            },
            expect![[r#"
                [35mTRACE [0m[2mFine-grained tracing info [1;2mfavorite_doggy_sound[0m[2m=awooooooo[0m[0m
            "#]],
        );
    }

    #[test]
    fn test_debug() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::DEBUG, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some("Debugging info".to_owned()),
                    fields: vec![("puppy".to_owned(), "pawbeans".to_owned())],
                },
                spans: vec![],
            },
            expect![[r#"
                [34mDEBUG [0m[2mDebugging info [1;2mpuppy[0m[2m=pawbeans[0m[0m
            "#]],
        );
    }

    #[test]
    fn test_wrapping() {
        check(
            HumanEvent {
                last_event_was_long: AtomicBool::new(false),
                style: Style::new(Level::WARN, true),
                fields: HumanFields {
                    extract_message: true,
                    message: Some("I was unable to clone `mercury-web-backend`; most likely this is because you don't have a proper SSH key available.\n\
                        Note that access to Mercury repositories on GitHub over SSH is required to enter the `nix develop` shell in `mercury-web-backend`\n\
                        See: https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account".to_owned()),
                    fields: vec![],
                },
                spans: vec![],
            },
            expect![[r#"

                [33m⚠ [0m[33mI was unable to clone `mercury-web-backend`; most likely this is because you
                  don't have a proper SSH key available.
                  Note that access to Mercury repositories on GitHub over SSH is required to
                  enter the `nix develop` shell in `mercury-web-backend`
                  See:
                  https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account[0m

            "#]],
        );
    }
}
