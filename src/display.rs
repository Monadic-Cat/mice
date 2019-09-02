/// Mbot-centric extension for displaying dice expressions how I want.
/// A better formatting API and control of information propagation
/// are in the docket, but I serve my needs first. This will serve
/// as a proving ground for those features.
///
/// `<term total>` := `N`
/// `<term summary>` := `EXP -> N`
/// `<term>` := `EXP -> N0 [+ N$]*`
/// `<verbose term>` := `EXP -> N0 [+ N$]* = N`
///
/// Concise:
/// `<total>`
/// Less concise:
/// `<total> = (<term total> [, <term total>]*)`
/// Even less concise:
/// `<total> = (<term summary> [, <term summary>]*)`
///
/// Verbosity Levels:
/// | Verbosity Level | Structure                                  |
/// | 0               | `T`                                        |
/// | 1               | `T = N [, N]*`                             |
/// | 2               | `T = EXP -> N [, EXP -> N]*`               |
/// | 3               | `T = EXP -> N [+ N]* [, EXP -> N [+ N]*]*` |
///
/// Alternative scheme:
/// Verbosity Levels:
/// | Verbosity Level | Structure                                      |
/// | 0               | `T`                                            |
/// | 1               | `T = N [+ N]*`                                 |
/// | 2               | `T = (EXP -> N) [+ (EXP -> N)]*`               |
/// | 3               | `T = (EXP -> N [+ N]*) [+ (EXP -> N [+ N]*)]*` |
///
/// The alternative scheme is preferred for its more explicit addition.
/// Use the "→" character in place of "->".
/// Use verbosity level 3 for mbot.
use crate::parse::{Expr, Term};
use crate::post::{EvaluatedTerm, ExpressionResult, FormatOptions};

/// `T = (EXP → N [+ N]*) [+ (EXP → N [+ N]*)]*`
pub fn mbot_format(e: ExpressionResult) -> String {
    let pairs = e.pairs();
    // let mut nstr = format!("{}", e.total());
    let mut nstr = String::new();
    nstr = if pairs.is_empty() {
        nstr
    } else if pairs.len() > 1 || pairs[0].1.parts().len() > 1 {
        // VERBOSE TIME.
        // format!("{}", e)
        // nstr.push_str(" = ");
        let mut iter = pairs.iter();
        let first = iter.next().unwrap();
        let formatting = FormatOptions::new().exclude_sign();
        let form = |prior: &Expr, val: &EvaluatedTerm| match prior.term {
            Term::Constant(_) => val.format(&formatting),
            Term::Die(_) => format!(
                "({} → {})",
                prior.format(&formatting),
                val.format(&formatting)
            ),
        };
        nstr.push_str(&form(&first.0, &first.1));
        for (before, after) in iter {
            nstr.push_str(&format!(" {} ", after.sign()));
            nstr.push_str(&form(before, &after));
        }
        nstr
    } else {
        nstr
    };
    nstr.push_str(&format!(" = {}", e.total()));
    nstr
}
