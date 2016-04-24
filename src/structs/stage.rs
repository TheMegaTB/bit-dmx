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
    pub fn add_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }
}
