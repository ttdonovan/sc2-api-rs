// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ActionResult {
    Success = 1,
    NotSupported = 2,
    Error = 3,
    CantQueueThatOrder = 4,
    Retry = 5,
    Cooldown = 6,
    QueueIsFull = 7,
    RallyQueueIsFull = 8,
    NotEnoughMinerals = 9,
    NotEnoughVespene = 10,
    NotEnoughTerrazine = 11,
    NotEnoughCustom = 12,
    NotEnoughFood = 13,
    FoodUsageImpossible = 14,
    NotEnoughLife = 15,
    NotEnoughShields = 16,
    NotEnoughEnergy = 17,
    LifeSuppressed = 18,
    ShieldsSuppressed = 19,
    EnergySuppressed = 20,
    NotEnoughCharges = 21,
    CantAddMoreCharges = 22,
    TooMuchMinerals = 23,
    TooMuchVespene = 24,
    TooMuchTerrazine = 25,
    TooMuchCustom = 26,
    TooMuchFood = 27,
    TooMuchLife = 28,
    TooMuchShields = 29,
    TooMuchEnergy = 30,
    MustTargetUnitWithLife = 31,
    MustTargetUnitWithShields = 32,
    MustTargetUnitWithEnergy = 33,
    CantTrade = 34,
    CantSpend = 35,
    CantTargetThatUnit = 36,
    CouldntAllocateUnit = 37,
    UnitCantMove = 38,
    TransportIsHoldingPosition = 39,
    BuildTechRequirementsNotMet = 40,
    CantFindPlacementLocation = 41,
    CantBuildOnThat = 42,
    CantBuildTooCloseToDropOff = 43,
    CantBuildLocationInvalid = 44,
    CantSeeBuildLocation = 45,
    CantBuildTooCloseToCreepSource = 46,
    CantBuildTooCloseToResources = 47,
    CantBuildTooFarFromWater = 48,
    CantBuildTooFarFromCreepSource = 49,
    CantBuildTooFarFromBuildPowerSource = 50,
    CantBuildOnDenseTerrain = 51,
    CantTrainTooFarFromTrainPowerSource = 52,
    CantLandLocationInvalid = 53,
    CantSeeLandLocation = 54,
    CantLandTooCloseToCreepSource = 55,
    CantLandTooCloseToResources = 56,
    CantLandTooFarFromWater = 57,
    CantLandTooFarFromCreepSource = 58,
    CantLandTooFarFromBuildPowerSource = 59,
    CantLandTooFarFromTrainPowerSource = 60,
    CantLandOnDenseTerrain = 61,
    AddOnTooFarFromBuilding = 62,
    MustBuildRefineryFirst = 63,
    BuildingIsUnderConstruction = 64,
    CantFindDropOff = 65,
    CantLoadOtherPlayersUnits = 66,
    NotEnoughRoomToLoadUnit = 67,
    CantUnloadUnitsThere = 68,
    CantWarpInUnitsThere = 69,
    CantLoadImmobileUnits = 70,
    CantRechargeImmobileUnits = 71,
    CantRechargeUnderConstructionUnits = 72,
    CantLoadThatUnit = 73,
    NoCargoToUnload = 74,
    LoadAllNoTargetsFound = 75,
    NotWhileOccupied = 76,
    CantAttackWithoutAmmo = 77,
    CantHoldAnyMoreAmmo = 78,
    TechRequirementsNotMet = 79,
    MustLockdownUnitFirst = 80,
    MustTargetUnit = 81,
    MustTargetInventory = 82,
    MustTargetVisibleUnit = 83,
    MustTargetVisibleLocation = 84,
    MustTargetWalkableLocation = 85,
    MustTargetPawnableUnit = 86,
    YouCantControlThatUnit = 87,
    YouCantIssueCommandsToThatUnit = 88,
    MustTargetResources = 89,
    RequiresHealTarget = 90,
    RequiresRepairTarget = 91,
    NoItemsToDrop = 92,
    CantHoldAnyMoreItems = 93,
    CantHoldThat = 94,
    TargetHasNoInventory = 95,
    CantDropThisItem = 96,
    CantMoveThisItem = 97,
    CantPawnThisUnit = 98,
    MustTargetCaster = 99,
    CantTargetCaster = 100,
    MustTargetOuter = 101,
    CantTargetOuter = 102,
    MustTargetYourOwnUnits = 103,
    CantTargetYourOwnUnits = 104,
    MustTargetFriendlyUnits = 105,
    CantTargetFriendlyUnits = 106,
    MustTargetNeutralUnits = 107,
    CantTargetNeutralUnits = 108,
    MustTargetEnemyUnits = 109,
    CantTargetEnemyUnits = 110,
    MustTargetAirUnits = 111,
    CantTargetAirUnits = 112,
    MustTargetGroundUnits = 113,
    CantTargetGroundUnits = 114,
    MustTargetStructures = 115,
    CantTargetStructures = 116,
    MustTargetLightUnits = 117,
    CantTargetLightUnits = 118,
    MustTargetArmoredUnits = 119,
    CantTargetArmoredUnits = 120,
    MustTargetBiologicalUnits = 121,
    CantTargetBiologicalUnits = 122,
    MustTargetHeroicUnits = 123,
    CantTargetHeroicUnits = 124,
    MustTargetRoboticUnits = 125,
    CantTargetRoboticUnits = 126,
    MustTargetMechanicalUnits = 127,
    CantTargetMechanicalUnits = 128,
    MustTargetPsionicUnits = 129,
    CantTargetPsionicUnits = 130,
    MustTargetMassiveUnits = 131,
    CantTargetMassiveUnits = 132,
    MustTargetMissile = 133,
    CantTargetMissile = 134,
    MustTargetWorkerUnits = 135,
    CantTargetWorkerUnits = 136,
    MustTargetEnergyCapableUnits = 137,
    CantTargetEnergyCapableUnits = 138,
    MustTargetShieldCapableUnits = 139,
    CantTargetShieldCapableUnits = 140,
    MustTargetFlyers = 141,
    CantTargetFlyers = 142,
    MustTargetBuriedUnits = 143,
    CantTargetBuriedUnits = 144,
    MustTargetCloakedUnits = 145,
    CantTargetCloakedUnits = 146,
    MustTargetUnitsInAStasisField = 147,
    CantTargetUnitsInAStasisField = 148,
    MustTargetUnderConstructionUnits = 149,
    CantTargetUnderConstructionUnits = 150,
    MustTargetDeadUnits = 151,
    CantTargetDeadUnits = 152,
    MustTargetRevivableUnits = 153,
    CantTargetRevivableUnits = 154,
    MustTargetHiddenUnits = 155,
    CantTargetHiddenUnits = 156,
    CantRechargeOtherPlayersUnits = 157,
    MustTargetHallucinations = 158,
    CantTargetHallucinations = 159,
    MustTargetInvulnerableUnits = 160,
    CantTargetInvulnerableUnits = 161,
    MustTargetDetectedUnits = 162,
    CantTargetDetectedUnits = 163,
    CantTargetUnitWithEnergy = 164,
    CantTargetUnitWithShields = 165,
    MustTargetUncommandableUnits = 166,
    CantTargetUncommandableUnits = 167,
    MustTargetPreventDefeatUnits = 168,
    CantTargetPreventDefeatUnits = 169,
    MustTargetPreventRevealUnits = 170,
    CantTargetPreventRevealUnits = 171,
    MustTargetPassiveUnits = 172,
    CantTargetPassiveUnits = 173,
    MustTargetStunnedUnits = 174,
    CantTargetStunnedUnits = 175,
    MustTargetSummonedUnits = 176,
    CantTargetSummonedUnits = 177,
    MustTargetUser1 = 178,
    CantTargetUser1 = 179,
    MustTargetUnstoppableUnits = 180,
    CantTargetUnstoppableUnits = 181,
    MustTargetResistantUnits = 182,
    CantTargetResistantUnits = 183,
    MustTargetDazedUnits = 184,
    CantTargetDazedUnits = 185,
    CantLockdown = 186,
    CantMindControl = 187,
    MustTargetDestructibles = 188,
    CantTargetDestructibles = 189,
    MustTargetItems = 190,
    CantTargetItems = 191,
    NoCalldownAvailable = 192,
    WaypointListFull = 193,
    MustTargetRace = 194,
    CantTargetRace = 195,
    MustTargetSimilarUnits = 196,
    CantTargetSimilarUnits = 197,
    CantFindEnoughTargets = 198,
    AlreadySpawningLarva = 199,
    CantTargetExhaustedResources = 200,
    CantUseMinimap = 201,
    CantUseInfoPanel = 202,
    OrderQueueIsFull = 203,
    CantHarvestThatResource = 204,
    HarvestersNotRequired = 205,
    AlreadyTargeted = 206,
    CantAttackWeaponsDisabled = 207,
    CouldntReachTarget = 208,
    TargetIsOutOfRange = 209,
    TargetIsTooClose = 210,
    TargetIsOutOfArc = 211,
    CantFindTeleportLocation = 212,
    InvalidItemClass = 213,
    CantFindCancelOrder = 214,
}

impl ::protobuf::ProtobufEnum for ActionResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ActionResult> {
        match value {
            1 => ::std::option::Option::Some(ActionResult::Success),
            2 => ::std::option::Option::Some(ActionResult::NotSupported),
            3 => ::std::option::Option::Some(ActionResult::Error),
            4 => ::std::option::Option::Some(ActionResult::CantQueueThatOrder),
            5 => ::std::option::Option::Some(ActionResult::Retry),
            6 => ::std::option::Option::Some(ActionResult::Cooldown),
            7 => ::std::option::Option::Some(ActionResult::QueueIsFull),
            8 => ::std::option::Option::Some(ActionResult::RallyQueueIsFull),
            9 => ::std::option::Option::Some(ActionResult::NotEnoughMinerals),
            10 => ::std::option::Option::Some(ActionResult::NotEnoughVespene),
            11 => ::std::option::Option::Some(ActionResult::NotEnoughTerrazine),
            12 => ::std::option::Option::Some(ActionResult::NotEnoughCustom),
            13 => ::std::option::Option::Some(ActionResult::NotEnoughFood),
            14 => ::std::option::Option::Some(ActionResult::FoodUsageImpossible),
            15 => ::std::option::Option::Some(ActionResult::NotEnoughLife),
            16 => ::std::option::Option::Some(ActionResult::NotEnoughShields),
            17 => ::std::option::Option::Some(ActionResult::NotEnoughEnergy),
            18 => ::std::option::Option::Some(ActionResult::LifeSuppressed),
            19 => ::std::option::Option::Some(ActionResult::ShieldsSuppressed),
            20 => ::std::option::Option::Some(ActionResult::EnergySuppressed),
            21 => ::std::option::Option::Some(ActionResult::NotEnoughCharges),
            22 => ::std::option::Option::Some(ActionResult::CantAddMoreCharges),
            23 => ::std::option::Option::Some(ActionResult::TooMuchMinerals),
            24 => ::std::option::Option::Some(ActionResult::TooMuchVespene),
            25 => ::std::option::Option::Some(ActionResult::TooMuchTerrazine),
            26 => ::std::option::Option::Some(ActionResult::TooMuchCustom),
            27 => ::std::option::Option::Some(ActionResult::TooMuchFood),
            28 => ::std::option::Option::Some(ActionResult::TooMuchLife),
            29 => ::std::option::Option::Some(ActionResult::TooMuchShields),
            30 => ::std::option::Option::Some(ActionResult::TooMuchEnergy),
            31 => ::std::option::Option::Some(ActionResult::MustTargetUnitWithLife),
            32 => ::std::option::Option::Some(ActionResult::MustTargetUnitWithShields),
            33 => ::std::option::Option::Some(ActionResult::MustTargetUnitWithEnergy),
            34 => ::std::option::Option::Some(ActionResult::CantTrade),
            35 => ::std::option::Option::Some(ActionResult::CantSpend),
            36 => ::std::option::Option::Some(ActionResult::CantTargetThatUnit),
            37 => ::std::option::Option::Some(ActionResult::CouldntAllocateUnit),
            38 => ::std::option::Option::Some(ActionResult::UnitCantMove),
            39 => ::std::option::Option::Some(ActionResult::TransportIsHoldingPosition),
            40 => ::std::option::Option::Some(ActionResult::BuildTechRequirementsNotMet),
            41 => ::std::option::Option::Some(ActionResult::CantFindPlacementLocation),
            42 => ::std::option::Option::Some(ActionResult::CantBuildOnThat),
            43 => ::std::option::Option::Some(ActionResult::CantBuildTooCloseToDropOff),
            44 => ::std::option::Option::Some(ActionResult::CantBuildLocationInvalid),
            45 => ::std::option::Option::Some(ActionResult::CantSeeBuildLocation),
            46 => ::std::option::Option::Some(ActionResult::CantBuildTooCloseToCreepSource),
            47 => ::std::option::Option::Some(ActionResult::CantBuildTooCloseToResources),
            48 => ::std::option::Option::Some(ActionResult::CantBuildTooFarFromWater),
            49 => ::std::option::Option::Some(ActionResult::CantBuildTooFarFromCreepSource),
            50 => ::std::option::Option::Some(ActionResult::CantBuildTooFarFromBuildPowerSource),
            51 => ::std::option::Option::Some(ActionResult::CantBuildOnDenseTerrain),
            52 => ::std::option::Option::Some(ActionResult::CantTrainTooFarFromTrainPowerSource),
            53 => ::std::option::Option::Some(ActionResult::CantLandLocationInvalid),
            54 => ::std::option::Option::Some(ActionResult::CantSeeLandLocation),
            55 => ::std::option::Option::Some(ActionResult::CantLandTooCloseToCreepSource),
            56 => ::std::option::Option::Some(ActionResult::CantLandTooCloseToResources),
            57 => ::std::option::Option::Some(ActionResult::CantLandTooFarFromWater),
            58 => ::std::option::Option::Some(ActionResult::CantLandTooFarFromCreepSource),
            59 => ::std::option::Option::Some(ActionResult::CantLandTooFarFromBuildPowerSource),
            60 => ::std::option::Option::Some(ActionResult::CantLandTooFarFromTrainPowerSource),
            61 => ::std::option::Option::Some(ActionResult::CantLandOnDenseTerrain),
            62 => ::std::option::Option::Some(ActionResult::AddOnTooFarFromBuilding),
            63 => ::std::option::Option::Some(ActionResult::MustBuildRefineryFirst),
            64 => ::std::option::Option::Some(ActionResult::BuildingIsUnderConstruction),
            65 => ::std::option::Option::Some(ActionResult::CantFindDropOff),
            66 => ::std::option::Option::Some(ActionResult::CantLoadOtherPlayersUnits),
            67 => ::std::option::Option::Some(ActionResult::NotEnoughRoomToLoadUnit),
            68 => ::std::option::Option::Some(ActionResult::CantUnloadUnitsThere),
            69 => ::std::option::Option::Some(ActionResult::CantWarpInUnitsThere),
            70 => ::std::option::Option::Some(ActionResult::CantLoadImmobileUnits),
            71 => ::std::option::Option::Some(ActionResult::CantRechargeImmobileUnits),
            72 => ::std::option::Option::Some(ActionResult::CantRechargeUnderConstructionUnits),
            73 => ::std::option::Option::Some(ActionResult::CantLoadThatUnit),
            74 => ::std::option::Option::Some(ActionResult::NoCargoToUnload),
            75 => ::std::option::Option::Some(ActionResult::LoadAllNoTargetsFound),
            76 => ::std::option::Option::Some(ActionResult::NotWhileOccupied),
            77 => ::std::option::Option::Some(ActionResult::CantAttackWithoutAmmo),
            78 => ::std::option::Option::Some(ActionResult::CantHoldAnyMoreAmmo),
            79 => ::std::option::Option::Some(ActionResult::TechRequirementsNotMet),
            80 => ::std::option::Option::Some(ActionResult::MustLockdownUnitFirst),
            81 => ::std::option::Option::Some(ActionResult::MustTargetUnit),
            82 => ::std::option::Option::Some(ActionResult::MustTargetInventory),
            83 => ::std::option::Option::Some(ActionResult::MustTargetVisibleUnit),
            84 => ::std::option::Option::Some(ActionResult::MustTargetVisibleLocation),
            85 => ::std::option::Option::Some(ActionResult::MustTargetWalkableLocation),
            86 => ::std::option::Option::Some(ActionResult::MustTargetPawnableUnit),
            87 => ::std::option::Option::Some(ActionResult::YouCantControlThatUnit),
            88 => ::std::option::Option::Some(ActionResult::YouCantIssueCommandsToThatUnit),
            89 => ::std::option::Option::Some(ActionResult::MustTargetResources),
            90 => ::std::option::Option::Some(ActionResult::RequiresHealTarget),
            91 => ::std::option::Option::Some(ActionResult::RequiresRepairTarget),
            92 => ::std::option::Option::Some(ActionResult::NoItemsToDrop),
            93 => ::std::option::Option::Some(ActionResult::CantHoldAnyMoreItems),
            94 => ::std::option::Option::Some(ActionResult::CantHoldThat),
            95 => ::std::option::Option::Some(ActionResult::TargetHasNoInventory),
            96 => ::std::option::Option::Some(ActionResult::CantDropThisItem),
            97 => ::std::option::Option::Some(ActionResult::CantMoveThisItem),
            98 => ::std::option::Option::Some(ActionResult::CantPawnThisUnit),
            99 => ::std::option::Option::Some(ActionResult::MustTargetCaster),
            100 => ::std::option::Option::Some(ActionResult::CantTargetCaster),
            101 => ::std::option::Option::Some(ActionResult::MustTargetOuter),
            102 => ::std::option::Option::Some(ActionResult::CantTargetOuter),
            103 => ::std::option::Option::Some(ActionResult::MustTargetYourOwnUnits),
            104 => ::std::option::Option::Some(ActionResult::CantTargetYourOwnUnits),
            105 => ::std::option::Option::Some(ActionResult::MustTargetFriendlyUnits),
            106 => ::std::option::Option::Some(ActionResult::CantTargetFriendlyUnits),
            107 => ::std::option::Option::Some(ActionResult::MustTargetNeutralUnits),
            108 => ::std::option::Option::Some(ActionResult::CantTargetNeutralUnits),
            109 => ::std::option::Option::Some(ActionResult::MustTargetEnemyUnits),
            110 => ::std::option::Option::Some(ActionResult::CantTargetEnemyUnits),
            111 => ::std::option::Option::Some(ActionResult::MustTargetAirUnits),
            112 => ::std::option::Option::Some(ActionResult::CantTargetAirUnits),
            113 => ::std::option::Option::Some(ActionResult::MustTargetGroundUnits),
            114 => ::std::option::Option::Some(ActionResult::CantTargetGroundUnits),
            115 => ::std::option::Option::Some(ActionResult::MustTargetStructures),
            116 => ::std::option::Option::Some(ActionResult::CantTargetStructures),
            117 => ::std::option::Option::Some(ActionResult::MustTargetLightUnits),
            118 => ::std::option::Option::Some(ActionResult::CantTargetLightUnits),
            119 => ::std::option::Option::Some(ActionResult::MustTargetArmoredUnits),
            120 => ::std::option::Option::Some(ActionResult::CantTargetArmoredUnits),
            121 => ::std::option::Option::Some(ActionResult::MustTargetBiologicalUnits),
            122 => ::std::option::Option::Some(ActionResult::CantTargetBiologicalUnits),
            123 => ::std::option::Option::Some(ActionResult::MustTargetHeroicUnits),
            124 => ::std::option::Option::Some(ActionResult::CantTargetHeroicUnits),
            125 => ::std::option::Option::Some(ActionResult::MustTargetRoboticUnits),
            126 => ::std::option::Option::Some(ActionResult::CantTargetRoboticUnits),
            127 => ::std::option::Option::Some(ActionResult::MustTargetMechanicalUnits),
            128 => ::std::option::Option::Some(ActionResult::CantTargetMechanicalUnits),
            129 => ::std::option::Option::Some(ActionResult::MustTargetPsionicUnits),
            130 => ::std::option::Option::Some(ActionResult::CantTargetPsionicUnits),
            131 => ::std::option::Option::Some(ActionResult::MustTargetMassiveUnits),
            132 => ::std::option::Option::Some(ActionResult::CantTargetMassiveUnits),
            133 => ::std::option::Option::Some(ActionResult::MustTargetMissile),
            134 => ::std::option::Option::Some(ActionResult::CantTargetMissile),
            135 => ::std::option::Option::Some(ActionResult::MustTargetWorkerUnits),
            136 => ::std::option::Option::Some(ActionResult::CantTargetWorkerUnits),
            137 => ::std::option::Option::Some(ActionResult::MustTargetEnergyCapableUnits),
            138 => ::std::option::Option::Some(ActionResult::CantTargetEnergyCapableUnits),
            139 => ::std::option::Option::Some(ActionResult::MustTargetShieldCapableUnits),
            140 => ::std::option::Option::Some(ActionResult::CantTargetShieldCapableUnits),
            141 => ::std::option::Option::Some(ActionResult::MustTargetFlyers),
            142 => ::std::option::Option::Some(ActionResult::CantTargetFlyers),
            143 => ::std::option::Option::Some(ActionResult::MustTargetBuriedUnits),
            144 => ::std::option::Option::Some(ActionResult::CantTargetBuriedUnits),
            145 => ::std::option::Option::Some(ActionResult::MustTargetCloakedUnits),
            146 => ::std::option::Option::Some(ActionResult::CantTargetCloakedUnits),
            147 => ::std::option::Option::Some(ActionResult::MustTargetUnitsInAStasisField),
            148 => ::std::option::Option::Some(ActionResult::CantTargetUnitsInAStasisField),
            149 => ::std::option::Option::Some(ActionResult::MustTargetUnderConstructionUnits),
            150 => ::std::option::Option::Some(ActionResult::CantTargetUnderConstructionUnits),
            151 => ::std::option::Option::Some(ActionResult::MustTargetDeadUnits),
            152 => ::std::option::Option::Some(ActionResult::CantTargetDeadUnits),
            153 => ::std::option::Option::Some(ActionResult::MustTargetRevivableUnits),
            154 => ::std::option::Option::Some(ActionResult::CantTargetRevivableUnits),
            155 => ::std::option::Option::Some(ActionResult::MustTargetHiddenUnits),
            156 => ::std::option::Option::Some(ActionResult::CantTargetHiddenUnits),
            157 => ::std::option::Option::Some(ActionResult::CantRechargeOtherPlayersUnits),
            158 => ::std::option::Option::Some(ActionResult::MustTargetHallucinations),
            159 => ::std::option::Option::Some(ActionResult::CantTargetHallucinations),
            160 => ::std::option::Option::Some(ActionResult::MustTargetInvulnerableUnits),
            161 => ::std::option::Option::Some(ActionResult::CantTargetInvulnerableUnits),
            162 => ::std::option::Option::Some(ActionResult::MustTargetDetectedUnits),
            163 => ::std::option::Option::Some(ActionResult::CantTargetDetectedUnits),
            164 => ::std::option::Option::Some(ActionResult::CantTargetUnitWithEnergy),
            165 => ::std::option::Option::Some(ActionResult::CantTargetUnitWithShields),
            166 => ::std::option::Option::Some(ActionResult::MustTargetUncommandableUnits),
            167 => ::std::option::Option::Some(ActionResult::CantTargetUncommandableUnits),
            168 => ::std::option::Option::Some(ActionResult::MustTargetPreventDefeatUnits),
            169 => ::std::option::Option::Some(ActionResult::CantTargetPreventDefeatUnits),
            170 => ::std::option::Option::Some(ActionResult::MustTargetPreventRevealUnits),
            171 => ::std::option::Option::Some(ActionResult::CantTargetPreventRevealUnits),
            172 => ::std::option::Option::Some(ActionResult::MustTargetPassiveUnits),
            173 => ::std::option::Option::Some(ActionResult::CantTargetPassiveUnits),
            174 => ::std::option::Option::Some(ActionResult::MustTargetStunnedUnits),
            175 => ::std::option::Option::Some(ActionResult::CantTargetStunnedUnits),
            176 => ::std::option::Option::Some(ActionResult::MustTargetSummonedUnits),
            177 => ::std::option::Option::Some(ActionResult::CantTargetSummonedUnits),
            178 => ::std::option::Option::Some(ActionResult::MustTargetUser1),
            179 => ::std::option::Option::Some(ActionResult::CantTargetUser1),
            180 => ::std::option::Option::Some(ActionResult::MustTargetUnstoppableUnits),
            181 => ::std::option::Option::Some(ActionResult::CantTargetUnstoppableUnits),
            182 => ::std::option::Option::Some(ActionResult::MustTargetResistantUnits),
            183 => ::std::option::Option::Some(ActionResult::CantTargetResistantUnits),
            184 => ::std::option::Option::Some(ActionResult::MustTargetDazedUnits),
            185 => ::std::option::Option::Some(ActionResult::CantTargetDazedUnits),
            186 => ::std::option::Option::Some(ActionResult::CantLockdown),
            187 => ::std::option::Option::Some(ActionResult::CantMindControl),
            188 => ::std::option::Option::Some(ActionResult::MustTargetDestructibles),
            189 => ::std::option::Option::Some(ActionResult::CantTargetDestructibles),
            190 => ::std::option::Option::Some(ActionResult::MustTargetItems),
            191 => ::std::option::Option::Some(ActionResult::CantTargetItems),
            192 => ::std::option::Option::Some(ActionResult::NoCalldownAvailable),
            193 => ::std::option::Option::Some(ActionResult::WaypointListFull),
            194 => ::std::option::Option::Some(ActionResult::MustTargetRace),
            195 => ::std::option::Option::Some(ActionResult::CantTargetRace),
            196 => ::std::option::Option::Some(ActionResult::MustTargetSimilarUnits),
            197 => ::std::option::Option::Some(ActionResult::CantTargetSimilarUnits),
            198 => ::std::option::Option::Some(ActionResult::CantFindEnoughTargets),
            199 => ::std::option::Option::Some(ActionResult::AlreadySpawningLarva),
            200 => ::std::option::Option::Some(ActionResult::CantTargetExhaustedResources),
            201 => ::std::option::Option::Some(ActionResult::CantUseMinimap),
            202 => ::std::option::Option::Some(ActionResult::CantUseInfoPanel),
            203 => ::std::option::Option::Some(ActionResult::OrderQueueIsFull),
            204 => ::std::option::Option::Some(ActionResult::CantHarvestThatResource),
            205 => ::std::option::Option::Some(ActionResult::HarvestersNotRequired),
            206 => ::std::option::Option::Some(ActionResult::AlreadyTargeted),
            207 => ::std::option::Option::Some(ActionResult::CantAttackWeaponsDisabled),
            208 => ::std::option::Option::Some(ActionResult::CouldntReachTarget),
            209 => ::std::option::Option::Some(ActionResult::TargetIsOutOfRange),
            210 => ::std::option::Option::Some(ActionResult::TargetIsTooClose),
            211 => ::std::option::Option::Some(ActionResult::TargetIsOutOfArc),
            212 => ::std::option::Option::Some(ActionResult::CantFindTeleportLocation),
            213 => ::std::option::Option::Some(ActionResult::InvalidItemClass),
            214 => ::std::option::Option::Some(ActionResult::CantFindCancelOrder),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ActionResult] = &[
            ActionResult::Success,
            ActionResult::NotSupported,
            ActionResult::Error,
            ActionResult::CantQueueThatOrder,
            ActionResult::Retry,
            ActionResult::Cooldown,
            ActionResult::QueueIsFull,
            ActionResult::RallyQueueIsFull,
            ActionResult::NotEnoughMinerals,
            ActionResult::NotEnoughVespene,
            ActionResult::NotEnoughTerrazine,
            ActionResult::NotEnoughCustom,
            ActionResult::NotEnoughFood,
            ActionResult::FoodUsageImpossible,
            ActionResult::NotEnoughLife,
            ActionResult::NotEnoughShields,
            ActionResult::NotEnoughEnergy,
            ActionResult::LifeSuppressed,
            ActionResult::ShieldsSuppressed,
            ActionResult::EnergySuppressed,
            ActionResult::NotEnoughCharges,
            ActionResult::CantAddMoreCharges,
            ActionResult::TooMuchMinerals,
            ActionResult::TooMuchVespene,
            ActionResult::TooMuchTerrazine,
            ActionResult::TooMuchCustom,
            ActionResult::TooMuchFood,
            ActionResult::TooMuchLife,
            ActionResult::TooMuchShields,
            ActionResult::TooMuchEnergy,
            ActionResult::MustTargetUnitWithLife,
            ActionResult::MustTargetUnitWithShields,
            ActionResult::MustTargetUnitWithEnergy,
            ActionResult::CantTrade,
            ActionResult::CantSpend,
            ActionResult::CantTargetThatUnit,
            ActionResult::CouldntAllocateUnit,
            ActionResult::UnitCantMove,
            ActionResult::TransportIsHoldingPosition,
            ActionResult::BuildTechRequirementsNotMet,
            ActionResult::CantFindPlacementLocation,
            ActionResult::CantBuildOnThat,
            ActionResult::CantBuildTooCloseToDropOff,
            ActionResult::CantBuildLocationInvalid,
            ActionResult::CantSeeBuildLocation,
            ActionResult::CantBuildTooCloseToCreepSource,
            ActionResult::CantBuildTooCloseToResources,
            ActionResult::CantBuildTooFarFromWater,
            ActionResult::CantBuildTooFarFromCreepSource,
            ActionResult::CantBuildTooFarFromBuildPowerSource,
            ActionResult::CantBuildOnDenseTerrain,
            ActionResult::CantTrainTooFarFromTrainPowerSource,
            ActionResult::CantLandLocationInvalid,
            ActionResult::CantSeeLandLocation,
            ActionResult::CantLandTooCloseToCreepSource,
            ActionResult::CantLandTooCloseToResources,
            ActionResult::CantLandTooFarFromWater,
            ActionResult::CantLandTooFarFromCreepSource,
            ActionResult::CantLandTooFarFromBuildPowerSource,
            ActionResult::CantLandTooFarFromTrainPowerSource,
            ActionResult::CantLandOnDenseTerrain,
            ActionResult::AddOnTooFarFromBuilding,
            ActionResult::MustBuildRefineryFirst,
            ActionResult::BuildingIsUnderConstruction,
            ActionResult::CantFindDropOff,
            ActionResult::CantLoadOtherPlayersUnits,
            ActionResult::NotEnoughRoomToLoadUnit,
            ActionResult::CantUnloadUnitsThere,
            ActionResult::CantWarpInUnitsThere,
            ActionResult::CantLoadImmobileUnits,
            ActionResult::CantRechargeImmobileUnits,
            ActionResult::CantRechargeUnderConstructionUnits,
            ActionResult::CantLoadThatUnit,
            ActionResult::NoCargoToUnload,
            ActionResult::LoadAllNoTargetsFound,
            ActionResult::NotWhileOccupied,
            ActionResult::CantAttackWithoutAmmo,
            ActionResult::CantHoldAnyMoreAmmo,
            ActionResult::TechRequirementsNotMet,
            ActionResult::MustLockdownUnitFirst,
            ActionResult::MustTargetUnit,
            ActionResult::MustTargetInventory,
            ActionResult::MustTargetVisibleUnit,
            ActionResult::MustTargetVisibleLocation,
            ActionResult::MustTargetWalkableLocation,
            ActionResult::MustTargetPawnableUnit,
            ActionResult::YouCantControlThatUnit,
            ActionResult::YouCantIssueCommandsToThatUnit,
            ActionResult::MustTargetResources,
            ActionResult::RequiresHealTarget,
            ActionResult::RequiresRepairTarget,
            ActionResult::NoItemsToDrop,
            ActionResult::CantHoldAnyMoreItems,
            ActionResult::CantHoldThat,
            ActionResult::TargetHasNoInventory,
            ActionResult::CantDropThisItem,
            ActionResult::CantMoveThisItem,
            ActionResult::CantPawnThisUnit,
            ActionResult::MustTargetCaster,
            ActionResult::CantTargetCaster,
            ActionResult::MustTargetOuter,
            ActionResult::CantTargetOuter,
            ActionResult::MustTargetYourOwnUnits,
            ActionResult::CantTargetYourOwnUnits,
            ActionResult::MustTargetFriendlyUnits,
            ActionResult::CantTargetFriendlyUnits,
            ActionResult::MustTargetNeutralUnits,
            ActionResult::CantTargetNeutralUnits,
            ActionResult::MustTargetEnemyUnits,
            ActionResult::CantTargetEnemyUnits,
            ActionResult::MustTargetAirUnits,
            ActionResult::CantTargetAirUnits,
            ActionResult::MustTargetGroundUnits,
            ActionResult::CantTargetGroundUnits,
            ActionResult::MustTargetStructures,
            ActionResult::CantTargetStructures,
            ActionResult::MustTargetLightUnits,
            ActionResult::CantTargetLightUnits,
            ActionResult::MustTargetArmoredUnits,
            ActionResult::CantTargetArmoredUnits,
            ActionResult::MustTargetBiologicalUnits,
            ActionResult::CantTargetBiologicalUnits,
            ActionResult::MustTargetHeroicUnits,
            ActionResult::CantTargetHeroicUnits,
            ActionResult::MustTargetRoboticUnits,
            ActionResult::CantTargetRoboticUnits,
            ActionResult::MustTargetMechanicalUnits,
            ActionResult::CantTargetMechanicalUnits,
            ActionResult::MustTargetPsionicUnits,
            ActionResult::CantTargetPsionicUnits,
            ActionResult::MustTargetMassiveUnits,
            ActionResult::CantTargetMassiveUnits,
            ActionResult::MustTargetMissile,
            ActionResult::CantTargetMissile,
            ActionResult::MustTargetWorkerUnits,
            ActionResult::CantTargetWorkerUnits,
            ActionResult::MustTargetEnergyCapableUnits,
            ActionResult::CantTargetEnergyCapableUnits,
            ActionResult::MustTargetShieldCapableUnits,
            ActionResult::CantTargetShieldCapableUnits,
            ActionResult::MustTargetFlyers,
            ActionResult::CantTargetFlyers,
            ActionResult::MustTargetBuriedUnits,
            ActionResult::CantTargetBuriedUnits,
            ActionResult::MustTargetCloakedUnits,
            ActionResult::CantTargetCloakedUnits,
            ActionResult::MustTargetUnitsInAStasisField,
            ActionResult::CantTargetUnitsInAStasisField,
            ActionResult::MustTargetUnderConstructionUnits,
            ActionResult::CantTargetUnderConstructionUnits,
            ActionResult::MustTargetDeadUnits,
            ActionResult::CantTargetDeadUnits,
            ActionResult::MustTargetRevivableUnits,
            ActionResult::CantTargetRevivableUnits,
            ActionResult::MustTargetHiddenUnits,
            ActionResult::CantTargetHiddenUnits,
            ActionResult::CantRechargeOtherPlayersUnits,
            ActionResult::MustTargetHallucinations,
            ActionResult::CantTargetHallucinations,
            ActionResult::MustTargetInvulnerableUnits,
            ActionResult::CantTargetInvulnerableUnits,
            ActionResult::MustTargetDetectedUnits,
            ActionResult::CantTargetDetectedUnits,
            ActionResult::CantTargetUnitWithEnergy,
            ActionResult::CantTargetUnitWithShields,
            ActionResult::MustTargetUncommandableUnits,
            ActionResult::CantTargetUncommandableUnits,
            ActionResult::MustTargetPreventDefeatUnits,
            ActionResult::CantTargetPreventDefeatUnits,
            ActionResult::MustTargetPreventRevealUnits,
            ActionResult::CantTargetPreventRevealUnits,
            ActionResult::MustTargetPassiveUnits,
            ActionResult::CantTargetPassiveUnits,
            ActionResult::MustTargetStunnedUnits,
            ActionResult::CantTargetStunnedUnits,
            ActionResult::MustTargetSummonedUnits,
            ActionResult::CantTargetSummonedUnits,
            ActionResult::MustTargetUser1,
            ActionResult::CantTargetUser1,
            ActionResult::MustTargetUnstoppableUnits,
            ActionResult::CantTargetUnstoppableUnits,
            ActionResult::MustTargetResistantUnits,
            ActionResult::CantTargetResistantUnits,
            ActionResult::MustTargetDazedUnits,
            ActionResult::CantTargetDazedUnits,
            ActionResult::CantLockdown,
            ActionResult::CantMindControl,
            ActionResult::MustTargetDestructibles,
            ActionResult::CantTargetDestructibles,
            ActionResult::MustTargetItems,
            ActionResult::CantTargetItems,
            ActionResult::NoCalldownAvailable,
            ActionResult::WaypointListFull,
            ActionResult::MustTargetRace,
            ActionResult::CantTargetRace,
            ActionResult::MustTargetSimilarUnits,
            ActionResult::CantTargetSimilarUnits,
            ActionResult::CantFindEnoughTargets,
            ActionResult::AlreadySpawningLarva,
            ActionResult::CantTargetExhaustedResources,
            ActionResult::CantUseMinimap,
            ActionResult::CantUseInfoPanel,
            ActionResult::OrderQueueIsFull,
            ActionResult::CantHarvestThatResource,
            ActionResult::HarvestersNotRequired,
            ActionResult::AlreadyTargeted,
            ActionResult::CantAttackWeaponsDisabled,
            ActionResult::CouldntReachTarget,
            ActionResult::TargetIsOutOfRange,
            ActionResult::TargetIsTooClose,
            ActionResult::TargetIsOutOfArc,
            ActionResult::CantFindTeleportLocation,
            ActionResult::InvalidItemClass,
            ActionResult::CantFindCancelOrder,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ActionResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ActionResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ActionResult {
}

impl ::protobuf::reflect::ProtobufValue for ActionResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cs2clientprotocol/error.proto\x12\x0eSC2APIProtocol*\xa8-\n\x0cActi\
    onResult\x12\x0b\n\x07Success\x10\x01\x12\x10\n\x0cNotSupported\x10\x02\
    \x12\t\n\x05Error\x10\x03\x12\x16\n\x12CantQueueThatOrder\x10\x04\x12\t\
    \n\x05Retry\x10\x05\x12\x0c\n\x08Cooldown\x10\x06\x12\x0f\n\x0bQueueIsFu\
    ll\x10\x07\x12\x14\n\x10RallyQueueIsFull\x10\x08\x12\x15\n\x11NotEnoughM\
    inerals\x10\t\x12\x14\n\x10NotEnoughVespene\x10\n\x12\x16\n\x12NotEnough\
    Terrazine\x10\x0b\x12\x13\n\x0fNotEnoughCustom\x10\x0c\x12\x11\n\rNotEno\
    ughFood\x10\r\x12\x17\n\x13FoodUsageImpossible\x10\x0e\x12\x11\n\rNotEno\
    ughLife\x10\x0f\x12\x14\n\x10NotEnoughShields\x10\x10\x12\x13\n\x0fNotEn\
    oughEnergy\x10\x11\x12\x12\n\x0eLifeSuppressed\x10\x12\x12\x15\n\x11Shie\
    ldsSuppressed\x10\x13\x12\x14\n\x10EnergySuppressed\x10\x14\x12\x14\n\
    \x10NotEnoughCharges\x10\x15\x12\x16\n\x12CantAddMoreCharges\x10\x16\x12\
    \x13\n\x0fTooMuchMinerals\x10\x17\x12\x12\n\x0eTooMuchVespene\x10\x18\
    \x12\x14\n\x10TooMuchTerrazine\x10\x19\x12\x11\n\rTooMuchCustom\x10\x1a\
    \x12\x0f\n\x0bTooMuchFood\x10\x1b\x12\x0f\n\x0bTooMuchLife\x10\x1c\x12\
    \x12\n\x0eTooMuchShields\x10\x1d\x12\x11\n\rTooMuchEnergy\x10\x1e\x12\
    \x1a\n\x16MustTargetUnitWithLife\x10\x1f\x12\x1d\n\x19MustTargetUnitWith\
    Shields\x10\x20\x12\x1c\n\x18MustTargetUnitWithEnergy\x10!\x12\r\n\tCant\
    Trade\x10\"\x12\r\n\tCantSpend\x10#\x12\x16\n\x12CantTargetThatUnit\x10$\
    \x12\x17\n\x13CouldntAllocateUnit\x10%\x12\x10\n\x0cUnitCantMove\x10&\
    \x12\x1e\n\x1aTransportIsHoldingPosition\x10'\x12\x1f\n\x1bBuildTechRequ\
    irementsNotMet\x10(\x12\x1d\n\x19CantFindPlacementLocation\x10)\x12\x13\
    \n\x0fCantBuildOnThat\x10*\x12\x1e\n\x1aCantBuildTooCloseToDropOff\x10+\
    \x12\x1c\n\x18CantBuildLocationInvalid\x10,\x12\x18\n\x14CantSeeBuildLoc\
    ation\x10-\x12\"\n\x1eCantBuildTooCloseToCreepSource\x10.\x12\x20\n\x1cC\
    antBuildTooCloseToResources\x10/\x12\x1c\n\x18CantBuildTooFarFromWater\
    \x100\x12\"\n\x1eCantBuildTooFarFromCreepSource\x101\x12'\n#CantBuildToo\
    FarFromBuildPowerSource\x102\x12\x1b\n\x17CantBuildOnDenseTerrain\x103\
    \x12'\n#CantTrainTooFarFromTrainPowerSource\x104\x12\x1b\n\x17CantLandLo\
    cationInvalid\x105\x12\x17\n\x13CantSeeLandLocation\x106\x12!\n\x1dCantL\
    andTooCloseToCreepSource\x107\x12\x1f\n\x1bCantLandTooCloseToResources\
    \x108\x12\x1b\n\x17CantLandTooFarFromWater\x109\x12!\n\x1dCantLandTooFar\
    FromCreepSource\x10:\x12&\n\"CantLandTooFarFromBuildPowerSource\x10;\x12\
    &\n\"CantLandTooFarFromTrainPowerSource\x10<\x12\x1a\n\x16CantLandOnDens\
    eTerrain\x10=\x12\x1b\n\x17AddOnTooFarFromBuilding\x10>\x12\x1a\n\x16Mus\
    tBuildRefineryFirst\x10?\x12\x1f\n\x1bBuildingIsUnderConstruction\x10@\
    \x12\x13\n\x0fCantFindDropOff\x10A\x12\x1d\n\x19CantLoadOtherPlayersUnit\
    s\x10B\x12\x1b\n\x17NotEnoughRoomToLoadUnit\x10C\x12\x18\n\x14CantUnload\
    UnitsThere\x10D\x12\x18\n\x14CantWarpInUnitsThere\x10E\x12\x19\n\x15Cant\
    LoadImmobileUnits\x10F\x12\x1d\n\x19CantRechargeImmobileUnits\x10G\x12&\
    \n\"CantRechargeUnderConstructionUnits\x10H\x12\x14\n\x10CantLoadThatUni\
    t\x10I\x12\x13\n\x0fNoCargoToUnload\x10J\x12\x19\n\x15LoadAllNoTargetsFo\
    und\x10K\x12\x14\n\x10NotWhileOccupied\x10L\x12\x19\n\x15CantAttackWitho\
    utAmmo\x10M\x12\x17\n\x13CantHoldAnyMoreAmmo\x10N\x12\x1a\n\x16TechRequi\
    rementsNotMet\x10O\x12\x19\n\x15MustLockdownUnitFirst\x10P\x12\x12\n\x0e\
    MustTargetUnit\x10Q\x12\x17\n\x13MustTargetInventory\x10R\x12\x19\n\x15M\
    ustTargetVisibleUnit\x10S\x12\x1d\n\x19MustTargetVisibleLocation\x10T\
    \x12\x1e\n\x1aMustTargetWalkableLocation\x10U\x12\x1a\n\x16MustTargetPaw\
    nableUnit\x10V\x12\x1a\n\x16YouCantControlThatUnit\x10W\x12\"\n\x1eYouCa\
    ntIssueCommandsToThatUnit\x10X\x12\x17\n\x13MustTargetResources\x10Y\x12\
    \x16\n\x12RequiresHealTarget\x10Z\x12\x18\n\x14RequiresRepairTarget\x10[\
    \x12\x11\n\rNoItemsToDrop\x10\\\x12\x18\n\x14CantHoldAnyMoreItems\x10]\
    \x12\x10\n\x0cCantHoldThat\x10^\x12\x18\n\x14TargetHasNoInventory\x10_\
    \x12\x14\n\x10CantDropThisItem\x10`\x12\x14\n\x10CantMoveThisItem\x10a\
    \x12\x14\n\x10CantPawnThisUnit\x10b\x12\x14\n\x10MustTargetCaster\x10c\
    \x12\x14\n\x10CantTargetCaster\x10d\x12\x13\n\x0fMustTargetOuter\x10e\
    \x12\x13\n\x0fCantTargetOuter\x10f\x12\x1a\n\x16MustTargetYourOwnUnits\
    \x10g\x12\x1a\n\x16CantTargetYourOwnUnits\x10h\x12\x1b\n\x17MustTargetFr\
    iendlyUnits\x10i\x12\x1b\n\x17CantTargetFriendlyUnits\x10j\x12\x1a\n\x16\
    MustTargetNeutralUnits\x10k\x12\x1a\n\x16CantTargetNeutralUnits\x10l\x12\
    \x18\n\x14MustTargetEnemyUnits\x10m\x12\x18\n\x14CantTargetEnemyUnits\
    \x10n\x12\x16\n\x12MustTargetAirUnits\x10o\x12\x16\n\x12CantTargetAirUni\
    ts\x10p\x12\x19\n\x15MustTargetGroundUnits\x10q\x12\x19\n\x15CantTargetG\
    roundUnits\x10r\x12\x18\n\x14MustTargetStructures\x10s\x12\x18\n\x14Cant\
    TargetStructures\x10t\x12\x18\n\x14MustTargetLightUnits\x10u\x12\x18\n\
    \x14CantTargetLightUnits\x10v\x12\x1a\n\x16MustTargetArmoredUnits\x10w\
    \x12\x1a\n\x16CantTargetArmoredUnits\x10x\x12\x1d\n\x19MustTargetBiologi\
    calUnits\x10y\x12\x1d\n\x19CantTargetBiologicalUnits\x10z\x12\x19\n\x15M\
    ustTargetHeroicUnits\x10{\x12\x19\n\x15CantTargetHeroicUnits\x10|\x12\
    \x1a\n\x16MustTargetRoboticUnits\x10}\x12\x1a\n\x16CantTargetRoboticUnit\
    s\x10~\x12\x1d\n\x19MustTargetMechanicalUnits\x10\x7f\x12\x1e\n\x19CantT\
    argetMechanicalUnits\x10\x80\x01\x12\x1b\n\x16MustTargetPsionicUnits\x10\
    \x81\x01\x12\x1b\n\x16CantTargetPsionicUnits\x10\x82\x01\x12\x1b\n\x16Mu\
    stTargetMassiveUnits\x10\x83\x01\x12\x1b\n\x16CantTargetMassiveUnits\x10\
    \x84\x01\x12\x16\n\x11MustTargetMissile\x10\x85\x01\x12\x16\n\x11CantTar\
    getMissile\x10\x86\x01\x12\x1a\n\x15MustTargetWorkerUnits\x10\x87\x01\
    \x12\x1a\n\x15CantTargetWorkerUnits\x10\x88\x01\x12!\n\x1cMustTargetEner\
    gyCapableUnits\x10\x89\x01\x12!\n\x1cCantTargetEnergyCapableUnits\x10\
    \x8a\x01\x12!\n\x1cMustTargetShieldCapableUnits\x10\x8b\x01\x12!\n\x1cCa\
    ntTargetShieldCapableUnits\x10\x8c\x01\x12\x15\n\x10MustTargetFlyers\x10\
    \x8d\x01\x12\x15\n\x10CantTargetFlyers\x10\x8e\x01\x12\x1a\n\x15MustTarg\
    etBuriedUnits\x10\x8f\x01\x12\x1a\n\x15CantTargetBuriedUnits\x10\x90\x01\
    \x12\x1b\n\x16MustTargetCloakedUnits\x10\x91\x01\x12\x1b\n\x16CantTarget\
    CloakedUnits\x10\x92\x01\x12\"\n\x1dMustTargetUnitsInAStasisField\x10\
    \x93\x01\x12\"\n\x1dCantTargetUnitsInAStasisField\x10\x94\x01\x12%\n\x20\
    MustTargetUnderConstructionUnits\x10\x95\x01\x12%\n\x20CantTargetUnderCo\
    nstructionUnits\x10\x96\x01\x12\x18\n\x13MustTargetDeadUnits\x10\x97\x01\
    \x12\x18\n\x13CantTargetDeadUnits\x10\x98\x01\x12\x1d\n\x18MustTargetRev\
    ivableUnits\x10\x99\x01\x12\x1d\n\x18CantTargetRevivableUnits\x10\x9a\
    \x01\x12\x1a\n\x15MustTargetHiddenUnits\x10\x9b\x01\x12\x1a\n\x15CantTar\
    getHiddenUnits\x10\x9c\x01\x12\"\n\x1dCantRechargeOtherPlayersUnits\x10\
    \x9d\x01\x12\x1d\n\x18MustTargetHallucinations\x10\x9e\x01\x12\x1d\n\x18\
    CantTargetHallucinations\x10\x9f\x01\x12\x20\n\x1bMustTargetInvulnerable\
    Units\x10\xa0\x01\x12\x20\n\x1bCantTargetInvulnerableUnits\x10\xa1\x01\
    \x12\x1c\n\x17MustTargetDetectedUnits\x10\xa2\x01\x12\x1c\n\x17CantTarge\
    tDetectedUnits\x10\xa3\x01\x12\x1d\n\x18CantTargetUnitWithEnergy\x10\xa4\
    \x01\x12\x1e\n\x19CantTargetUnitWithShields\x10\xa5\x01\x12!\n\x1cMustTa\
    rgetUncommandableUnits\x10\xa6\x01\x12!\n\x1cCantTargetUncommandableUnit\
    s\x10\xa7\x01\x12!\n\x1cMustTargetPreventDefeatUnits\x10\xa8\x01\x12!\n\
    \x1cCantTargetPreventDefeatUnits\x10\xa9\x01\x12!\n\x1cMustTargetPrevent\
    RevealUnits\x10\xaa\x01\x12!\n\x1cCantTargetPreventRevealUnits\x10\xab\
    \x01\x12\x1b\n\x16MustTargetPassiveUnits\x10\xac\x01\x12\x1b\n\x16CantTa\
    rgetPassiveUnits\x10\xad\x01\x12\x1b\n\x16MustTargetStunnedUnits\x10\xae\
    \x01\x12\x1b\n\x16CantTargetStunnedUnits\x10\xaf\x01\x12\x1c\n\x17MustTa\
    rgetSummonedUnits\x10\xb0\x01\x12\x1c\n\x17CantTargetSummonedUnits\x10\
    \xb1\x01\x12\x14\n\x0fMustTargetUser1\x10\xb2\x01\x12\x14\n\x0fCantTarge\
    tUser1\x10\xb3\x01\x12\x1f\n\x1aMustTargetUnstoppableUnits\x10\xb4\x01\
    \x12\x1f\n\x1aCantTargetUnstoppableUnits\x10\xb5\x01\x12\x1d\n\x18MustTa\
    rgetResistantUnits\x10\xb6\x01\x12\x1d\n\x18CantTargetResistantUnits\x10\
    \xb7\x01\x12\x19\n\x14MustTargetDazedUnits\x10\xb8\x01\x12\x19\n\x14Cant\
    TargetDazedUnits\x10\xb9\x01\x12\x11\n\x0cCantLockdown\x10\xba\x01\x12\
    \x14\n\x0fCantMindControl\x10\xbb\x01\x12\x1c\n\x17MustTargetDestructibl\
    es\x10\xbc\x01\x12\x1c\n\x17CantTargetDestructibles\x10\xbd\x01\x12\x14\
    \n\x0fMustTargetItems\x10\xbe\x01\x12\x14\n\x0fCantTargetItems\x10\xbf\
    \x01\x12\x18\n\x13NoCalldownAvailable\x10\xc0\x01\x12\x15\n\x10WaypointL\
    istFull\x10\xc1\x01\x12\x13\n\x0eMustTargetRace\x10\xc2\x01\x12\x13\n\
    \x0eCantTargetRace\x10\xc3\x01\x12\x1b\n\x16MustTargetSimilarUnits\x10\
    \xc4\x01\x12\x1b\n\x16CantTargetSimilarUnits\x10\xc5\x01\x12\x1a\n\x15Ca\
    ntFindEnoughTargets\x10\xc6\x01\x12\x19\n\x14AlreadySpawningLarva\x10\
    \xc7\x01\x12!\n\x1cCantTargetExhaustedResources\x10\xc8\x01\x12\x13\n\
    \x0eCantUseMinimap\x10\xc9\x01\x12\x15\n\x10CantUseInfoPanel\x10\xca\x01\
    \x12\x15\n\x10OrderQueueIsFull\x10\xcb\x01\x12\x1c\n\x17CantHarvestThatR\
    esource\x10\xcc\x01\x12\x1a\n\x15HarvestersNotRequired\x10\xcd\x01\x12\
    \x14\n\x0fAlreadyTargeted\x10\xce\x01\x12\x1e\n\x19CantAttackWeaponsDisa\
    bled\x10\xcf\x01\x12\x17\n\x12CouldntReachTarget\x10\xd0\x01\x12\x17\n\
    \x12TargetIsOutOfRange\x10\xd1\x01\x12\x15\n\x10TargetIsTooClose\x10\xd2\
    \x01\x12\x15\n\x10TargetIsOutOfArc\x10\xd3\x01\x12\x1d\n\x18CantFindTele\
    portLocation\x10\xd4\x01\x12\x15\n\x10InvalidItemClass\x10\xd5\x01\x12\
    \x18\n\x13CantFindCancelOrder\x10\xd6\x01J\x92I\n\x07\x12\x05\x01\0\xdc\
    \x01\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\x01\x02\x12\x03\x03\
    \x08\x16\n\x0b\n\x02\x05\0\x12\x05\x05\0\xdc\x01\x01\n\n\n\x03\x05\0\x01\
    \x12\x03\x05\x05\x11\n\x0b\n\x04\x05\0\x02\0\x12\x03\x06\x02\x0e\n\x0c\n\
    \x05\x05\0\x02\0\x01\x12\x03\x06\x02\t\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03\x06\x0c\r\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x07\x02\x13\n\x0c\n\x05\
    \x05\0\x02\x01\x01\x12\x03\x07\x02\x0e\n\x0c\n\x05\x05\0\x02\x01\x02\x12\
    \x03\x07\x11\x12\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x08\x02\x0c\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03\x08\x02\x07\n\x0c\n\x05\x05\0\x02\x02\x02\
    \x12\x03\x08\n\x0b\n\x0b\n\x04\x05\0\x02\x03\x12\x03\t\x02\x19\n\x0c\n\
    \x05\x05\0\x02\x03\x01\x12\x03\t\x02\x14\n\x0c\n\x05\x05\0\x02\x03\x02\
    \x12\x03\t\x17\x18\n\x0b\n\x04\x05\0\x02\x04\x12\x03\n\x02\x0c\n\x0c\n\
    \x05\x05\0\x02\x04\x01\x12\x03\n\x02\x07\n\x0c\n\x05\x05\0\x02\x04\x02\
    \x12\x03\n\n\x0b\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x0b\x02\x0f\n\x0c\n\
    \x05\x05\0\x02\x05\x01\x12\x03\x0b\x02\n\n\x0c\n\x05\x05\0\x02\x05\x02\
    \x12\x03\x0b\r\x0e\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x0c\x02\x12\n\x0c\n\
    \x05\x05\0\x02\x06\x01\x12\x03\x0c\x02\r\n\x0c\n\x05\x05\0\x02\x06\x02\
    \x12\x03\x0c\x10\x11\n\x0b\n\x04\x05\0\x02\x07\x12\x03\r\x02\x17\n\x0c\n\
    \x05\x05\0\x02\x07\x01\x12\x03\r\x02\x12\n\x0c\n\x05\x05\0\x02\x07\x02\
    \x12\x03\r\x15\x16\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x0e\x02\x18\n\x0c\n\
    \x05\x05\0\x02\x08\x01\x12\x03\x0e\x02\x13\n\x0c\n\x05\x05\0\x02\x08\x02\
    \x12\x03\x0e\x16\x17\n\x0b\n\x04\x05\0\x02\t\x12\x03\x0f\x02\x18\n\x0c\n\
    \x05\x05\0\x02\t\x01\x12\x03\x0f\x02\x12\n\x0c\n\x05\x05\0\x02\t\x02\x12\
    \x03\x0f\x15\x17\n\x0b\n\x04\x05\0\x02\n\x12\x03\x10\x02\x1a\n\x0c\n\x05\
    \x05\0\x02\n\x01\x12\x03\x10\x02\x14\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\
    \x10\x17\x19\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x11\x02\x17\n\x0c\n\x05\
    \x05\0\x02\x0b\x01\x12\x03\x11\x02\x11\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\
    \x03\x11\x14\x16\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\x12\x02\x15\n\x0c\n\
    \x05\x05\0\x02\x0c\x01\x12\x03\x12\x02\x0f\n\x0c\n\x05\x05\0\x02\x0c\x02\
    \x12\x03\x12\x12\x14\n\x0b\n\x04\x05\0\x02\r\x12\x03\x13\x02\x1b\n\x0c\n\
    \x05\x05\0\x02\r\x01\x12\x03\x13\x02\x15\n\x0c\n\x05\x05\0\x02\r\x02\x12\
    \x03\x13\x18\x1a\n\x0b\n\x04\x05\0\x02\x0e\x12\x03\x14\x02\x15\n\x0c\n\
    \x05\x05\0\x02\x0e\x01\x12\x03\x14\x02\x0f\n\x0c\n\x05\x05\0\x02\x0e\x02\
    \x12\x03\x14\x12\x14\n\x0b\n\x04\x05\0\x02\x0f\x12\x03\x15\x02\x18\n\x0c\
    \n\x05\x05\0\x02\x0f\x01\x12\x03\x15\x02\x12\n\x0c\n\x05\x05\0\x02\x0f\
    \x02\x12\x03\x15\x15\x17\n\x0b\n\x04\x05\0\x02\x10\x12\x03\x16\x02\x17\n\
    \x0c\n\x05\x05\0\x02\x10\x01\x12\x03\x16\x02\x11\n\x0c\n\x05\x05\0\x02\
    \x10\x02\x12\x03\x16\x14\x16\n\x0b\n\x04\x05\0\x02\x11\x12\x03\x17\x02\
    \x16\n\x0c\n\x05\x05\0\x02\x11\x01\x12\x03\x17\x02\x10\n\x0c\n\x05\x05\0\
    \x02\x11\x02\x12\x03\x17\x13\x15\n\x0b\n\x04\x05\0\x02\x12\x12\x03\x18\
    \x02\x19\n\x0c\n\x05\x05\0\x02\x12\x01\x12\x03\x18\x02\x13\n\x0c\n\x05\
    \x05\0\x02\x12\x02\x12\x03\x18\x16\x18\n\x0b\n\x04\x05\0\x02\x13\x12\x03\
    \x19\x02\x18\n\x0c\n\x05\x05\0\x02\x13\x01\x12\x03\x19\x02\x12\n\x0c\n\
    \x05\x05\0\x02\x13\x02\x12\x03\x19\x15\x17\n\x0b\n\x04\x05\0\x02\x14\x12\
    \x03\x1a\x02\x18\n\x0c\n\x05\x05\0\x02\x14\x01\x12\x03\x1a\x02\x12\n\x0c\
    \n\x05\x05\0\x02\x14\x02\x12\x03\x1a\x15\x17\n\x0b\n\x04\x05\0\x02\x15\
    \x12\x03\x1b\x02\x1a\n\x0c\n\x05\x05\0\x02\x15\x01\x12\x03\x1b\x02\x14\n\
    \x0c\n\x05\x05\0\x02\x15\x02\x12\x03\x1b\x17\x19\n\x0b\n\x04\x05\0\x02\
    \x16\x12\x03\x1c\x02\x17\n\x0c\n\x05\x05\0\x02\x16\x01\x12\x03\x1c\x02\
    \x11\n\x0c\n\x05\x05\0\x02\x16\x02\x12\x03\x1c\x14\x16\n\x0b\n\x04\x05\0\
    \x02\x17\x12\x03\x1d\x02\x16\n\x0c\n\x05\x05\0\x02\x17\x01\x12\x03\x1d\
    \x02\x10\n\x0c\n\x05\x05\0\x02\x17\x02\x12\x03\x1d\x13\x15\n\x0b\n\x04\
    \x05\0\x02\x18\x12\x03\x1e\x02\x18\n\x0c\n\x05\x05\0\x02\x18\x01\x12\x03\
    \x1e\x02\x12\n\x0c\n\x05\x05\0\x02\x18\x02\x12\x03\x1e\x15\x17\n\x0b\n\
    \x04\x05\0\x02\x19\x12\x03\x1f\x02\x15\n\x0c\n\x05\x05\0\x02\x19\x01\x12\
    \x03\x1f\x02\x0f\n\x0c\n\x05\x05\0\x02\x19\x02\x12\x03\x1f\x12\x14\n\x0b\
    \n\x04\x05\0\x02\x1a\x12\x03\x20\x02\x13\n\x0c\n\x05\x05\0\x02\x1a\x01\
    \x12\x03\x20\x02\r\n\x0c\n\x05\x05\0\x02\x1a\x02\x12\x03\x20\x10\x12\n\
    \x0b\n\x04\x05\0\x02\x1b\x12\x03!\x02\x13\n\x0c\n\x05\x05\0\x02\x1b\x01\
    \x12\x03!\x02\r\n\x0c\n\x05\x05\0\x02\x1b\x02\x12\x03!\x10\x12\n\x0b\n\
    \x04\x05\0\x02\x1c\x12\x03\"\x02\x16\n\x0c\n\x05\x05\0\x02\x1c\x01\x12\
    \x03\"\x02\x10\n\x0c\n\x05\x05\0\x02\x1c\x02\x12\x03\"\x13\x15\n\x0b\n\
    \x04\x05\0\x02\x1d\x12\x03#\x02\x15\n\x0c\n\x05\x05\0\x02\x1d\x01\x12\
    \x03#\x02\x0f\n\x0c\n\x05\x05\0\x02\x1d\x02\x12\x03#\x12\x14\n\x0b\n\x04\
    \x05\0\x02\x1e\x12\x03$\x02\x1e\n\x0c\n\x05\x05\0\x02\x1e\x01\x12\x03$\
    \x02\x18\n\x0c\n\x05\x05\0\x02\x1e\x02\x12\x03$\x1b\x1d\n\x0b\n\x04\x05\
    \0\x02\x1f\x12\x03%\x02!\n\x0c\n\x05\x05\0\x02\x1f\x01\x12\x03%\x02\x1b\
    \n\x0c\n\x05\x05\0\x02\x1f\x02\x12\x03%\x1e\x20\n\x0b\n\x04\x05\0\x02\
    \x20\x12\x03&\x02\x20\n\x0c\n\x05\x05\0\x02\x20\x01\x12\x03&\x02\x1a\n\
    \x0c\n\x05\x05\0\x02\x20\x02\x12\x03&\x1d\x1f\n\x0b\n\x04\x05\0\x02!\x12\
    \x03'\x02\x11\n\x0c\n\x05\x05\0\x02!\x01\x12\x03'\x02\x0b\n\x0c\n\x05\
    \x05\0\x02!\x02\x12\x03'\x0e\x10\n\x0b\n\x04\x05\0\x02\"\x12\x03(\x02\
    \x11\n\x0c\n\x05\x05\0\x02\"\x01\x12\x03(\x02\x0b\n\x0c\n\x05\x05\0\x02\
    \"\x02\x12\x03(\x0e\x10\n\x0b\n\x04\x05\0\x02#\x12\x03)\x02\x1a\n\x0c\n\
    \x05\x05\0\x02#\x01\x12\x03)\x02\x14\n\x0c\n\x05\x05\0\x02#\x02\x12\x03)\
    \x17\x19\n\x0b\n\x04\x05\0\x02$\x12\x03*\x02\x1b\n\x0c\n\x05\x05\0\x02$\
    \x01\x12\x03*\x02\x15\n\x0c\n\x05\x05\0\x02$\x02\x12\x03*\x18\x1a\n\x0b\
    \n\x04\x05\0\x02%\x12\x03+\x02\x14\n\x0c\n\x05\x05\0\x02%\x01\x12\x03+\
    \x02\x0e\n\x0c\n\x05\x05\0\x02%\x02\x12\x03+\x11\x13\n\x0b\n\x04\x05\0\
    \x02&\x12\x03,\x02\"\n\x0c\n\x05\x05\0\x02&\x01\x12\x03,\x02\x1c\n\x0c\n\
    \x05\x05\0\x02&\x02\x12\x03,\x1f!\n\x0b\n\x04\x05\0\x02'\x12\x03-\x02#\n\
    \x0c\n\x05\x05\0\x02'\x01\x12\x03-\x02\x1d\n\x0c\n\x05\x05\0\x02'\x02\
    \x12\x03-\x20\"\n\x0b\n\x04\x05\0\x02(\x12\x03.\x02!\n\x0c\n\x05\x05\0\
    \x02(\x01\x12\x03.\x02\x1b\n\x0c\n\x05\x05\0\x02(\x02\x12\x03.\x1e\x20\n\
    \x0b\n\x04\x05\0\x02)\x12\x03/\x02\x17\n\x0c\n\x05\x05\0\x02)\x01\x12\
    \x03/\x02\x11\n\x0c\n\x05\x05\0\x02)\x02\x12\x03/\x14\x16\n\x0b\n\x04\
    \x05\0\x02*\x12\x030\x02\"\n\x0c\n\x05\x05\0\x02*\x01\x12\x030\x02\x1c\n\
    \x0c\n\x05\x05\0\x02*\x02\x12\x030\x1f!\n\x0b\n\x04\x05\0\x02+\x12\x031\
    \x02\x20\n\x0c\n\x05\x05\0\x02+\x01\x12\x031\x02\x1a\n\x0c\n\x05\x05\0\
    \x02+\x02\x12\x031\x1d\x1f\n\x0b\n\x04\x05\0\x02,\x12\x032\x02\x1c\n\x0c\
    \n\x05\x05\0\x02,\x01\x12\x032\x02\x16\n\x0c\n\x05\x05\0\x02,\x02\x12\
    \x032\x19\x1b\n\x0b\n\x04\x05\0\x02-\x12\x033\x02&\n\x0c\n\x05\x05\0\x02\
    -\x01\x12\x033\x02\x20\n\x0c\n\x05\x05\0\x02-\x02\x12\x033#%\n\x0b\n\x04\
    \x05\0\x02.\x12\x034\x02$\n\x0c\n\x05\x05\0\x02.\x01\x12\x034\x02\x1e\n\
    \x0c\n\x05\x05\0\x02.\x02\x12\x034!#\n\x0b\n\x04\x05\0\x02/\x12\x035\x02\
    \x20\n\x0c\n\x05\x05\0\x02/\x01\x12\x035\x02\x1a\n\x0c\n\x05\x05\0\x02/\
    \x02\x12\x035\x1d\x1f\n\x0b\n\x04\x05\0\x020\x12\x036\x02&\n\x0c\n\x05\
    \x05\0\x020\x01\x12\x036\x02\x20\n\x0c\n\x05\x05\0\x020\x02\x12\x036#%\n\
    \x0b\n\x04\x05\0\x021\x12\x037\x02+\n\x0c\n\x05\x05\0\x021\x01\x12\x037\
    \x02%\n\x0c\n\x05\x05\0\x021\x02\x12\x037(*\n\x0b\n\x04\x05\0\x022\x12\
    \x038\x02\x1f\n\x0c\n\x05\x05\0\x022\x01\x12\x038\x02\x19\n\x0c\n\x05\
    \x05\0\x022\x02\x12\x038\x1c\x1e\n\x0b\n\x04\x05\0\x023\x12\x039\x02+\n\
    \x0c\n\x05\x05\0\x023\x01\x12\x039\x02%\n\x0c\n\x05\x05\0\x023\x02\x12\
    \x039(*\n\x0b\n\x04\x05\0\x024\x12\x03:\x02\x1f\n\x0c\n\x05\x05\0\x024\
    \x01\x12\x03:\x02\x19\n\x0c\n\x05\x05\0\x024\x02\x12\x03:\x1c\x1e\n\x0b\
    \n\x04\x05\0\x025\x12\x03;\x02\x1b\n\x0c\n\x05\x05\0\x025\x01\x12\x03;\
    \x02\x15\n\x0c\n\x05\x05\0\x025\x02\x12\x03;\x18\x1a\n\x0b\n\x04\x05\0\
    \x026\x12\x03<\x02%\n\x0c\n\x05\x05\0\x026\x01\x12\x03<\x02\x1f\n\x0c\n\
    \x05\x05\0\x026\x02\x12\x03<\"$\n\x0b\n\x04\x05\0\x027\x12\x03=\x02#\n\
    \x0c\n\x05\x05\0\x027\x01\x12\x03=\x02\x1d\n\x0c\n\x05\x05\0\x027\x02\
    \x12\x03=\x20\"\n\x0b\n\x04\x05\0\x028\x12\x03>\x02\x1f\n\x0c\n\x05\x05\
    \0\x028\x01\x12\x03>\x02\x19\n\x0c\n\x05\x05\0\x028\x02\x12\x03>\x1c\x1e\
    \n\x0b\n\x04\x05\0\x029\x12\x03?\x02%\n\x0c\n\x05\x05\0\x029\x01\x12\x03\
    ?\x02\x1f\n\x0c\n\x05\x05\0\x029\x02\x12\x03?\"$\n\x0b\n\x04\x05\0\x02:\
    \x12\x03@\x02*\n\x0c\n\x05\x05\0\x02:\x01\x12\x03@\x02$\n\x0c\n\x05\x05\
    \0\x02:\x02\x12\x03@')\n\x0b\n\x04\x05\0\x02;\x12\x03A\x02*\n\x0c\n\x05\
    \x05\0\x02;\x01\x12\x03A\x02$\n\x0c\n\x05\x05\0\x02;\x02\x12\x03A')\n\
    \x0b\n\x04\x05\0\x02<\x12\x03B\x02\x1e\n\x0c\n\x05\x05\0\x02<\x01\x12\
    \x03B\x02\x18\n\x0c\n\x05\x05\0\x02<\x02\x12\x03B\x1b\x1d\n\x0b\n\x04\
    \x05\0\x02=\x12\x03C\x02\x1f\n\x0c\n\x05\x05\0\x02=\x01\x12\x03C\x02\x19\
    \n\x0c\n\x05\x05\0\x02=\x02\x12\x03C\x1c\x1e\n\x0b\n\x04\x05\0\x02>\x12\
    \x03D\x02\x1e\n\x0c\n\x05\x05\0\x02>\x01\x12\x03D\x02\x18\n\x0c\n\x05\
    \x05\0\x02>\x02\x12\x03D\x1b\x1d\n\x0b\n\x04\x05\0\x02?\x12\x03E\x02#\n\
    \x0c\n\x05\x05\0\x02?\x01\x12\x03E\x02\x1d\n\x0c\n\x05\x05\0\x02?\x02\
    \x12\x03E\x20\"\n\x0b\n\x04\x05\0\x02@\x12\x03F\x02\x17\n\x0c\n\x05\x05\
    \0\x02@\x01\x12\x03F\x02\x11\n\x0c\n\x05\x05\0\x02@\x02\x12\x03F\x14\x16\
    \n\x0b\n\x04\x05\0\x02A\x12\x03G\x02!\n\x0c\n\x05\x05\0\x02A\x01\x12\x03\
    G\x02\x1b\n\x0c\n\x05\x05\0\x02A\x02\x12\x03G\x1e\x20\n\x0b\n\x04\x05\0\
    \x02B\x12\x03H\x02\x1f\n\x0c\n\x05\x05\0\x02B\x01\x12\x03H\x02\x19\n\x0c\
    \n\x05\x05\0\x02B\x02\x12\x03H\x1c\x1e\n\x0b\n\x04\x05\0\x02C\x12\x03I\
    \x02\x1c\n\x0c\n\x05\x05\0\x02C\x01\x12\x03I\x02\x16\n\x0c\n\x05\x05\0\
    \x02C\x02\x12\x03I\x19\x1b\n\x0b\n\x04\x05\0\x02D\x12\x03J\x02\x1c\n\x0c\
    \n\x05\x05\0\x02D\x01\x12\x03J\x02\x16\n\x0c\n\x05\x05\0\x02D\x02\x12\
    \x03J\x19\x1b\n\x0b\n\x04\x05\0\x02E\x12\x03K\x02\x1d\n\x0c\n\x05\x05\0\
    \x02E\x01\x12\x03K\x02\x17\n\x0c\n\x05\x05\0\x02E\x02\x12\x03K\x1a\x1c\n\
    \x0b\n\x04\x05\0\x02F\x12\x03L\x02!\n\x0c\n\x05\x05\0\x02F\x01\x12\x03L\
    \x02\x1b\n\x0c\n\x05\x05\0\x02F\x02\x12\x03L\x1e\x20\n\x0b\n\x04\x05\0\
    \x02G\x12\x03M\x02*\n\x0c\n\x05\x05\0\x02G\x01\x12\x03M\x02$\n\x0c\n\x05\
    \x05\0\x02G\x02\x12\x03M')\n\x0b\n\x04\x05\0\x02H\x12\x03N\x02\x18\n\x0c\
    \n\x05\x05\0\x02H\x01\x12\x03N\x02\x12\n\x0c\n\x05\x05\0\x02H\x02\x12\
    \x03N\x15\x17\n\x0b\n\x04\x05\0\x02I\x12\x03O\x02\x17\n\x0c\n\x05\x05\0\
    \x02I\x01\x12\x03O\x02\x11\n\x0c\n\x05\x05\0\x02I\x02\x12\x03O\x14\x16\n\
    \x0b\n\x04\x05\0\x02J\x12\x03P\x02\x1d\n\x0c\n\x05\x05\0\x02J\x01\x12\
    \x03P\x02\x17\n\x0c\n\x05\x05\0\x02J\x02\x12\x03P\x1a\x1c\n\x0b\n\x04\
    \x05\0\x02K\x12\x03Q\x02\x18\n\x0c\n\x05\x05\0\x02K\x01\x12\x03Q\x02\x12\
    \n\x0c\n\x05\x05\0\x02K\x02\x12\x03Q\x15\x17\n\x0b\n\x04\x05\0\x02L\x12\
    \x03R\x02\x1d\n\x0c\n\x05\x05\0\x02L\x01\x12\x03R\x02\x17\n\x0c\n\x05\
    \x05\0\x02L\x02\x12\x03R\x1a\x1c\n\x0b\n\x04\x05\0\x02M\x12\x03S\x02\x1b\
    \n\x0c\n\x05\x05\0\x02M\x01\x12\x03S\x02\x15\n\x0c\n\x05\x05\0\x02M\x02\
    \x12\x03S\x18\x1a\n\x0b\n\x04\x05\0\x02N\x12\x03T\x02\x1e\n\x0c\n\x05\
    \x05\0\x02N\x01\x12\x03T\x02\x18\n\x0c\n\x05\x05\0\x02N\x02\x12\x03T\x1b\
    \x1d\n\x0b\n\x04\x05\0\x02O\x12\x03U\x02\x1d\n\x0c\n\x05\x05\0\x02O\x01\
    \x12\x03U\x02\x17\n\x0c\n\x05\x05\0\x02O\x02\x12\x03U\x1a\x1c\n\x0b\n\
    \x04\x05\0\x02P\x12\x03V\x02\x16\n\x0c\n\x05\x05\0\x02P\x01\x12\x03V\x02\
    \x10\n\x0c\n\x05\x05\0\x02P\x02\x12\x03V\x13\x15\n\x0b\n\x04\x05\0\x02Q\
    \x12\x03W\x02\x1b\n\x0c\n\x05\x05\0\x02Q\x01\x12\x03W\x02\x15\n\x0c\n\
    \x05\x05\0\x02Q\x02\x12\x03W\x18\x1a\n\x0b\n\x04\x05\0\x02R\x12\x03X\x02\
    \x1d\n\x0c\n\x05\x05\0\x02R\x01\x12\x03X\x02\x17\n\x0c\n\x05\x05\0\x02R\
    \x02\x12\x03X\x1a\x1c\n\x0b\n\x04\x05\0\x02S\x12\x03Y\x02!\n\x0c\n\x05\
    \x05\0\x02S\x01\x12\x03Y\x02\x1b\n\x0c\n\x05\x05\0\x02S\x02\x12\x03Y\x1e\
    \x20\n\x0b\n\x04\x05\0\x02T\x12\x03Z\x02\"\n\x0c\n\x05\x05\0\x02T\x01\
    \x12\x03Z\x02\x1c\n\x0c\n\x05\x05\0\x02T\x02\x12\x03Z\x1f!\n\x0b\n\x04\
    \x05\0\x02U\x12\x03[\x02\x1e\n\x0c\n\x05\x05\0\x02U\x01\x12\x03[\x02\x18\
    \n\x0c\n\x05\x05\0\x02U\x02\x12\x03[\x1b\x1d\n\x0b\n\x04\x05\0\x02V\x12\
    \x03\\\x02\x1e\n\x0c\n\x05\x05\0\x02V\x01\x12\x03\\\x02\x18\n\x0c\n\x05\
    \x05\0\x02V\x02\x12\x03\\\x1b\x1d\n\x0b\n\x04\x05\0\x02W\x12\x03]\x02&\n\
    \x0c\n\x05\x05\0\x02W\x01\x12\x03]\x02\x20\n\x0c\n\x05\x05\0\x02W\x02\
    \x12\x03]#%\n\x0b\n\x04\x05\0\x02X\x12\x03^\x02\x1b\n\x0c\n\x05\x05\0\
    \x02X\x01\x12\x03^\x02\x15\n\x0c\n\x05\x05\0\x02X\x02\x12\x03^\x18\x1a\n\
    \x0b\n\x04\x05\0\x02Y\x12\x03_\x02\x1a\n\x0c\n\x05\x05\0\x02Y\x01\x12\
    \x03_\x02\x14\n\x0c\n\x05\x05\0\x02Y\x02\x12\x03_\x17\x19\n\x0b\n\x04\
    \x05\0\x02Z\x12\x03`\x02\x1c\n\x0c\n\x05\x05\0\x02Z\x01\x12\x03`\x02\x16\
    \n\x0c\n\x05\x05\0\x02Z\x02\x12\x03`\x19\x1b\n\x0b\n\x04\x05\0\x02[\x12\
    \x03a\x02\x15\n\x0c\n\x05\x05\0\x02[\x01\x12\x03a\x02\x0f\n\x0c\n\x05\
    \x05\0\x02[\x02\x12\x03a\x12\x14\n\x0b\n\x04\x05\0\x02\\\x12\x03b\x02\
    \x1c\n\x0c\n\x05\x05\0\x02\\\x01\x12\x03b\x02\x16\n\x0c\n\x05\x05\0\x02\
    \\\x02\x12\x03b\x19\x1b\n\x0b\n\x04\x05\0\x02]\x12\x03c\x02\x14\n\x0c\n\
    \x05\x05\0\x02]\x01\x12\x03c\x02\x0e\n\x0c\n\x05\x05\0\x02]\x02\x12\x03c\
    \x11\x13\n\x0b\n\x04\x05\0\x02^\x12\x03d\x02\x1c\n\x0c\n\x05\x05\0\x02^\
    \x01\x12\x03d\x02\x16\n\x0c\n\x05\x05\0\x02^\x02\x12\x03d\x19\x1b\n\x0b\
    \n\x04\x05\0\x02_\x12\x03e\x02\x18\n\x0c\n\x05\x05\0\x02_\x01\x12\x03e\
    \x02\x12\n\x0c\n\x05\x05\0\x02_\x02\x12\x03e\x15\x17\n\x0b\n\x04\x05\0\
    \x02`\x12\x03f\x02\x18\n\x0c\n\x05\x05\0\x02`\x01\x12\x03f\x02\x12\n\x0c\
    \n\x05\x05\0\x02`\x02\x12\x03f\x15\x17\n\x0b\n\x04\x05\0\x02a\x12\x03g\
    \x02\x18\n\x0c\n\x05\x05\0\x02a\x01\x12\x03g\x02\x12\n\x0c\n\x05\x05\0\
    \x02a\x02\x12\x03g\x15\x17\n\x0b\n\x04\x05\0\x02b\x12\x03h\x02\x18\n\x0c\
    \n\x05\x05\0\x02b\x01\x12\x03h\x02\x12\n\x0c\n\x05\x05\0\x02b\x02\x12\
    \x03h\x15\x17\n\x0b\n\x04\x05\0\x02c\x12\x03i\x02\x19\n\x0c\n\x05\x05\0\
    \x02c\x01\x12\x03i\x02\x12\n\x0c\n\x05\x05\0\x02c\x02\x12\x03i\x15\x18\n\
    \x0b\n\x04\x05\0\x02d\x12\x03j\x02\x18\n\x0c\n\x05\x05\0\x02d\x01\x12\
    \x03j\x02\x11\n\x0c\n\x05\x05\0\x02d\x02\x12\x03j\x14\x17\n\x0b\n\x04\
    \x05\0\x02e\x12\x03k\x02\x18\n\x0c\n\x05\x05\0\x02e\x01\x12\x03k\x02\x11\
    \n\x0c\n\x05\x05\0\x02e\x02\x12\x03k\x14\x17\n\x0b\n\x04\x05\0\x02f\x12\
    \x03l\x02\x1f\n\x0c\n\x05\x05\0\x02f\x01\x12\x03l\x02\x18\n\x0c\n\x05\
    \x05\0\x02f\x02\x12\x03l\x1b\x1e\n\x0b\n\x04\x05\0\x02g\x12\x03m\x02\x1f\
    \n\x0c\n\x05\x05\0\x02g\x01\x12\x03m\x02\x18\n\x0c\n\x05\x05\0\x02g\x02\
    \x12\x03m\x1b\x1e\n\x0b\n\x04\x05\0\x02h\x12\x03n\x02\x20\n\x0c\n\x05\
    \x05\0\x02h\x01\x12\x03n\x02\x19\n\x0c\n\x05\x05\0\x02h\x02\x12\x03n\x1c\
    \x1f\n\x0b\n\x04\x05\0\x02i\x12\x03o\x02\x20\n\x0c\n\x05\x05\0\x02i\x01\
    \x12\x03o\x02\x19\n\x0c\n\x05\x05\0\x02i\x02\x12\x03o\x1c\x1f\n\x0b\n\
    \x04\x05\0\x02j\x12\x03p\x02\x1f\n\x0c\n\x05\x05\0\x02j\x01\x12\x03p\x02\
    \x18\n\x0c\n\x05\x05\0\x02j\x02\x12\x03p\x1b\x1e\n\x0b\n\x04\x05\0\x02k\
    \x12\x03q\x02\x1f\n\x0c\n\x05\x05\0\x02k\x01\x12\x03q\x02\x18\n\x0c\n\
    \x05\x05\0\x02k\x02\x12\x03q\x1b\x1e\n\x0b\n\x04\x05\0\x02l\x12\x03r\x02\
    \x1d\n\x0c\n\x05\x05\0\x02l\x01\x12\x03r\x02\x16\n\x0c\n\x05\x05\0\x02l\
    \x02\x12\x03r\x19\x1c\n\x0b\n\x04\x05\0\x02m\x12\x03s\x02\x1d\n\x0c\n\
    \x05\x05\0\x02m\x01\x12\x03s\x02\x16\n\x0c\n\x05\x05\0\x02m\x02\x12\x03s\
    \x19\x1c\n\x0b\n\x04\x05\0\x02n\x12\x03t\x02\x1b\n\x0c\n\x05\x05\0\x02n\
    \x01\x12\x03t\x02\x14\n\x0c\n\x05\x05\0\x02n\x02\x12\x03t\x17\x1a\n\x0b\
    \n\x04\x05\0\x02o\x12\x03u\x02\x1b\n\x0c\n\x05\x05\0\x02o\x01\x12\x03u\
    \x02\x14\n\x0c\n\x05\x05\0\x02o\x02\x12\x03u\x17\x1a\n\x0b\n\x04\x05\0\
    \x02p\x12\x03v\x02\x1e\n\x0c\n\x05\x05\0\x02p\x01\x12\x03v\x02\x17\n\x0c\
    \n\x05\x05\0\x02p\x02\x12\x03v\x1a\x1d\n\x0b\n\x04\x05\0\x02q\x12\x03w\
    \x02\x1e\n\x0c\n\x05\x05\0\x02q\x01\x12\x03w\x02\x17\n\x0c\n\x05\x05\0\
    \x02q\x02\x12\x03w\x1a\x1d\n\x0b\n\x04\x05\0\x02r\x12\x03x\x02\x1d\n\x0c\
    \n\x05\x05\0\x02r\x01\x12\x03x\x02\x16\n\x0c\n\x05\x05\0\x02r\x02\x12\
    \x03x\x19\x1c\n\x0b\n\x04\x05\0\x02s\x12\x03y\x02\x1d\n\x0c\n\x05\x05\0\
    \x02s\x01\x12\x03y\x02\x16\n\x0c\n\x05\x05\0\x02s\x02\x12\x03y\x19\x1c\n\
    \x0b\n\x04\x05\0\x02t\x12\x03z\x02\x1d\n\x0c\n\x05\x05\0\x02t\x01\x12\
    \x03z\x02\x16\n\x0c\n\x05\x05\0\x02t\x02\x12\x03z\x19\x1c\n\x0b\n\x04\
    \x05\0\x02u\x12\x03{\x02\x1d\n\x0c\n\x05\x05\0\x02u\x01\x12\x03{\x02\x16\
    \n\x0c\n\x05\x05\0\x02u\x02\x12\x03{\x19\x1c\n\x0b\n\x04\x05\0\x02v\x12\
    \x03|\x02\x1f\n\x0c\n\x05\x05\0\x02v\x01\x12\x03|\x02\x18\n\x0c\n\x05\
    \x05\0\x02v\x02\x12\x03|\x1b\x1e\n\x0b\n\x04\x05\0\x02w\x12\x03}\x02\x1f\
    \n\x0c\n\x05\x05\0\x02w\x01\x12\x03}\x02\x18\n\x0c\n\x05\x05\0\x02w\x02\
    \x12\x03}\x1b\x1e\n\x0b\n\x04\x05\0\x02x\x12\x03~\x02\"\n\x0c\n\x05\x05\
    \0\x02x\x01\x12\x03~\x02\x1b\n\x0c\n\x05\x05\0\x02x\x02\x12\x03~\x1e!\n\
    \x0b\n\x04\x05\0\x02y\x12\x03\x7f\x02\"\n\x0c\n\x05\x05\0\x02y\x01\x12\
    \x03\x7f\x02\x1b\n\x0c\n\x05\x05\0\x02y\x02\x12\x03\x7f\x1e!\n\x0c\n\x04\
    \x05\0\x02z\x12\x04\x80\x01\x02\x1e\n\r\n\x05\x05\0\x02z\x01\x12\x04\x80\
    \x01\x02\x17\n\r\n\x05\x05\0\x02z\x02\x12\x04\x80\x01\x1a\x1d\n\x0c\n\
    \x04\x05\0\x02{\x12\x04\x81\x01\x02\x1e\n\r\n\x05\x05\0\x02{\x01\x12\x04\
    \x81\x01\x02\x17\n\r\n\x05\x05\0\x02{\x02\x12\x04\x81\x01\x1a\x1d\n\x0c\
    \n\x04\x05\0\x02|\x12\x04\x82\x01\x02\x1f\n\r\n\x05\x05\0\x02|\x01\x12\
    \x04\x82\x01\x02\x18\n\r\n\x05\x05\0\x02|\x02\x12\x04\x82\x01\x1b\x1e\n\
    \x0c\n\x04\x05\0\x02}\x12\x04\x83\x01\x02\x1f\n\r\n\x05\x05\0\x02}\x01\
    \x12\x04\x83\x01\x02\x18\n\r\n\x05\x05\0\x02}\x02\x12\x04\x83\x01\x1b\
    \x1e\n\x0c\n\x04\x05\0\x02~\x12\x04\x84\x01\x02\"\n\r\n\x05\x05\0\x02~\
    \x01\x12\x04\x84\x01\x02\x1b\n\r\n\x05\x05\0\x02~\x02\x12\x04\x84\x01\
    \x1e!\n\x0c\n\x04\x05\0\x02\x7f\x12\x04\x85\x01\x02\"\n\r\n\x05\x05\0\
    \x02\x7f\x01\x12\x04\x85\x01\x02\x1b\n\r\n\x05\x05\0\x02\x7f\x02\x12\x04\
    \x85\x01\x1e!\n\r\n\x05\x05\0\x02\x80\x01\x12\x04\x86\x01\x02\x1f\n\x0e\
    \n\x06\x05\0\x02\x80\x01\x01\x12\x04\x86\x01\x02\x18\n\x0e\n\x06\x05\0\
    \x02\x80\x01\x02\x12\x04\x86\x01\x1b\x1e\n\r\n\x05\x05\0\x02\x81\x01\x12\
    \x04\x87\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\x81\x01\x01\x12\x04\x87\x01\
    \x02\x18\n\x0e\n\x06\x05\0\x02\x81\x01\x02\x12\x04\x87\x01\x1b\x1e\n\r\n\
    \x05\x05\0\x02\x82\x01\x12\x04\x88\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\x82\
    \x01\x01\x12\x04\x88\x01\x02\x18\n\x0e\n\x06\x05\0\x02\x82\x01\x02\x12\
    \x04\x88\x01\x1b\x1e\n\r\n\x05\x05\0\x02\x83\x01\x12\x04\x89\x01\x02\x1f\
    \n\x0e\n\x06\x05\0\x02\x83\x01\x01\x12\x04\x89\x01\x02\x18\n\x0e\n\x06\
    \x05\0\x02\x83\x01\x02\x12\x04\x89\x01\x1b\x1e\n\r\n\x05\x05\0\x02\x84\
    \x01\x12\x04\x8a\x01\x02\x1a\n\x0e\n\x06\x05\0\x02\x84\x01\x01\x12\x04\
    \x8a\x01\x02\x13\n\x0e\n\x06\x05\0\x02\x84\x01\x02\x12\x04\x8a\x01\x16\
    \x19\n\r\n\x05\x05\0\x02\x85\x01\x12\x04\x8b\x01\x02\x1a\n\x0e\n\x06\x05\
    \0\x02\x85\x01\x01\x12\x04\x8b\x01\x02\x13\n\x0e\n\x06\x05\0\x02\x85\x01\
    \x02\x12\x04\x8b\x01\x16\x19\n\r\n\x05\x05\0\x02\x86\x01\x12\x04\x8c\x01\
    \x02\x1e\n\x0e\n\x06\x05\0\x02\x86\x01\x01\x12\x04\x8c\x01\x02\x17\n\x0e\
    \n\x06\x05\0\x02\x86\x01\x02\x12\x04\x8c\x01\x1a\x1d\n\r\n\x05\x05\0\x02\
    \x87\x01\x12\x04\x8d\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\x87\x01\x01\x12\
    \x04\x8d\x01\x02\x17\n\x0e\n\x06\x05\0\x02\x87\x01\x02\x12\x04\x8d\x01\
    \x1a\x1d\n\r\n\x05\x05\0\x02\x88\x01\x12\x04\x8e\x01\x02%\n\x0e\n\x06\
    \x05\0\x02\x88\x01\x01\x12\x04\x8e\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\x88\
    \x01\x02\x12\x04\x8e\x01!$\n\r\n\x05\x05\0\x02\x89\x01\x12\x04\x8f\x01\
    \x02%\n\x0e\n\x06\x05\0\x02\x89\x01\x01\x12\x04\x8f\x01\x02\x1e\n\x0e\n\
    \x06\x05\0\x02\x89\x01\x02\x12\x04\x8f\x01!$\n\r\n\x05\x05\0\x02\x8a\x01\
    \x12\x04\x90\x01\x02%\n\x0e\n\x06\x05\0\x02\x8a\x01\x01\x12\x04\x90\x01\
    \x02\x1e\n\x0e\n\x06\x05\0\x02\x8a\x01\x02\x12\x04\x90\x01!$\n\r\n\x05\
    \x05\0\x02\x8b\x01\x12\x04\x91\x01\x02%\n\x0e\n\x06\x05\0\x02\x8b\x01\
    \x01\x12\x04\x91\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\x8b\x01\x02\x12\x04\
    \x91\x01!$\n\r\n\x05\x05\0\x02\x8c\x01\x12\x04\x92\x01\x02\x19\n\x0e\n\
    \x06\x05\0\x02\x8c\x01\x01\x12\x04\x92\x01\x02\x12\n\x0e\n\x06\x05\0\x02\
    \x8c\x01\x02\x12\x04\x92\x01\x15\x18\n\r\n\x05\x05\0\x02\x8d\x01\x12\x04\
    \x93\x01\x02\x19\n\x0e\n\x06\x05\0\x02\x8d\x01\x01\x12\x04\x93\x01\x02\
    \x12\n\x0e\n\x06\x05\0\x02\x8d\x01\x02\x12\x04\x93\x01\x15\x18\n\r\n\x05\
    \x05\0\x02\x8e\x01\x12\x04\x94\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\x8e\x01\
    \x01\x12\x04\x94\x01\x02\x17\n\x0e\n\x06\x05\0\x02\x8e\x01\x02\x12\x04\
    \x94\x01\x1a\x1d\n\r\n\x05\x05\0\x02\x8f\x01\x12\x04\x95\x01\x02\x1e\n\
    \x0e\n\x06\x05\0\x02\x8f\x01\x01\x12\x04\x95\x01\x02\x17\n\x0e\n\x06\x05\
    \0\x02\x8f\x01\x02\x12\x04\x95\x01\x1a\x1d\n\r\n\x05\x05\0\x02\x90\x01\
    \x12\x04\x96\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\x90\x01\x01\x12\x04\x96\
    \x01\x02\x18\n\x0e\n\x06\x05\0\x02\x90\x01\x02\x12\x04\x96\x01\x1b\x1e\n\
    \r\n\x05\x05\0\x02\x91\x01\x12\x04\x97\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\
    \x91\x01\x01\x12\x04\x97\x01\x02\x18\n\x0e\n\x06\x05\0\x02\x91\x01\x02\
    \x12\x04\x97\x01\x1b\x1e\n\r\n\x05\x05\0\x02\x92\x01\x12\x04\x98\x01\x02\
    &\n\x0e\n\x06\x05\0\x02\x92\x01\x01\x12\x04\x98\x01\x02\x1f\n\x0e\n\x06\
    \x05\0\x02\x92\x01\x02\x12\x04\x98\x01\"%\n\r\n\x05\x05\0\x02\x93\x01\
    \x12\x04\x99\x01\x02&\n\x0e\n\x06\x05\0\x02\x93\x01\x01\x12\x04\x99\x01\
    \x02\x1f\n\x0e\n\x06\x05\0\x02\x93\x01\x02\x12\x04\x99\x01\"%\n\r\n\x05\
    \x05\0\x02\x94\x01\x12\x04\x9a\x01\x02)\n\x0e\n\x06\x05\0\x02\x94\x01\
    \x01\x12\x04\x9a\x01\x02\"\n\x0e\n\x06\x05\0\x02\x94\x01\x02\x12\x04\x9a\
    \x01%(\n\r\n\x05\x05\0\x02\x95\x01\x12\x04\x9b\x01\x02)\n\x0e\n\x06\x05\
    \0\x02\x95\x01\x01\x12\x04\x9b\x01\x02\"\n\x0e\n\x06\x05\0\x02\x95\x01\
    \x02\x12\x04\x9b\x01%(\n\r\n\x05\x05\0\x02\x96\x01\x12\x04\x9c\x01\x02\
    \x1c\n\x0e\n\x06\x05\0\x02\x96\x01\x01\x12\x04\x9c\x01\x02\x15\n\x0e\n\
    \x06\x05\0\x02\x96\x01\x02\x12\x04\x9c\x01\x18\x1b\n\r\n\x05\x05\0\x02\
    \x97\x01\x12\x04\x9d\x01\x02\x1c\n\x0e\n\x06\x05\0\x02\x97\x01\x01\x12\
    \x04\x9d\x01\x02\x15\n\x0e\n\x06\x05\0\x02\x97\x01\x02\x12\x04\x9d\x01\
    \x18\x1b\n\r\n\x05\x05\0\x02\x98\x01\x12\x04\x9e\x01\x02!\n\x0e\n\x06\
    \x05\0\x02\x98\x01\x01\x12\x04\x9e\x01\x02\x1a\n\x0e\n\x06\x05\0\x02\x98\
    \x01\x02\x12\x04\x9e\x01\x1d\x20\n\r\n\x05\x05\0\x02\x99\x01\x12\x04\x9f\
    \x01\x02!\n\x0e\n\x06\x05\0\x02\x99\x01\x01\x12\x04\x9f\x01\x02\x1a\n\
    \x0e\n\x06\x05\0\x02\x99\x01\x02\x12\x04\x9f\x01\x1d\x20\n\r\n\x05\x05\0\
    \x02\x9a\x01\x12\x04\xa0\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\x9a\x01\x01\
    \x12\x04\xa0\x01\x02\x17\n\x0e\n\x06\x05\0\x02\x9a\x01\x02\x12\x04\xa0\
    \x01\x1a\x1d\n\r\n\x05\x05\0\x02\x9b\x01\x12\x04\xa1\x01\x02\x1e\n\x0e\n\
    \x06\x05\0\x02\x9b\x01\x01\x12\x04\xa1\x01\x02\x17\n\x0e\n\x06\x05\0\x02\
    \x9b\x01\x02\x12\x04\xa1\x01\x1a\x1d\n\r\n\x05\x05\0\x02\x9c\x01\x12\x04\
    \xa2\x01\x02&\n\x0e\n\x06\x05\0\x02\x9c\x01\x01\x12\x04\xa2\x01\x02\x1f\
    \n\x0e\n\x06\x05\0\x02\x9c\x01\x02\x12\x04\xa2\x01\"%\n\r\n\x05\x05\0\
    \x02\x9d\x01\x12\x04\xa3\x01\x02!\n\x0e\n\x06\x05\0\x02\x9d\x01\x01\x12\
    \x04\xa3\x01\x02\x1a\n\x0e\n\x06\x05\0\x02\x9d\x01\x02\x12\x04\xa3\x01\
    \x1d\x20\n\r\n\x05\x05\0\x02\x9e\x01\x12\x04\xa4\x01\x02!\n\x0e\n\x06\
    \x05\0\x02\x9e\x01\x01\x12\x04\xa4\x01\x02\x1a\n\x0e\n\x06\x05\0\x02\x9e\
    \x01\x02\x12\x04\xa4\x01\x1d\x20\n\r\n\x05\x05\0\x02\x9f\x01\x12\x04\xa5\
    \x01\x02$\n\x0e\n\x06\x05\0\x02\x9f\x01\x01\x12\x04\xa5\x01\x02\x1d\n\
    \x0e\n\x06\x05\0\x02\x9f\x01\x02\x12\x04\xa5\x01\x20#\n\r\n\x05\x05\0\
    \x02\xa0\x01\x12\x04\xa6\x01\x02$\n\x0e\n\x06\x05\0\x02\xa0\x01\x01\x12\
    \x04\xa6\x01\x02\x1d\n\x0e\n\x06\x05\0\x02\xa0\x01\x02\x12\x04\xa6\x01\
    \x20#\n\r\n\x05\x05\0\x02\xa1\x01\x12\x04\xa7\x01\x02\x20\n\x0e\n\x06\
    \x05\0\x02\xa1\x01\x01\x12\x04\xa7\x01\x02\x19\n\x0e\n\x06\x05\0\x02\xa1\
    \x01\x02\x12\x04\xa7\x01\x1c\x1f\n\r\n\x05\x05\0\x02\xa2\x01\x12\x04\xa8\
    \x01\x02\x20\n\x0e\n\x06\x05\0\x02\xa2\x01\x01\x12\x04\xa8\x01\x02\x19\n\
    \x0e\n\x06\x05\0\x02\xa2\x01\x02\x12\x04\xa8\x01\x1c\x1f\n\r\n\x05\x05\0\
    \x02\xa3\x01\x12\x04\xa9\x01\x02!\n\x0e\n\x06\x05\0\x02\xa3\x01\x01\x12\
    \x04\xa9\x01\x02\x1a\n\x0e\n\x06\x05\0\x02\xa3\x01\x02\x12\x04\xa9\x01\
    \x1d\x20\n\r\n\x05\x05\0\x02\xa4\x01\x12\x04\xaa\x01\x02\"\n\x0e\n\x06\
    \x05\0\x02\xa4\x01\x01\x12\x04\xaa\x01\x02\x1b\n\x0e\n\x06\x05\0\x02\xa4\
    \x01\x02\x12\x04\xaa\x01\x1e!\n\r\n\x05\x05\0\x02\xa5\x01\x12\x04\xab\
    \x01\x02%\n\x0e\n\x06\x05\0\x02\xa5\x01\x01\x12\x04\xab\x01\x02\x1e\n\
    \x0e\n\x06\x05\0\x02\xa5\x01\x02\x12\x04\xab\x01!$\n\r\n\x05\x05\0\x02\
    \xa6\x01\x12\x04\xac\x01\x02%\n\x0e\n\x06\x05\0\x02\xa6\x01\x01\x12\x04\
    \xac\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\xa6\x01\x02\x12\x04\xac\x01!$\n\r\
    \n\x05\x05\0\x02\xa7\x01\x12\x04\xad\x01\x02%\n\x0e\n\x06\x05\0\x02\xa7\
    \x01\x01\x12\x04\xad\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\xa7\x01\x02\x12\
    \x04\xad\x01!$\n\r\n\x05\x05\0\x02\xa8\x01\x12\x04\xae\x01\x02%\n\x0e\n\
    \x06\x05\0\x02\xa8\x01\x01\x12\x04\xae\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\
    \xa8\x01\x02\x12\x04\xae\x01!$\n\r\n\x05\x05\0\x02\xa9\x01\x12\x04\xaf\
    \x01\x02%\n\x0e\n\x06\x05\0\x02\xa9\x01\x01\x12\x04\xaf\x01\x02\x1e\n\
    \x0e\n\x06\x05\0\x02\xa9\x01\x02\x12\x04\xaf\x01!$\n\r\n\x05\x05\0\x02\
    \xaa\x01\x12\x04\xb0\x01\x02%\n\x0e\n\x06\x05\0\x02\xaa\x01\x01\x12\x04\
    \xb0\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\xaa\x01\x02\x12\x04\xb0\x01!$\n\r\
    \n\x05\x05\0\x02\xab\x01\x12\x04\xb1\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\
    \xab\x01\x01\x12\x04\xb1\x01\x02\x18\n\x0e\n\x06\x05\0\x02\xab\x01\x02\
    \x12\x04\xb1\x01\x1b\x1e\n\r\n\x05\x05\0\x02\xac\x01\x12\x04\xb2\x01\x02\
    \x1f\n\x0e\n\x06\x05\0\x02\xac\x01\x01\x12\x04\xb2\x01\x02\x18\n\x0e\n\
    \x06\x05\0\x02\xac\x01\x02\x12\x04\xb2\x01\x1b\x1e\n\r\n\x05\x05\0\x02\
    \xad\x01\x12\x04\xb3\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\xad\x01\x01\x12\
    \x04\xb3\x01\x02\x18\n\x0e\n\x06\x05\0\x02\xad\x01\x02\x12\x04\xb3\x01\
    \x1b\x1e\n\r\n\x05\x05\0\x02\xae\x01\x12\x04\xb4\x01\x02\x1f\n\x0e\n\x06\
    \x05\0\x02\xae\x01\x01\x12\x04\xb4\x01\x02\x18\n\x0e\n\x06\x05\0\x02\xae\
    \x01\x02\x12\x04\xb4\x01\x1b\x1e\n\r\n\x05\x05\0\x02\xaf\x01\x12\x04\xb5\
    \x01\x02\x20\n\x0e\n\x06\x05\0\x02\xaf\x01\x01\x12\x04\xb5\x01\x02\x19\n\
    \x0e\n\x06\x05\0\x02\xaf\x01\x02\x12\x04\xb5\x01\x1c\x1f\n\r\n\x05\x05\0\
    \x02\xb0\x01\x12\x04\xb6\x01\x02\x20\n\x0e\n\x06\x05\0\x02\xb0\x01\x01\
    \x12\x04\xb6\x01\x02\x19\n\x0e\n\x06\x05\0\x02\xb0\x01\x02\x12\x04\xb6\
    \x01\x1c\x1f\n\r\n\x05\x05\0\x02\xb1\x01\x12\x04\xb7\x01\x02\x18\n\x0e\n\
    \x06\x05\0\x02\xb1\x01\x01\x12\x04\xb7\x01\x02\x11\n\x0e\n\x06\x05\0\x02\
    \xb1\x01\x02\x12\x04\xb7\x01\x14\x17\n\r\n\x05\x05\0\x02\xb2\x01\x12\x04\
    \xb8\x01\x02\x18\n\x0e\n\x06\x05\0\x02\xb2\x01\x01\x12\x04\xb8\x01\x02\
    \x11\n\x0e\n\x06\x05\0\x02\xb2\x01\x02\x12\x04\xb8\x01\x14\x17\n\r\n\x05\
    \x05\0\x02\xb3\x01\x12\x04\xb9\x01\x02#\n\x0e\n\x06\x05\0\x02\xb3\x01\
    \x01\x12\x04\xb9\x01\x02\x1c\n\x0e\n\x06\x05\0\x02\xb3\x01\x02\x12\x04\
    \xb9\x01\x1f\"\n\r\n\x05\x05\0\x02\xb4\x01\x12\x04\xba\x01\x02#\n\x0e\n\
    \x06\x05\0\x02\xb4\x01\x01\x12\x04\xba\x01\x02\x1c\n\x0e\n\x06\x05\0\x02\
    \xb4\x01\x02\x12\x04\xba\x01\x1f\"\n\r\n\x05\x05\0\x02\xb5\x01\x12\x04\
    \xbb\x01\x02!\n\x0e\n\x06\x05\0\x02\xb5\x01\x01\x12\x04\xbb\x01\x02\x1a\
    \n\x0e\n\x06\x05\0\x02\xb5\x01\x02\x12\x04\xbb\x01\x1d\x20\n\r\n\x05\x05\
    \0\x02\xb6\x01\x12\x04\xbc\x01\x02!\n\x0e\n\x06\x05\0\x02\xb6\x01\x01\
    \x12\x04\xbc\x01\x02\x1a\n\x0e\n\x06\x05\0\x02\xb6\x01\x02\x12\x04\xbc\
    \x01\x1d\x20\n\r\n\x05\x05\0\x02\xb7\x01\x12\x04\xbd\x01\x02\x1d\n\x0e\n\
    \x06\x05\0\x02\xb7\x01\x01\x12\x04\xbd\x01\x02\x16\n\x0e\n\x06\x05\0\x02\
    \xb7\x01\x02\x12\x04\xbd\x01\x19\x1c\n\r\n\x05\x05\0\x02\xb8\x01\x12\x04\
    \xbe\x01\x02\x1d\n\x0e\n\x06\x05\0\x02\xb8\x01\x01\x12\x04\xbe\x01\x02\
    \x16\n\x0e\n\x06\x05\0\x02\xb8\x01\x02\x12\x04\xbe\x01\x19\x1c\n\r\n\x05\
    \x05\0\x02\xb9\x01\x12\x04\xbf\x01\x02\x15\n\x0e\n\x06\x05\0\x02\xb9\x01\
    \x01\x12\x04\xbf\x01\x02\x0e\n\x0e\n\x06\x05\0\x02\xb9\x01\x02\x12\x04\
    \xbf\x01\x11\x14\n\r\n\x05\x05\0\x02\xba\x01\x12\x04\xc0\x01\x02\x18\n\
    \x0e\n\x06\x05\0\x02\xba\x01\x01\x12\x04\xc0\x01\x02\x11\n\x0e\n\x06\x05\
    \0\x02\xba\x01\x02\x12\x04\xc0\x01\x14\x17\n\r\n\x05\x05\0\x02\xbb\x01\
    \x12\x04\xc1\x01\x02\x20\n\x0e\n\x06\x05\0\x02\xbb\x01\x01\x12\x04\xc1\
    \x01\x02\x19\n\x0e\n\x06\x05\0\x02\xbb\x01\x02\x12\x04\xc1\x01\x1c\x1f\n\
    \r\n\x05\x05\0\x02\xbc\x01\x12\x04\xc2\x01\x02\x20\n\x0e\n\x06\x05\0\x02\
    \xbc\x01\x01\x12\x04\xc2\x01\x02\x19\n\x0e\n\x06\x05\0\x02\xbc\x01\x02\
    \x12\x04\xc2\x01\x1c\x1f\n\r\n\x05\x05\0\x02\xbd\x01\x12\x04\xc3\x01\x02\
    \x18\n\x0e\n\x06\x05\0\x02\xbd\x01\x01\x12\x04\xc3\x01\x02\x11\n\x0e\n\
    \x06\x05\0\x02\xbd\x01\x02\x12\x04\xc3\x01\x14\x17\n\r\n\x05\x05\0\x02\
    \xbe\x01\x12\x04\xc4\x01\x02\x18\n\x0e\n\x06\x05\0\x02\xbe\x01\x01\x12\
    \x04\xc4\x01\x02\x11\n\x0e\n\x06\x05\0\x02\xbe\x01\x02\x12\x04\xc4\x01\
    \x14\x17\n\r\n\x05\x05\0\x02\xbf\x01\x12\x04\xc5\x01\x02\x1c\n\x0e\n\x06\
    \x05\0\x02\xbf\x01\x01\x12\x04\xc5\x01\x02\x15\n\x0e\n\x06\x05\0\x02\xbf\
    \x01\x02\x12\x04\xc5\x01\x18\x1b\n\r\n\x05\x05\0\x02\xc0\x01\x12\x04\xc6\
    \x01\x02\x19\n\x0e\n\x06\x05\0\x02\xc0\x01\x01\x12\x04\xc6\x01\x02\x12\n\
    \x0e\n\x06\x05\0\x02\xc0\x01\x02\x12\x04\xc6\x01\x15\x18\n\r\n\x05\x05\0\
    \x02\xc1\x01\x12\x04\xc7\x01\x02\x17\n\x0e\n\x06\x05\0\x02\xc1\x01\x01\
    \x12\x04\xc7\x01\x02\x10\n\x0e\n\x06\x05\0\x02\xc1\x01\x02\x12\x04\xc7\
    \x01\x13\x16\n\r\n\x05\x05\0\x02\xc2\x01\x12\x04\xc8\x01\x02\x17\n\x0e\n\
    \x06\x05\0\x02\xc2\x01\x01\x12\x04\xc8\x01\x02\x10\n\x0e\n\x06\x05\0\x02\
    \xc2\x01\x02\x12\x04\xc8\x01\x13\x16\n\r\n\x05\x05\0\x02\xc3\x01\x12\x04\
    \xc9\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\xc3\x01\x01\x12\x04\xc9\x01\x02\
    \x18\n\x0e\n\x06\x05\0\x02\xc3\x01\x02\x12\x04\xc9\x01\x1b\x1e\n\r\n\x05\
    \x05\0\x02\xc4\x01\x12\x04\xca\x01\x02\x1f\n\x0e\n\x06\x05\0\x02\xc4\x01\
    \x01\x12\x04\xca\x01\x02\x18\n\x0e\n\x06\x05\0\x02\xc4\x01\x02\x12\x04\
    \xca\x01\x1b\x1e\n\r\n\x05\x05\0\x02\xc5\x01\x12\x04\xcb\x01\x02\x1e\n\
    \x0e\n\x06\x05\0\x02\xc5\x01\x01\x12\x04\xcb\x01\x02\x17\n\x0e\n\x06\x05\
    \0\x02\xc5\x01\x02\x12\x04\xcb\x01\x1a\x1d\n\r\n\x05\x05\0\x02\xc6\x01\
    \x12\x04\xcc\x01\x02\x1d\n\x0e\n\x06\x05\0\x02\xc6\x01\x01\x12\x04\xcc\
    \x01\x02\x16\n\x0e\n\x06\x05\0\x02\xc6\x01\x02\x12\x04\xcc\x01\x19\x1c\n\
    \r\n\x05\x05\0\x02\xc7\x01\x12\x04\xcd\x01\x02%\n\x0e\n\x06\x05\0\x02\
    \xc7\x01\x01\x12\x04\xcd\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\xc7\x01\x02\
    \x12\x04\xcd\x01!$\n\r\n\x05\x05\0\x02\xc8\x01\x12\x04\xce\x01\x02\x17\n\
    \x0e\n\x06\x05\0\x02\xc8\x01\x01\x12\x04\xce\x01\x02\x10\n\x0e\n\x06\x05\
    \0\x02\xc8\x01\x02\x12\x04\xce\x01\x13\x16\n\r\n\x05\x05\0\x02\xc9\x01\
    \x12\x04\xcf\x01\x02\x19\n\x0e\n\x06\x05\0\x02\xc9\x01\x01\x12\x04\xcf\
    \x01\x02\x12\n\x0e\n\x06\x05\0\x02\xc9\x01\x02\x12\x04\xcf\x01\x15\x18\n\
    \r\n\x05\x05\0\x02\xca\x01\x12\x04\xd0\x01\x02\x19\n\x0e\n\x06\x05\0\x02\
    \xca\x01\x01\x12\x04\xd0\x01\x02\x12\n\x0e\n\x06\x05\0\x02\xca\x01\x02\
    \x12\x04\xd0\x01\x15\x18\n\r\n\x05\x05\0\x02\xcb\x01\x12\x04\xd1\x01\x02\
    \x20\n\x0e\n\x06\x05\0\x02\xcb\x01\x01\x12\x04\xd1\x01\x02\x19\n\x0e\n\
    \x06\x05\0\x02\xcb\x01\x02\x12\x04\xd1\x01\x1c\x1f\n\r\n\x05\x05\0\x02\
    \xcc\x01\x12\x04\xd2\x01\x02\x1e\n\x0e\n\x06\x05\0\x02\xcc\x01\x01\x12\
    \x04\xd2\x01\x02\x17\n\x0e\n\x06\x05\0\x02\xcc\x01\x02\x12\x04\xd2\x01\
    \x1a\x1d\n\r\n\x05\x05\0\x02\xcd\x01\x12\x04\xd3\x01\x02\x18\n\x0e\n\x06\
    \x05\0\x02\xcd\x01\x01\x12\x04\xd3\x01\x02\x11\n\x0e\n\x06\x05\0\x02\xcd\
    \x01\x02\x12\x04\xd3\x01\x14\x17\n\r\n\x05\x05\0\x02\xce\x01\x12\x04\xd4\
    \x01\x02\"\n\x0e\n\x06\x05\0\x02\xce\x01\x01\x12\x04\xd4\x01\x02\x1b\n\
    \x0e\n\x06\x05\0\x02\xce\x01\x02\x12\x04\xd4\x01\x1e!\n\r\n\x05\x05\0\
    \x02\xcf\x01\x12\x04\xd5\x01\x02\x1b\n\x0e\n\x06\x05\0\x02\xcf\x01\x01\
    \x12\x04\xd5\x01\x02\x14\n\x0e\n\x06\x05\0\x02\xcf\x01\x02\x12\x04\xd5\
    \x01\x17\x1a\n\r\n\x05\x05\0\x02\xd0\x01\x12\x04\xd6\x01\x02\x1b\n\x0e\n\
    \x06\x05\0\x02\xd0\x01\x01\x12\x04\xd6\x01\x02\x14\n\x0e\n\x06\x05\0\x02\
    \xd0\x01\x02\x12\x04\xd6\x01\x17\x1a\n\r\n\x05\x05\0\x02\xd1\x01\x12\x04\
    \xd7\x01\x02\x19\n\x0e\n\x06\x05\0\x02\xd1\x01\x01\x12\x04\xd7\x01\x02\
    \x12\n\x0e\n\x06\x05\0\x02\xd1\x01\x02\x12\x04\xd7\x01\x15\x18\n\r\n\x05\
    \x05\0\x02\xd2\x01\x12\x04\xd8\x01\x02\x19\n\x0e\n\x06\x05\0\x02\xd2\x01\
    \x01\x12\x04\xd8\x01\x02\x12\n\x0e\n\x06\x05\0\x02\xd2\x01\x02\x12\x04\
    \xd8\x01\x15\x18\n\r\n\x05\x05\0\x02\xd3\x01\x12\x04\xd9\x01\x02!\n\x0e\
    \n\x06\x05\0\x02\xd3\x01\x01\x12\x04\xd9\x01\x02\x1a\n\x0e\n\x06\x05\0\
    \x02\xd3\x01\x02\x12\x04\xd9\x01\x1d\x20\n\r\n\x05\x05\0\x02\xd4\x01\x12\
    \x04\xda\x01\x02\x19\n\x0e\n\x06\x05\0\x02\xd4\x01\x01\x12\x04\xda\x01\
    \x02\x12\n\x0e\n\x06\x05\0\x02\xd4\x01\x02\x12\x04\xda\x01\x15\x18\n\r\n\
    \x05\x05\0\x02\xd5\x01\x12\x04\xdb\x01\x02\x1c\n\x0e\n\x06\x05\0\x02\xd5\
    \x01\x01\x12\x04\xdb\x01\x02\x15\n\x0e\n\x06\x05\0\x02\xd5\x01\x02\x12\
    \x04\xdb\x01\x18\x1b\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
