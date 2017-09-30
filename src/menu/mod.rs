use idle;
use rtfm::Threshold;

pub mod feed;

pub trait Menu {
    fn run(&mut self, _t: &mut Threshold, r: &mut idle::Resources);
}
