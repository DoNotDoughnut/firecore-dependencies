#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Sound {

    // Save,
    // Select,
    Cry(u16),

}