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

    EncounterBoy,
    EncounterGirl,

    BattleWild, // 44.480
    BattleTrainer, // 1:41.870
    BattleGym, // 56.780
    // BattleChampion,

}

pub const MUSIC_LIST: &[Music; 21] = &[
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
    Music::BattleTrainer,
    Music::BattleGym,
    Music::ViridianForest,
    Music::MountMoon,
    Music::Gym,
];

impl Default for Music {
    fn default() -> Self {
        Music::Pallet
    }
}