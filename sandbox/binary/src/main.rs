use lumbermill::Logger;

fn main() {
  Logger::default().init();
  library::add(1, 2);
}
