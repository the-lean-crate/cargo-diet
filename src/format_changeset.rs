//! Adapted from https://github.com/colin-kiegel/rust-pretty-assertions/blob/master/src/format_changeset.rs and
//! (License MIT)
//! and adapted from https://github.com/johannhof/difference.rs/blob/c5749ad7d82aa3d480c15cb61af9f6baa08f116f/examples/line-by-line.rs
//! (License MIT)
use ansi_term::Colour::{Green, Red};
use ansi_term::Style;
use difference::{Changeset, Difference};

#[macro_export]
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

fn lines_of(d: &Difference) -> impl Iterator<Item = &str> {
    let buf = match d {
        Difference::Same(b) | Difference::Rem(b) | Difference::Add(b) => b,
    };
    buf.split('\n')
}

fn print_context<'a>(
    lines: Option<(
        Option<String>,
        impl Iterator<Item = &'a str>,
        Option<String>,
    )>,
    f: &mut impl std::io::Write,
    use_color: bool,
) -> std::io::Result<()> {
    match lines {
        None => {}
        Some((before, lines, after)) => {
            if let Some(before) = before {
                paint!(f, use_color, Style::new().dimmed(), "{}\n", before,);
            }
            for line in lines {
                writeln!(f, "{}", line)?;
            }
            if let Some(after) = after {
                paint!(f, use_color, Style::new().dimmed(), "{}\n", after,);
            }
        }
    };
    Ok(())
}

pub fn format_changeset(
    mut t: impl std::io::Write,
    use_color: bool,
    changeset: &Changeset,
) -> std::io::Result<()> {
    let diffs = &changeset.diffs;

    if use_color {
        writeln!(
            t,
            "{} {} / {} :",
            Style::new().bold().paint("Diff"),
            Red.paint(format!("{} removed", SIGN_LEFT)),
            Green.paint(format!("added {}", SIGN_RIGHT))
        )?;
    } else {
        writeln!(
            t,
            "Diff {} / {} :",
            format!("{} removed", SIGN_LEFT),
            format!("added {}", SIGN_RIGHT)
        )?;
    }

    let is_different = |d: &_| {
        if let Difference::Same(_) = d {
            false
        } else {
            true
        }
    };

    let first_changed_hunk = diffs.iter().position(is_different);
    let last_changed_hunk = diffs.iter().rposition(is_different);
    let context = 2;
    {
        let hunk_before_first_change = first_changed_hunk.and_then(|l| l.checked_sub(1));
        print_context(
            hunk_before_first_change.map(|l| {
                let lines_outside_of_context =
                    lines_of(&diffs[l]).by_ref().count().saturating_sub(context);
                (
                    if lines_outside_of_context > 0 {
                        Some(format!("[…skipped {} lines…]", lines_outside_of_context))
                    } else {
                        None
                    },
                    lines_of(&diffs[l]).skip(lines_outside_of_context),
                    None,
                )
            }),
            &mut t,
            use_color,
        )?;
    }

    if let (Some(first_changed_hunk), Some(last_changed_hunk)) =
        (first_changed_hunk, last_changed_hunk)
    {
        for diff in diffs
            .iter()
            .take(last_changed_hunk + 1)
            .skip(first_changed_hunk)
        {
            match diff {
                Difference::Same(x) => {
                    writeln!(t, " {}", x)?;
                }
                Difference::Add(x) => {
                    paint!(t, use_color, Green, "{}{}\n", SIGN_RIGHT, x);
                }
                Difference::Rem(x) => {
                    paint!(t, use_color, Red, "{}{}\n", SIGN_LEFT, x);
                }
            }
        }
    }

    {
        let hunk_after_last_change = last_changed_hunk.map(|l| (l + 1).min(diffs.len()));
        print_context(
            hunk_after_last_change.map(|l| {
                let skipped_lines_note = lines_of(&diffs[l])
                    .count()
                    .checked_sub(context)
                    .map(|skipped| format!("[…skipped {} lines…]", skipped));
                (None, lines_of(&diffs[l]).take(context), skipped_lines_note)
            }),
            &mut t,
            use_color,
        )?;
    }
    Ok(())
}
