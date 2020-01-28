// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
// ./scripts/update-generated-code.sh
pub(crate) fn find_single_character_fraction(nominator: i64, denominator: i64) -> Option<char> {
    match (nominator, denominator) {
        (0i64, 3i64) => Some('\u{2189}'),
        (1i64, 2i64) => Some('\u{bd}'),
        (1i64, 3i64) => Some('\u{2153}'),
        (1i64, 4i64) => Some('\u{bc}'),
        (1i64, 5i64) => Some('\u{2155}'),
        (1i64, 6i64) => Some('\u{2159}'),
        (1i64, 7i64) => Some('\u{2150}'),
        (1i64, 8i64) => Some('\u{215b}'),
        (1i64, 9i64) => Some('\u{2151}'),
        (1i64, 10i64) => Some('\u{2152}'),
        (2i64, 3i64) => Some('\u{2154}'),
        (2i64, 5i64) => Some('\u{2156}'),
        (3i64, 4i64) => Some('\u{be}'),
        (3i64, 5i64) => Some('\u{2157}'),
        (3i64, 8i64) => Some('\u{215c}'),
        (4i64, 5i64) => Some('\u{2158}'),
        (5i64, 6i64) => Some('\u{215a}'),
        (5i64, 8i64) => Some('\u{215d}'),
        (7i64, 8i64) => Some('\u{215e}'),
        _ => None,
    }
}