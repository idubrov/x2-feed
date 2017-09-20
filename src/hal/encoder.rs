pub trait Encoder {
    fn set_limit(&mut self, limit : u16);

    fn current(&self) -> u16;

    fn set_current(&mut self, pos: u16);
}