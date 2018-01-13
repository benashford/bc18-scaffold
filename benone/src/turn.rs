use bc::controller::GameController;

pub(crate) struct Turn {}

impl Turn {
    pub(crate) fn new(gc: &GameController) -> Turn {
        Turn {}
    }

    pub(crate) fn update(&mut self, gc: &GameController) {}
}
