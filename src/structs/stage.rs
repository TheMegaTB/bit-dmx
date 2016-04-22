use Fixture;

#[derive(Debug)]
pub struct Stage {
    fixtures: Vec<Fixture>
}

impl Stage {
    pub fn new() -> Stage {
        Stage {
            fixtures: Vec::new()
        }
    }
}
