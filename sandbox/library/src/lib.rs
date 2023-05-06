use lumbermill::info;

pub fn add(left: usize, right: usize) -> usize {
  info!(left, right, "Adding");
  left + right
}
