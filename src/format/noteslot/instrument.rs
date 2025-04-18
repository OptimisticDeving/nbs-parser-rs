#[derive(Debug, Clone, Copy)]
pub enum InstrumentBlock {
    Air,
    Wood,
    Stone,
    Sand,
    Glass,
    Wool,
    Clay,
    GoldBlock,
    PackedIce,
    BoneBlock,
    IronBlock,
    SoulSand,
    Pumpkin,
    EmeraldBlock,
    HayBale,
    Glowstone,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum Instrument {
    Piano,
    DoubleBass,
    BassDrum,
    SnareDrum,
    Click,
    Guitar,
    Flute,
    Bell,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
    Unrecognized(u8),
}

impl From<Instrument> for InstrumentBlock {
    fn from(value: Instrument) -> Self {
        match value {
            Instrument::Piano => Self::Air,
            Instrument::DoubleBass => Self::Wood,
            Instrument::BassDrum => Self::Stone,
            Instrument::SnareDrum => Self::Sand,
            Instrument::Click => Self::Glass,
            Instrument::Guitar => Self::Wool,
            Instrument::Flute => Self::Clay,
            Instrument::Bell => Self::GoldBlock,
            Instrument::Chime => Self::PackedIce,
            Instrument::Xylophone => Self::BoneBlock,
            Instrument::IronXylophone => Self::IronBlock,
            Instrument::CowBell => Self::SoulSand,
            Instrument::Didgeridoo => Self::Pumpkin,
            Instrument::Bit => Self::EmeraldBlock,
            Instrument::Banjo => Self::HayBale,
            Instrument::Pling => Self::Glowstone,
            Instrument::Unrecognized(_) => Self::Unknown,
        }
    }
}

impl From<InstrumentBlock> for Instrument {
    fn from(value: InstrumentBlock) -> Self {
        match value {
            InstrumentBlock::Air => Self::Piano,
            InstrumentBlock::Wood => Self::DoubleBass,
            InstrumentBlock::Stone => Self::BassDrum,
            InstrumentBlock::Sand => Self::SnareDrum,
            InstrumentBlock::Glass => Self::Click,
            InstrumentBlock::Wool => Self::Guitar,
            InstrumentBlock::Clay => Self::Flute,
            InstrumentBlock::GoldBlock => Self::Bell,
            InstrumentBlock::PackedIce => Self::Chime,
            InstrumentBlock::BoneBlock => Self::Xylophone,
            InstrumentBlock::IronBlock => Self::IronXylophone,
            InstrumentBlock::SoulSand => Self::CowBell,
            InstrumentBlock::Pumpkin => Self::Didgeridoo,
            InstrumentBlock::EmeraldBlock => Self::Bit,
            InstrumentBlock::HayBale => Self::Banjo,
            InstrumentBlock::Glowstone => Self::Pling,
            InstrumentBlock::Unknown => Self::Unrecognized(u8::MAX),
        }
    }
}

impl Instrument {
    #[inline]
    pub const fn from_byte(n: u8) -> Self {
        match n {
            0 => Self::Piano,
            1 => Self::DoubleBass,
            2 => Self::BassDrum,
            3 => Self::SnareDrum,
            4 => Self::Click,
            5 => Self::Guitar,
            6 => Self::Flute,
            7 => Self::Bell,
            8 => Self::Chime,
            9 => Self::Xylophone,
            10 => Self::IronXylophone,
            11 => Self::CowBell,
            12 => Self::Didgeridoo,
            13 => Self::Bit,
            14 => Self::Banjo,
            15 => Self::Pling,
            _ => Self::Unrecognized(n),
        }
    }
}
