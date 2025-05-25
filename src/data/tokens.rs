// Trim both sides

#[derive(Clone, Copy)]
pub struct Tbs {}

// Trim left side
#[derive(Clone, Copy)]
pub struct Tls {}

// Trim right side
#[derive(Clone, Copy)]
pub struct Trs {}
// Replace all with
#[derive(Clone)]
pub struct Raw {
    pub pattern: String,
    pub text_to_replace: String,
}

// Replace first with
#[derive(Clone)]
pub struct Rfw {
    pub pattern: String,
    pub text_to_replace: String,
}
// add to end
#[derive(Clone)]
pub struct Ate {
    pub text: String,
}
// add to beginning
#[derive(Clone)]
pub struct Atb {
    pub text: String,
}
// Delete before
#[derive(Clone, Copy)]
pub struct Dlb {
    pub index: usize,
}

// Delete after
#[derive(Clone, Copy)]
pub struct Dla {
    pub index: usize,
}
// Delete Chunk
#[derive(Clone, Copy)]
pub struct Dlc {
    pub start_index: usize,
    pub end_index: usize,
}

// Delete first
#[derive(Clone, Copy)]
pub struct Dlf {}

// Delete last
#[derive(Clone, Copy)]
pub struct Dll {}
