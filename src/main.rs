use rustevo::engine::setup::run;

fn main() {
    pollster::block_on(run());
}