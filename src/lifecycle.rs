pub trait Lifecycle {
    fn tick(&mut self) -> Result<(), String>;
}
