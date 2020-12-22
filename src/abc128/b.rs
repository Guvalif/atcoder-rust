pub type Scores<'a> = Vec<(&'a str, u32)>;

pub fn solve(_n: u32, scores: Scores) -> Vec<u32> {
  let mut scores: Vec<(String, u32)> = scores
    .into_iter()
    .enumerate()
    .map(|(i, (name, score))| (format!("{}{:03}", name, 100 - score), (i + 1) as u32))
    .collect();

  scores.sort_by(|(lhs, _), (rhs, _)| lhs.partial_cmp(rhs).unwrap());

  scores
    .into_iter()
    .map(|(_, i)| i)
    .collect()
}
