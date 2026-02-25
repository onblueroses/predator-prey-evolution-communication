#[derive(Clone, Debug)]
pub(crate) struct Food {
    pub(crate) energy: f32,
    pub(crate) regrow_timer: u32,
}

impl Food {
    pub(crate) fn new(energy: f32) -> Self {
        Self {
            energy,
            regrow_timer: 0,
        }
    }
}
