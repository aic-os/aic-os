use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType { Evm, Utxo, Cosmos, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chain {
    // EVM (65)
    Ethereum, Bsc, Polygon, Arbitrum, Optimism, Base, Avalanche,
    Linea, Scroll, ZkSync, Mantle, Blast, Gnosis, Celo, Fantom,
    Cronos, Moonbeam, Aurora, Filecoin, PolygonZkEvm,
    Harmony, Metis, Klaytn, Astar, Moonriver, Boba, Conflux,
    Evmos, Canto, Okc, Fuse, Telos, Dogechain, Palm, Dfk,
    Songbird, Gochain, Callisto, Ubic, Xinfin, Wanchain,
    ThunderCore, Hoo, Kcc, Tombchain, Bitgert, ArbitrumNova,
    Opbnb, Zora, Mode, Mint, MantaPacific, Kroma, Pgn,
    Redstone, Ancient8, Cyber, Lisk, Rsk, Bitlayer,
    Zetachain, Dexalot, ApeChain, Curtis, Frame, Rari,
    // UTXO (10)
    Bitcoin, Litecoin, Dogecoin, BitcoinCash, Dash, Zcash,
    BitcoinSv, DigiByte, Ravencoin, Groestlcoin,
    // Cosmos (25)
    CosmosHub, Osmosis, Celestia, Injective, Kava, Sei, Akash, Juno,
    Stargaze, Kujira, Stride, Neutron, Archway, Persistence,
    Umee, Comdex, Crescent, Regen, Sentinel, Sifchain,
    Chihuahua, Bitsong, Desmos, Emoney, Medibloc,
    // Other (6)
    Solana, Tron, Ton, Sui, Aptos, Ripple,
}

impl Chain {
    pub const ALL: &'static [Chain] = &[
        // EVM
        Chain::Ethereum, Chain::Bsc, Chain::Polygon, Chain::Arbitrum,
        Chain::Optimism, Chain::Base, Chain::Avalanche, Chain::Linea,
        Chain::Scroll, Chain::ZkSync, Chain::Mantle, Chain::Blast,
        Chain::Gnosis, Chain::Celo, Chain::Fantom, Chain::Cronos,
        Chain::Moonbeam, Chain::Aurora, Chain::Filecoin, Chain::PolygonZkEvm,
        Chain::Harmony, Chain::Metis, Chain::Klaytn, Chain::Astar,
        Chain::Moonriver, Chain::Boba, Chain::Conflux, Chain::Evmos,
        Chain::Canto, Chain::Okc, Chain::Fuse, Chain::Telos,
        Chain::Dogechain, Chain::Palm, Chain::Dfk, Chain::Songbird,
        Chain::Gochain, Chain::Callisto, Chain::Ubic, Chain::Xinfin,
        Chain::Wanchain, Chain::ThunderCore, Chain::Hoo, Chain::Kcc,
        Chain::Tombchain, Chain::Bitgert, Chain::ArbitrumNova, Chain::Opbnb,
        Chain::Zora, Chain::Mode, Chain::Mint, Chain::MantaPacific,
        Chain::Kroma, Chain::Pgn, Chain::Redstone, Chain::Ancient8,
        Chain::Cyber, Chain::Lisk, Chain::Rsk, Chain::Bitlayer,
        Chain::Zetachain, Chain::Dexalot, Chain::ApeChain, Chain::Curtis,
        Chain::Frame, Chain::Rari,
        // UTXO
        Chain::Bitcoin, Chain::Litecoin, Chain::Dogecoin, Chain::BitcoinCash,
        Chain::Dash, Chain::Zcash, Chain::BitcoinSv, Chain::DigiByte,
        Chain::Ravencoin, Chain::Groestlcoin,
        // Cosmos
        Chain::CosmosHub, Chain::Osmosis, Chain::Celestia, Chain::Injective,
        Chain::Kava, Chain::Sei, Chain::Akash, Chain::Juno,
        Chain::Stargaze, Chain::Kujira, Chain::Stride, Chain::Neutron,
        Chain::Archway, Chain::Persistence, Chain::Umee, Chain::Comdex,
        Chain::Crescent, Chain::Regen, Chain::Sentinel, Chain::Sifchain,
        Chain::Chihuahua, Chain::Bitsong, Chain::Desmos, Chain::Emoney,
        Chain::Medibloc,
        // Other
        Chain::Solana, Chain::Tron, Chain::Ton, Chain::Sui,
        Chain::Aptos, Chain::Ripple,
    ];

    pub fn count() -> usize { Self::ALL.len() }

    pub fn chain_type(self) -> ChainType {
        use Chain::*;
        match self {
            Ethereum | Bsc | Polygon | Arbitrum | Optimism | Base | Avalanche
            | Linea | Scroll | ZkSync | Mantle | Blast | Gnosis | Celo | Fantom
            | Cronos | Moonbeam | Aurora | Filecoin | PolygonZkEvm
            | Harmony | Metis | Klaytn | Astar | Moonriver | Boba | Conflux
            | Evmos | Canto | Okc | Fuse | Telos | Dogechain | Palm | Dfk
            | Songbird | Gochain | Callisto | Ubic | Xinfin | Wanchain
            | ThunderCore | Hoo | Kcc | Tombchain | Bitgert | ArbitrumNova
            | Opbnb | Zora | Mode | Mint | MantaPacific | Kroma | Pgn
            | Redstone | Ancient8 | Cyber | Lisk | Rsk | Bitlayer
            | Zetachain | Dexalot | ApeChain | Curtis | Frame | Rari => ChainType::Evm,
            Bitcoin | Litecoin | Dogecoin | BitcoinCash | Dash | Zcash
            | BitcoinSv | DigiByte | Ravencoin | Groestlcoin => ChainType::Utxo,
            CosmosHub | Osmosis | Celestia | Injective | Kava | Sei | Akash | Juno
            | Stargaze | Kujira | Stride | Neutron | Archway | Persistence
            | Umee | Comdex | Crescent | Regen | Sentinel | Sifchain
            | Chihuahua | Bitsong | Desmos | Emoney | Medibloc => ChainType::Cosmos,
            Solana | Tron | Ton | Sui | Aptos | Ripple => ChainType::Other,
        }
    }

    pub fn as_str(self) -> &'static str {
        use Chain::*;
        match self {
            Ethereum => "ethereum", Bsc => "bsc", Polygon => "polygon",
            Arbitrum => "arbitrum", Optimism => "optimism", Base => "base",
            Avalanche => "avalanche", Linea => "linea", Scroll => "scroll",
            ZkSync => "zksync", Mantle => "mantle", Blast => "blast",
            Gnosis => "gnosis", Celo => "celo", Fantom => "fantom",
            Cronos => "cronos", Moonbeam => "moonbeam", Aurora => "aurora",
            Filecoin => "filecoin", PolygonZkEvm => "polygon-zkevm",
            Harmony => "harmony", Metis => "metis", Klaytn => "klaytn",
            Astar => "astar", Moonriver => "moonriver", Boba => "boba",
            Conflux => "conflux", Evmos => "evmos", Canto => "canto",
            Okc => "okc", Fuse => "fuse", Telos => "telos",
            Dogechain => "dogechain", Palm => "palm", Dfk => "dfk",
            Songbird => "songbird", Gochain => "gochain", Callisto => "callisto",
            Ubic => "ubiq", Xinfin => "xinfin", Wanchain => "wanchain",
            ThunderCore => "thundercore", Hoo => "hoo", Kcc => "kcc",
            Tombchain => "tombchain", Bitgert => "bitgert", ArbitrumNova => "arbitrum-nova",
            Opbnb => "opbnb", Zora => "zora", Mode => "mode",
            Mint => "mint", MantaPacific => "manta-pacific", Kroma => "kroma",
            Pgn => "pgn", Redstone => "redstone", Ancient8 => "ancient8",
            Cyber => "cyber", Lisk => "lisk", Rsk => "rsk",
            Bitlayer => "bitlayer", Zetachain => "zetachain", Dexalot => "dexalot",
            ApeChain => "apechain", Curtis => "curtis", Frame => "frame",
            Rari => "rari",
            Bitcoin => "bitcoin", Litecoin => "litecoin", Dogecoin => "dogecoin",
            BitcoinCash => "bitcoin-cash", Dash => "dash", Zcash => "zcash",
            BitcoinSv => "bitcoin-sv", DigiByte => "digibyte", Ravencoin => "ravencoin",
            Groestlcoin => "groestlcoin",
            CosmosHub => "cosmos", Osmosis => "osmosis", Celestia => "celestia",
            Injective => "injective", Kava => "kava", Sei => "sei",
            Akash => "akash", Juno => "juno", Stargaze => "stargaze",
            Kujira => "kujira", Stride => "stride", Neutron => "neutron",
            Archway => "archway", Persistence => "persistence", Umee => "umee",
            Comdex => "comdex", Crescent => "crescent", Regen => "regen",
            Sentinel => "sentinel", Sifchain => "sifchain", Chihuahua => "chihuahua",
            Bitsong => "bitsong", Desmos => "desmos", Emoney => "emoney",
            Medibloc => "medibloc",
            Solana => "solana", Tron => "tron", Ton => "ton",
            Sui => "sui", Aptos => "aptos", Ripple => "ripple",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        let s = s.trim().to_lowercase();
        for &c in Self::ALL {
            if c.as_str() == s { return Some(c); }
        }
        use Chain::*;
        Some(match s.as_str() {
            "eth" => Ethereum, "bnb" | "binance" => Bsc, "matic" => Polygon,
            "arb" => Arbitrum, "op" => Optimism, "avax" => Avalanche,
            "zkevm" => PolygonZkEvm, "btc" => Bitcoin, "ltc" => Litecoin,
            "doge" => Dogecoin, "bch" => BitcoinCash, "bsv" => BitcoinSv,
            "atom" => CosmosHub, "osmo" => Osmosis, "tia" => Celestia,
            "inj" => Injective, "sol" => Solana, "trx" => Tron,
            "xrp" => Ripple, "apt" => Aptos, "ftm" => Fantom,
            "glmr" => Moonbeam, "movr" => Moonriver, "cro" => Cronos,
            "fil" => Filecoin, "xdai" => Gnosis,
            _ => return None,
        })
    }
}
