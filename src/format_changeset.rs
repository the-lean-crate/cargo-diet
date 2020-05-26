//! Adapted from https://github.com/colin-kiegel/rust-pretty-assertions/blob/master/src/format_changeset.rs and
//! (License MIT)
//! and adapted from https://github.com/johannhof/difference.rs/blob/c5749ad7d82aa3d480c15cb61af9f6baa08f116f/examples/line-by-line.rs
//! (License MIT)
use ansi_term::Colour::{Green, Red};
use ansi_term::Style;
use difference::{Changeset, Difference};

macro_rules! paint {
    ($f:ident, $with_color:ident, $colour:expr, $fmt:expr, $($args:tt)*) => (
        if $with_color {
            write!($f, "{}", $colour.paint(format!($fmt, $($args)*)))?;
        } else {
            write!($f, "{}", format!($fmt, $($args)*))?;
        }
    )
}

const SIGN_RIGHT: char = '+'; // + > →
const SIGN_LEFT: char = '-'; // - < ←

pub fn format_changeset(
    mut f: impl std::io::Write,
    use_color: bool,
    changeset: &Changeset,
) -> std::io::Result<()> {
    let ref diffs = changeset.diffs;

    if use_color {
        writeln!(
            f,
            "{} {} / {} :",
            Style::new().bold().paint("Diff"),
            Red.paint(format!("{} left", SIGN_LEFT)),
            Green.paint(format!("right {}", SIGN_RIGHT))
        )?;
    } else {
        writeln!(
            f,
            "{} {} / {} :",
            "Diff",
            format!("{} left", SIGN_LEFT),
            format!("right {}", SIGN_RIGHT)
        )?;
    }

    let mut t = f;

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                writeln!(t, " {}", x)?;
            }
            Difference::Add(ref x) => {
                paint!(t, use_color, Green, "{}{}\n", SIGN_RIGHT, x);
            }
            Difference::Rem(ref x) => {
                paint!(t, use_color, Red, "{}{}\n", SIGN_LEFT, x);
            }
        }
    }
    Ok(())
}
