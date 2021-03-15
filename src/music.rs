#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Music {

    IntroGamefreak,
    // IntroPokemon,
    Title, // 45.010

    Pallet, // 43.640
    Pewter,
    Fuchsia,
    Lavender,
    Celadon,
    Cinnabar,
    Vermilion,

    Route1, // 25.090
    Route2,
    Route3,
    Route4,

    ViridianForest,
    MountMoon,
    Gym,
    OaksLab,

    EncounterBoy,
    EncounterGirl,
    EncounterRival,
    Oak,

    BattleWild, // 44.480
    BattleTrainer, // 1:41.870
    BattleGym, // 56.780
    // BattleChampion,

}

pub const MUSIC_LIST: &[Music; 23] = &[
    Music::IntroGamefreak,
    Music::Title,
    Music::Route1,
    Music::Route2,
    Music::Route3,
    Music::Route4,
    Music::Pallet,
    Music::Pewter,
    Music::Fuchsia,
    Music::Lavender,
    Music::Celadon,
    Music::Cinnabar,
    Music::Vermilion,
    Music::BattleWild,
    Music::EncounterBoy,
    Music::EncounterGirl,
    Music::EncounterRival,
    Music::BattleTrainer,
    Music::BattleGym,
    Music::ViridianForest,
    Music::MountMoon,
    Music::Gym,
    Music::Oak,
];

impl Default for Music {
    fn default() -> Self {
        Music::Pallet
    }
}

impl std::convert::TryFrom<u8> for Music {
    type Error = U8ToMusicError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1F => Ok(Music::ViridianForest),
            0x13 => Ok(Music::Gym),
            0x20 => Ok(Music::MountMoon),
            0x23 => Ok(Music::Route1),
            0x24 => Ok(Music::Route2),
            0x25 => Ok(Music::Route3),
            0x26 => Ok(Music::Route4),
            0x34 => Ok(Music::Fuchsia),
            0x3A => Ok(Music::Pewter),
            0x18 => Ok(Music::Lavender),
            0x35 => Ok(Music::Celadon),
            0x17 => Ok(Music::Cinnabar),
            0x39 => Ok(Music::Vermilion),
            0x2C => Ok(Music::Pallet),
            0x2D => Ok(Music::OaksLab),
            0x2E => Ok(Music::Oak),
            _ => Err(U8ToMusicError::Missing),
        }
    }
}

#[derive(Debug)]
pub enum U8ToMusicError {
    Missing
}

impl std::error::Error for U8ToMusicError {}

impl core::fmt::Display for U8ToMusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}