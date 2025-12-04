use itertools::iproduct;

pub fn neighboring_indices((i, j): (usize, usize), num_lines: usize, num_columns: usize) -> impl Iterator<Item = (usize, usize)> {
    iproduct!(i.saturating_sub(1) .. (i + 2).min(num_lines), j.saturating_sub(1) .. (j + 2).min(num_columns))
        .filter(move |&(k, l)| k != i || l != j)
}
