/// Should we color a piece of output?
///
/// This is nicer than a [`bool`], but I'm not sure how it should interact with
/// `supports_color`.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ShouldColor {
    /// Always color the output.
    Always,
    /// Never color the output.
    Never,
}
