// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use node_runtime::constants::currency::*;
use node_runtime::Block;
use node_runtime::{
    wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, BridgeConfig,
    ContractsConfig, CouncilConfig, CreditConfig, DeeperNodeConfig, DemocracyConfig,
    ElectionsConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig, SessionKeys,
    SocietyConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
};
use pallet_credit::{CreditData, CreditLevel, CreditSetting};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill, Percent,
};

pub use node_primitives::{AccountId, Balance, BlockNumber, Signature};
pub use node_runtime::GenesisConfig;
use serde_json as json;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// get root key for deeper testnet
pub fn testnet_root_key() -> AccountId {
    hex![
        // 5CHu6tEdZWEnGHa928e9CfsXnL5otzRg4xGwqCscXDrcH38t
        "0a100b6bf4e332cac53b98af0003bbbf6984d2171bbe30a05a97bb28f5212119"
    ]
    .into()
}

/// return other authority keys as default validators
pub fn other_authority_keys() -> Vec<(
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
)> {
    vec![
        (
            // 5CwMNoeXEktdpJFDNiPi29odWr8KANBWodzkBuE56DGa5ksq
            hex!["26a0928a4a88db828747ac4d503a902f279052aa6d48f1541bad709bbad1d750"].into(),
            // 5FyXGesEKhP7qKgx8GQs61hWF8HvrDCbCBTaN298SX3QTDhq
            hex!["acfd11cf17c7253febc403cf4c27d1ad673011f18c5aae8846eed067ae81d342"].into(),
            // 5EHzqtDmbUDZvxgGWjKGYz5kvmv1McBsBfZ3T2ZBL763yhj4
            hex!["629bd6b5e0bee300e455d2d5a367afca580cfbb7cab9856486c4fcc32ef9e825"]
                .unchecked_into(),
            // 5FyXGesEKhP7qKgx8GQs61hWF8HvrDCbCBTaN298SX3QTDhq
            hex!["acfd11cf17c7253febc403cf4c27d1ad673011f18c5aae8846eed067ae81d342"]
                .unchecked_into(),
            // 5FyXGesEKhP7qKgx8GQs61hWF8HvrDCbCBTaN298SX3QTDhq
            hex!["acfd11cf17c7253febc403cf4c27d1ad673011f18c5aae8846eed067ae81d342"]
                .unchecked_into(),
            // 5FyXGesEKhP7qKgx8GQs61hWF8HvrDCbCBTaN298SX3QTDhq
            hex!["acfd11cf17c7253febc403cf4c27d1ad673011f18c5aae8846eed067ae81d342"]
                .unchecked_into(),
        ),
        (
            // 5GQrjS6o6xG1LZxdc3SfVhoyCCCqBFL434seLiJLsJg92SyB
            hex!["c04fb7faed38acbb55f02afe12f624fc77a1b30e02ca8a6a08dde940baa9a82f"].into(),
            // 5GQpi5PnxBEBTzPwt8x4bYks1uD4Hy5A8ZxmmLihMiN3nqAA
            hex!["c048e845940a64de14307e316e987e95f4a199faf8ceb8d4e5a76f5f98f59c16"].into(),
            // 5DxYdPQuxWpjNWwPbzUE1QgkXJQR8NGjhQn7UuvD1Vaz4veX
            hex!["53c5ed4aec243acac4a02866f891f32653bc2ed54063eb5d9962ebdaa2dcdcbe"]
                .unchecked_into(),
            // 5GQpi5PnxBEBTzPwt8x4bYks1uD4Hy5A8ZxmmLihMiN3nqAA
            hex!["c048e845940a64de14307e316e987e95f4a199faf8ceb8d4e5a76f5f98f59c16"]
                .unchecked_into(),
            // 5GQpi5PnxBEBTzPwt8x4bYks1uD4Hy5A8ZxmmLihMiN3nqAA
            hex!["c048e845940a64de14307e316e987e95f4a199faf8ceb8d4e5a76f5f98f59c16"]
                .unchecked_into(),
            // 5GQpi5PnxBEBTzPwt8x4bYks1uD4Hy5A8ZxmmLihMiN3nqAA
            hex!["c048e845940a64de14307e316e987e95f4a199faf8ceb8d4e5a76f5f98f59c16"]
                .unchecked_into(),
        ),
        (
            // 5HNiABAGEcQtvtdkqrALzeieczDMAKjB4nEBqV7UcRsAEJxe
            hex!["eae899c4aac8bd52b2d206d244f26b6d39a7701939cbd33b2eafd11ca9050b0e"].into(),
            // 5Cd5bhgiBVAWxZsGiLUfC213A5cybeGgGShovY6ktKp5mosf
            hex!["18b10afafa9c3a3ac5ab3c886f68d7c13ac500fe009e9c35c9c2cc0188ad112f"].into(),
            // 5GFViwQFPAJqSw47jA9GmShvpG57kFEUku9GPJhR6EPNe6Ac
            hex!["b92bc9fcc24867030bb544e432e3a190a7516bde6008bcf3eeae6ec0c191fb8c"]
                .unchecked_into(),
            // 5Cd5bhgiBVAWxZsGiLUfC213A5cybeGgGShovY6ktKp5mosf
            hex!["18b10afafa9c3a3ac5ab3c886f68d7c13ac500fe009e9c35c9c2cc0188ad112f"]
                .unchecked_into(),
            // 5Cd5bhgiBVAWxZsGiLUfC213A5cybeGgGShovY6ktKp5mosf
            hex!["18b10afafa9c3a3ac5ab3c886f68d7c13ac500fe009e9c35c9c2cc0188ad112f"]
                .unchecked_into(),
            // 5Cd5bhgiBVAWxZsGiLUfC213A5cybeGgGShovY6ktKp5mosf
            hex!["18b10afafa9c3a3ac5ab3c886f68d7c13ac500fe009e9c35c9c2cc0188ad112f"]
                .unchecked_into(),
        ),
    ]
}

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
    // stash, controller, session-key
    // generated with secret:
    // for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
    // and
    // for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            // 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
            hex!["9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"].into(),
            // 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
            hex!["781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"].into(),
            // 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
            hex!["9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
        ),
        (
            // 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
            hex!["68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"].into(),
            // 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
            hex!["c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"].into(),
            // 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
            hex!["7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
        ),
        (
            // 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
            hex!["547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"].into(),
            // 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
            hex!["9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"].into(),
            // 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
            hex!["5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
        ),
        (
            // 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
            hex!["f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"].into(),
            // 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
            hex!["66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"].into(),
            // 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
            hex!["3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
        ),
    ];

    // generated with secret: subkey inspect "$secret"/fir
    let root_key: AccountId = hex![
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
    ]
    .into();

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

    testnet_genesis(initial_authorities, root_key, Some(endowed_accounts), false)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Staging Testnet",
        "staging_testnet",
        ChainType::Live,
        staging_testnet_config_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        None,
        None,
        Default::default(),
    )
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    enable_println: bool,
) -> GenesisConfig {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });
    initial_authorities.iter().for_each(|x| {
        if !endowed_accounts.contains(&x.0) {
            endowed_accounts.push(x.0.clone())
        }
    });

    let num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 10_000_000 * DPR;
    const STASH: Balance = ENDOWMENT / 1000;

    let bridge_validators: Vec<AccountId> = vec![
        hex!("32b6e2fd3d19d875fc5a23a2bbc449b9b2dad1aa5f11aec6fe5ea9f5ba08f70e").into(),
        // 5DDCabfWypaJwMdXeKCxHmBtxWwob3RSYZeP9pMZa6V3bKEL
        hex!("9c164987ba60615be6074837036983ab96559cb4a3d6ada17ed0e092f044a521").into(),
        // 5FbMwvsF5serYgaQkcJ9itgiUX4RxftCF6reptrLym6YgERX
        hex!("5e414ecf3c9d3fba082d1b440b24abb7539ef64e9473bed53a754f686f06e52f").into(),
        // 5ECHkxssXVeENxozUbe4p64sZq6ktzFnv37BCbsAoS8AMxU3
    ];

    let mut new_endowed_accounts = endowed_accounts.clone();
    new_endowed_accounts
        .push(hex!("32b6e2fd3d19d875fc5a23a2bbc449b9b2dad1aa5f11aec6fe5ea9f5ba08f70e").into());
    new_endowed_accounts
        .push(hex!("9c164987ba60615be6074837036983ab96559cb4a3d6ada17ed0e092f044a521").into());
    new_endowed_accounts
        .push(hex!("5e414ecf3c9d3fba082d1b440b24abb7539ef64e9473bed53a754f686f06e52f").into());

    GenesisConfig {
        frame_system: Some(SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: new_endowed_accounts
                .iter()
                .cloned()
                .map(|x| (x, ENDOWMENT))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            era_validator_reward: 57534 * DPR, // about 21 million DPR per year
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println, // this should only be enabled on development chains
                ..Default::default()
            },
        }),
        pallet_sudo: Some(SudoConfig { key: root_key }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_membership_Instance1: Some(Default::default()),
        pallet_treasury: Some(Default::default()),
        pallet_society: Some(SocietyConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            pot: 0,
            max_members: 999,
        }),
        pallet_vesting: Some(Default::default()),
        pallet_deeper_node: Some(DeeperNodeConfig { tmp: 0 }),
        pallet_eth_sub_bridge: Some(BridgeConfig {
            validator_accounts: bridge_validators,
            validators_count: 3u32,
            current_limits: vec![
                20000 * DPR,     // max single tx limit
                200000000 * DPR, // max daily tx limit
                60000 * DPR,     // max daily tx per address limit
                400000000 * DPR, // max pending tx limit
                100 * DPR,       // min tx limit
            ],
        }),
        pallet_credit: Some(CreditConfig {
            credit_settings: vec![
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Zero,
                    staking_balance: 0,
                    base_apy: Percent::from_percent(0),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 0,
                    reward_per_referee: 0,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::One,
                    staking_balance: 20_000 * DPR,
                    base_apy: Percent::from_percent(39),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 1,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Two,
                    staking_balance: 46_800 * DPR,
                    base_apy: Percent::from_percent(47),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 2,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Three,
                    staking_balance: 76_800 * DPR,
                    base_apy: Percent::from_percent(53),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 3,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Four,
                    staking_balance: 138_000 * DPR,
                    base_apy: Percent::from_percent(59),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 7,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Five,
                    staking_balance: 218_000 * DPR,
                    base_apy: Percent::from_percent(66),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 12,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Six,
                    staking_balance: 288_000 * DPR,
                    base_apy: Percent::from_percent(74),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 18,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Seven,
                    staking_balance: 368_000 * DPR,
                    base_apy: Percent::from_percent(82),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 25,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 0,
                    credit_level: CreditLevel::Eight,
                    staking_balance: 468_000 * DPR,
                    base_apy: Percent::from_percent(90),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 34,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Zero,
                    staking_balance: 0,
                    base_apy: Percent::from_percent(0),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 0,
                    reward_per_referee: 0,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::One,
                    staking_balance: 20_000 * DPR,
                    base_apy: Percent::from_percent(39),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 1,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Two,
                    staking_balance: 46_800 * DPR,
                    base_apy: Percent::from_percent(44),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 2,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Three,
                    staking_balance: 76_800 * DPR,
                    base_apy: Percent::from_percent(50),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 3,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Four,
                    staking_balance: 138_000 * DPR,
                    base_apy: Percent::from_percent(56),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 7,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Five,
                    staking_balance: 218_000 * DPR,
                    base_apy: Percent::from_percent(62),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 12,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Six,
                    staking_balance: 288_000 * DPR,
                    base_apy: Percent::from_percent(69),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 18,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Seven,
                    staking_balance: 368_000 * DPR,
                    base_apy: Percent::from_percent(75),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 25,
                    reward_per_referee: 0 * DPR,
                },
                CreditSetting {
                    campaign_id: 1,
                    credit_level: CreditLevel::Eight,
                    staking_balance: 468_000 * DPR,
                    base_apy: Percent::from_percent(80),
                    bonus_apy: Percent::from_percent(0),
                    max_rank_with_bonus: 0u32,
                    tax_rate: Percent::from_percent(0),
                    max_referees_with_rewards: 34,
                    reward_per_referee: 0 * DPR,
                },
            ],
            user_credit_data: vec![
                (
                    hex!("c4044e2d452a1db12a1e3b2e4999847ac9a6ad707a21ca596855f51970b4842e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 1u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0eb30736cbffd2087ff991c198b057c64817868cc8545da1a4b3b7e1d39a0950")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 2u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("48919fb8df33c28f29ba330a75d6c8c5578fcc620fae13540ee71b5dd7ccbb15")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 3u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("68c9d52297abc6804b356063aac74c0f415a3a0c80dc27ac09d4234d60114426")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 4u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b2bc15349b219841dcba5c74be186990ec1bb26f524b2c2f1e84c854b94a7b09")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 5u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c87f53ca2c553af3081d7587bca12ec4c672aecc210161e2eed6eb12e53a4e12")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 6u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("46559a65b548e6d19a72d926936582f07d75ba9cd6d748ac1a49d3ee449f0235")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 7u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3625199a8d4b32709e515293e64ff2401638f8e7ce26d30a875ae16dee565754")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 8u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9ab992f6f7928bf43a2ec0e20e008858dce1c47e392fa933ac0d71181e94a144")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 9u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5e2f103b9a0ea88d74b37a19974ea3ea0314c0ba68f8eb05f3ff019604d40b40")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 10u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("848e550b91454abbb4b449f888d5a708f4cf419df3bcab72b229c3f540597677")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 11u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("322707b4f1e445db63a83c87f4f625c641adaa45a1d68e9bc0d5158cd852e66e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 12u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("44c2828afa8f6a3fee1a3660feda5087b0ecc65f9d47528bb9151c0e911d1041")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 13u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("14c46e9212d34f08f388b83011f94bdd39eda81aef4cff0f1e36e06f0360e40f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 14u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("681f3616b4635679845c2a67a1e697a7445c1b2b7a7d6b9025ce0ca9eac7e021")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 15u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("02c77d04079b5d4ac21c793095e0ca0eff572c1fc148c266c10a745bcaf8632d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 16u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d0f63e77a6b1ab6e98b11b8bcb6d625229a4d9a1abe51b9cf23e7f08e9485731")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 17u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ab214be550084b3b84e3d7721a0ad49be6631c1683b34221d11d9cdc00e7207")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 18u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a759bccd293a22f16a87824e3a7043f830518ef25b50ddbcd5f4e88e7292672")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 19u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("383febae6d445cd9ad57dccf12aa07ba612cf91d9cf1bbb9961a2c9f6e670626")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 20u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8817494cc836486cbd16b24a542f6faf23b84f552991cb15bac0fad709712002")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 21u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7206c5c7c5a841867c57aad945c4d26cbef3abb085aa7198cee6b19495c9bc33")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 22u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4c15a985c8c57bd1c7af7585e5fcdf21abc7524a4252ce3169dc2dc6727272b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 23u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d0f32b5373edc3c9fafb6828f2e8de3a20c409f3041e2b1f693af81b80acca1c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 24u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3cbbb43521e95b6939af85fdb808cc954c0ed58a33ab93bc4d336371944e9540")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 25u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ad6b7adbd6ed69db7eb03588784011de064ca00b90a21ffb7d237075385e675")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 26u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("64fe27ab968e7597c1061b0d7566732cf11000830554b3a4770c6ce14c4f570c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 27u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c8c762600b29535a13c974e4e4c6370f45cd43bf281d52845ed49ea97eb0bf5d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 28u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a0f10e31d73fa25485d9a74f6138ca7f58f49b2b9f2b818eb1728b06daf6cd50")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 29u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80037395918d66f2663c5e2f062811b69486a65a44480f1cb50209af0c9e6c40")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 30u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ccf5430bc244eec9c57cfb3b905df897e0d36342e3e45ae8663902e1fb595605")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 31u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8eb1393a8bddc58ac8bdecaa71dc4772dcb710a329291bbf24434d68785a6023")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 32u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9a303c70fddd7eb206a80a0a52d57f9e0ec2cf94af9798cb3b503d18db4ed13b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 33u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e23796443d254e915aeebac83e9778d669fac6aec5018b0f1c318fbbeef07f03")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 34u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f0ce391540d60a6c65083c4a8ec3810eaad59b2a42d8f92b943dffeb489f1d03")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 35u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("28ad3a2d1550f64bd26710fc6892274cc58852dd202a45702ffd7e4b514d7c39")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 36u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b8ce50ea939bf14e254bb9895709a58e2d4b9d90024b339f978d65102f54cf53")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 37u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7e107e22a6bb9fdc33381b1e164cf90ea519025a87181a46440201eff8a87579")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 38u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("06c124e808ed37b9d9ad05df0dd122a0d49af984b480f6b3b2ee94e8966e5f5e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 39u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b6d34c218adba0f8a888f64e7ae59adb49d1873ab41e3c97ddd99e531c9b484e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 40u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0e0db47d00dda594589370b579fdf000686b0b5892f9faf3700d327455697e5c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 41u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7ed7487b5453c6d8845a3455b2cb95cf9cd94520f91be5ae6efc01d12bbab27b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 42u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8a6561f8fc5169c7a583a0a7f816cbc85c3787eafb44593dcd64c26f996b0b1c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 43u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d428b84d97c3aab2a5cf13660e6270043ab872e23d34493f04380167becc4478")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 44u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("90c03fe64e7a39af2b44374e6ed30587b363c40330723acf706b7b9d04d9d820")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 45u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c810944a666d422e6b4d600cdb4e71f41fcf5eaedd93db9463173afcb7159d27")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 46u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("924391088ec232968e70f4dab313524433cafc615e834e62c06fdcb6ecba931f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 47u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4050d109263fb22d4191bd959e8e9cb62f79739321c8d17a62f75aeaf4d284f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 48u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6a9dd87e54988a92f6fb3d741f6e0942c83623e114c39b6a9c171a2229e5a904")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 49u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f23c785ef7a1852d1443c3668ea00a651c6f42190ca7706a3c4853591046dc59")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 50u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aed5b1d41a0095a7980e438f40c88cf08574ac2ed230ba72abeddb68c513d11b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 51u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6e92a683ba23b41837b47986138963a7fd05dd46bd838edf8dbac24719307d61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 52u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("dc5beac91119837fd1f88068526b1571b9f35a6f16e816159a891be708dc5b4d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 53u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de237e2ac1aeeb5aa396a8a9ada0a010b1c465debbbab05e6ccbf4dc88bbce7c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 54u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3070195950bf59afb81734b72560d9330f44bb5216c6f1080273f59e08e82a48")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 55u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2861b31d840069d67c031652ccf4a76e05d12d62706ecc67d968a5b482c11548")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 56u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4ec4d9f4c56b3e61802d91370589d7ae16b1c63e0dea0d98ceeee57ac3e5032")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 57u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7cc51153938e76a10cc1bc4a0e517d47e9134b83da4010b46be64f4e27a59971")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 58u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("62d44701063e3e6df0f0970c5bc88736d9a97b56be8a5c1d5a11454d3899a260")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 59u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cea22a66324ed2085faa5a440d3fe4dac0ccf9d2c91641ec5f63f4904bdde262")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 60u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2cd4eb08603f3dfcd37addfbeb1158ad6be4fddac407ed4c5dd3273926217d4a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 61u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a099f434edf3382a1349b054fd964f09ecce359d93abb068b8fa2fd7b3b33b0a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 62u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec2ce280d1f2c7b1f73260447bc9091bf8cb972c8ad913a920387b1a22d1a905")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 63u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5aacd2a551a9c7d5e3ffe49b21a14ddb2600f35c1ea55c136501f06005eb7725")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 64u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("962c584f479e98d97e234bd8096b829581854008b0d24a213b68eea7c9243c05")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 65u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ea3f9ed1e5aeca8e2e7ce9c70e39161c47749115a651476e1c5f271e43eae09")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 66u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fc718bae1aae7fb1513d4f665596736f2532afa6cac865f89ecdb76fe3bc657a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 67u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e6887086a07b4b1053d954e0098f84cb53bdae53ad71e2da9fd70e9b11329c2d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 68u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e28cc348d859a50a40421a2557459679e539097450d2df559e9dedb1fd67d772")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 69u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5e60a40b1ffb0dc121fb892a241d7790948690191b1e4bd721180ca7032aad2b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 70u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("989c3a04d550b331bf180ff6ceee2b96cfc915769339e77ea9a9e26d3639963a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 71u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b8c5502cdb92a793f388d31d85e9f45c7b1f0f829dae0879e2503c6abe715c62")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 72u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a097a0915cff84ac42851f33070dea0cfb5ea4d484494ce5c16e1256fd8abd48")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 73u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9e8278fe15cc0da6d0bf9aee274a738d2585ee5d4d3fb2d96bf2962cd2a66354")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 74u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("84ca3f1afddaab7d2eb6c483db4497873eaca0c7f6f220a13cbadf12ccca8f13")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 75u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a41c6cf5a3ce6e69c833f8b65309c2455bc7af837933a5b36d8547bdfe22d913")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 76u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4cbe0fc035466e1aaf9aca6cca75f4f266a9f726144321914233c493c9f44a38")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 77u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2228853de83a535ec1c55011550f833607080c1ed81187598392bc8a7b9dd63a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 78u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8c91232c8f9ba12fa54266d1419f4dca9fecc398643affe5665e7b102ab33952")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 79u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a05bec8663324e9436026407698fdddae7b3c0f7171fd2643afa27b64f4fff2c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 80u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2eeb5c4748b62577eaff0dcad8d0842377f4958367fdd34657054f71606ea71c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 81u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f8a6fb8a63bdcbca5b479d6e5fa08b7292e4a941fe86d525f973b8ccff2a1d73")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 82u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("444492f58b41b3e594b6babfa019d531851d3d161d244ef4f7ba29b7d770733c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 83u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("546f01f26fc45b33434391ba1bc8399d70ea3abe5c193b85ae0c146375784434")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 84u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8a732b314190d826677c03656a8b50723b3054e3f680db881105c74e832e4873")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 85u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a86085f170a30d278a26a089af21febf1a116bac9763c64c5779124d1c36807f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 86u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c2830e6817c3d126ab0503a1fe7a8f1d07504f1e374da548e9fba0530c4fac46")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 87u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0e86c6877746e0bf3771df63942c95ec098b6a9688cbe215b81150e97e819933")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 88u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("26eb9c33aecf579733317798599106c1c9416002a37321874429a8f6f6cec430")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 89u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aa641eeda5e8a401f108c26f0721442c49353c8e27dcf8e4cdeeed8604722946")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 90u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("886095eaa83ac48b7471075490c28a4019bfb874e4577b61c31f8c57b9db8508")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 91u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b8928e6bd7bd8471c0fb27a33c6222e7bc44c5868e9fe1bd004670b150ba7d3a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 92u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5e1de5ca2cc2877c4f9b12016eca948f8d759984b41b3be7bb9068ea4f8e4141")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 93u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6070189c9ee0c6b8197f821b2315538ab99fb5d5a3f9cb426547a3477ed4323a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 94u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aeb5bf873945acc18e13f067eea66609612456de78f8995905e6b1733ae90104")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 95u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d23be0ae119e2bf29853648147dcd2a37ec979fad5ee3a427e3ffb797fdc6042")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 96u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a059b4d0154ba4d8694a19fcc9d8d55fc9aea567f77b0257d2c17fae6d778350")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 97u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("78e17202646f8c74eb314ff97017f3aa6c494e5c7bb972580261fd7637ea177f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 98u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2e17c64e80ede4fc840bee03a34d4380a86c1cc3e84ca895c6a612ce6c62fa01")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 99u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5a77e9ddda595babca52448dcd2c843a8d4c904cdc174fd9b33353b684bfae36")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 100u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("64509f8a72102208c830d4aef96a687276df9222a3cda4f2ecc1ec5bf56ef776")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 101u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("12e4e05b71c103eb450f15ff449b46f595ebd0c170253d9dd404dbe3b67cb804")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 102u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("524c7984d17d66886be689d550dfe5c1f13fdbe302bb55acc6a6ae45e1146a30")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 103u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("604108a5ae92276786b18c6b727405e8ad3b1f8ab0c37b2fd51289a564ba8e7a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 104u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5ccea590a2674ae147b5d1c299ef37b961fd948f1296adb7a0e481a9ff5eec39")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 105u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a09b1675fcb5ecb9663ac6e16a9e3ba1f51a842e427a98b09b8335f471987e68")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 106u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1c1334ed4e292e9849ef6aada6a482903ba751b30d95388e10355ac8b040ea0c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 107u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7c69be88c92a31cd0c80a914b784e344996669ac7db92b056a0726b0ee06e176")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 108u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c6b87641ac0ae9f8498140c71bb52c05283680ae4c315d8c30adfdd3bc1b462c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 109u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("922af5aae8a9afd77c3780860f0ce3b14341f268e66c1f797ad30e4af348443e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 110u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a057daf33ec76b7bb884878e1471738b095f11474ce966bf61c73be3f9c5f373")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 111u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b00d661a6650b78d6be5cff80b3c7805395026c1d296e6a4a0bc0c39ed149179")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 112u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("263aff4ab236b19db609f4f8efff670eb903500bc95ecd1f4878614a852c8228")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 113u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4dc5df2e21fcc3e7ade80a50f11986c66249bf9cea55763a8e8c7506a8e8363")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 114u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("160d360ce30ae0bed5a1269e19d41e39e1e74842f2551920ad4ae9adeb7c3e37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 115u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("84db9711a2d34012e6300a70254014df4d35f61e8d30a83f7372240fe48aa859")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 116u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aceffd569068666f8f373ac5c48e138df26925927f4032cc724a2a25e7083c4f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 117u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("36a906b9bd0e0c91aa681e206f4eba666c0d5b99b7b473ae00a71c637cbcaa55")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 118u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1842161ccd42d34366935cd4fa098ab9b588f0a6178957217164115ef0583c27")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 119u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7e4c197ab00d9f62443a1c9161181598a86d7ec9d527bd5d26482a08e0323852")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 120u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e67f47e3b69decf4e2c88acae2391660cddc0eccd5d5ade6e4fc171bc82fd037")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 121u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("707ad50e55d055545544c9543de3fb0f4eecb5036c9771558bc3e847d276786b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 122u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8ecc4b9e458586f9e083050e73e83c1d0b48809a415a383c794b044d1123f516")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 123u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b0893eeb31b828fdae96a5c403db0411f7c69cf9e656959fea49670ebba6ed2a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 124u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8a58da0548b43df9631026e4f1209e39439700070f34ef7e102611653f699a13")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 125u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3665e539ef02667c06e3341dad0051ca3f3f224d2914e6f5dd347b27bd07831b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 126u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("32d3c19f2f2715b9ecbfb29241870525e7b00d3d614e6545c2172723c0afa850")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 127u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ea3c7dfe91efb72a46c55a3360d7761e3aa9426642c9228068157466b283f706")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 128u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7e28a9a2ab536c516eb0c6f9feea4b6bd0823264b604af3b4ba41d87fcc1da7e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 129u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b2fde70662e321b762f7811b01a906b29067ae1a936a4a3255c98ce7629d346a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 130u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8aeb8fbfc6a6b337815849f57fd4af463ae42305a51d23ede041976b8dd5fc22")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 131u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("64fb73537ca4d5605a9a153ab897ad309e1f05798a129174c60359bf963b5501")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 132u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3269d6bba679e951e04fdac315695d511c0b27c54c39e3c7e116aa282bbdc57a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 133u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c0083f3203b05d5399bc9f31d3e557709c475e250bb877b7c149928b1f07e604")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 134u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9a7a76e2ff53f230cf74b88f9c61a97e6fccb98b955e989fa81798dac9b40401")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 135u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a818e58c2143545b45810ac9ab4d729e91ce910fb556d4855797ea8f0448f00")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 136u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8ed91e1b88893d0c1d4fecaca550319eaef1928d09992994585550f0e3b30771")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 137u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0a5bd0cbbd11ac60f94026ba1e24f32cd6990fd3ffb83df690960dfff227103a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 138u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b85521659db9916408f5496f490051fc89aa9310e775b83e8f33b0bc17dc3d53")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 139u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2ea0bcd523cd89519acad6ff17399fa29aff74d6677555c6e732ef66b78d846c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 140u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ac86d64ef1a77e94607086a047d6ad8a5c2ac2858961cccac9eaee738b215944")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 141u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6411e18ca58bb58a3c760fe1655ebbd38ecb0407779ce7f1b4d4516102287e68")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 142u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("60aab0bf41e1cfaaadce92669c8153cf6ced86ad4e99166653a19798ec80706c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 143u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9c13eacb548c8c51e06254df57765213ea66b48d952bb42a2fe538bb3c742048")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 144u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2c18fd66e1671d0889f067d3f03aeb5222fdce5eb17c5b1fa6c55ccd7560fa7a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 145u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b285014f0a8f63843cbaa1773457272ba6ba5a8e3694b5f145e99e58f63cb85b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 146u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e831f529967b5d5da463ad516037fbdf9afac6ae163aa18c346ea7505dadf606")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 147u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d6d196061e736c6e79949b6d083076b18767a981211ba7ca4ee46a33bf88b72f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 148u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fe9079f3ccfd4ad7ed0f640df000463293b864125efa9232b773fb19f41c5f52")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 149u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8a858fe92dae9fd35d11bfade979d7d6356c03564a09ce9f3057dbdb4598ec4f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 150u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b859f6652b104a40dd772284996b7dbe9810e501c2cd4da0d24d40af551d7c01")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 151u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("843e882fba657b9f2137fcb2dd33994cee4e1ee458a46be5d8c57ed9e2634a50")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 152u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("66c1d0213b7d1215c9e154a2d37077751db62cd5debcbd50a52c8a8706f9cd77")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 153u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("be08b2a5771c999fea5d95525a693a05a528ddb3b0e724d95510730378723e6a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 154u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4436e5ae9d54ec1474458829482d92596214c6f1645e3698d4c086cda99de03")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 155u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2818e58953ceb655a1b96751e2041401dc44e52b2cfeb926ec7c2d2a0c0dee53")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 156u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b62f0b8ee6cf2b9e1ca9554be3dec7b39f94178dc0ce805151b2af25bf2b5348")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 157u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7cdcf66e0fc457ec3c9eb143c890387348c4b9d3e53c5f6f2582c6f91b0d1716")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 158u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("587be763899af6ddf8064210eb34a2829d16061afe6ead86c9440bdcbb35e90b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 159u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4ab612af10fa3a0694590522da0542408e17f5e5110df0118efb7f986ae62023")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 160u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b4660629288262f10a9f438b685202b522ded323ffd7419ff637aeae6b134133")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 800,
                        initial_credit_level: CreditLevel::Eight,
                        rank_in_initial_credit_level: 161u32,
                        number_of_referees: 34,
                        current_credit_level: CreditLevel::Eight,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c69be6717bff85948e16040d6a60854b300046eed419802566db3159ce772358")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 162u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1c1273ca128025b3c8ee1f55aa548ee9f05d89d26ac01470fb3a1fe61f0d0735")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 163u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c251e545337613edf39a9292fe1a56235be118319956edfdebb1eb284ee10c58")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 164u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("229a8ef3ac43eb0aeaaa9050110f08d3a3efe2441060abd8d80d02eaaf426a59")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 165u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fc33fbe7e6238466328a28a27fceb791149ad4e7600181a06332a4c1f0048472")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 166u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("209a02d88c4233cd676232d5dcc74af414535a107376a38cb8744213b674517f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 167u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fc8975c5f5584340feddf08354a2737b93b14585865e4894540b9702e8117c77")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 168u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9471e704d203bb125f0a09a125ded598c75ebf6bdb6aea2616f3a9dc4966d175")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 169u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("824359825d7170075568164a3ec55eac81bf363fbcc12329d88a37d673169b4c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 170u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2e1afcd9cbc2a15943f2407c5e2d740b9ba1d1b0bad30f8b673cdf3f6ffca151")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 171u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c07b75abaaa6102621ab8a9762710f89cd056157ece2e5934e1db37734dcd871")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 172u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cccdafe0ea5bd5a257d110a9de37c4bf2315aeff9f5f3496b3ba6afc8e832773")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 173u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3212064805b33be0b49be875e89a9c9194bd198b64fe63196f1957bca653364e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 174u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8efc828a754d1d8ba9cc634c8ed53d95f85e13359bb31b1b7bbbbb004f2c1362")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 175u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("62ecf98feec4ebe525a26785534e96b6610c898b3df3e9dd31bc78b71433c67a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 176u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e668a34f324643359b55b7dca0ed5b23af7e5ed293333828d2d02dd0039ae113")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 177u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6e8c75ca9e7417963c77822db221597dfcbb3a6bea251e720be111c8a656066b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 700,
                        initial_credit_level: CreditLevel::Seven,
                        rank_in_initial_credit_level: 178u32,
                        number_of_referees: 25,
                        current_credit_level: CreditLevel::Seven,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("301a0cc254fe17a05e63142e7a37b46922ffd410c792c606a6e9983a942bf318")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 179u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8abd39cb650cac04019a542822eca69cf460918f864b9a9ec41fc816cadeea02")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 180u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b4f846b10fd63eb82a81cb79952341059b9f3d4afd2e832588a945898e8f4f75")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 181u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("04ba7d7d876ae1082750b5a37341e8786795966eb29dec5276b17f0c99a8c056")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 182u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("183d02e5a22cafc6931a2ee3b4146f2f560ae2fdfe296a0059714c781cff5414")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 183u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2e17d2efc414434e050322974f4446c4f1890dab42f5aeba78d6f9a84f9bc615")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 184u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ba6f07182d511624067cc87d40c8739eb923205df04ed193449e088bf50aec1a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 185u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c28d1f2a6ea70fb44878b2c885aaf8d64214615d09cc40cc07dbea8fd93b6a11")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 186u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c68b835b181801fa647a7071a07f9204775ef4c1764fad636e3477e44e2c3623")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 187u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fc6dfced2e81ab859a32333d6bfb2c91894aebde6cfd8593dabebb1c63572f3c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 188u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ce041dd4a30d332b7dfa95084c87d0cf4587a7d86f04484ab9eb95c976544b1d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 189u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4edea4397a7f6d65b7fb07bfe525b9779f012d3ca91d91466e6fd40582d003c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 190u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f67523884cf194e0d40c95ae733626f6b6411097a993c014ae9fd0cd2e682907")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 191u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a845d3f32a4abb3ccb71cc197af66135f2ecaf8a57e582531898f3c50319c37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 192u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0cd45ead6ca523213cc8d3937e3cc48e125d35d2039b56b78e13ebee86ebf44f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 193u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("62fd084afffe4ab93f20a4a068397396429913b28b0fb6ddb692885fc43aea7f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 194u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3a030b62d1b80ceed8c9b38ade126d6a833db9875aca4a45ced87ffd2f90690c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 195u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a269bb0934b1812a340ff66f932fb574d4997ffe664ebe53b5c7acb2c46def1d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 196u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("166e8045205369508ab9a493ac5dc4cbf78da8253d0b77dd0e3ba5bbb51c803f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 197u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e009682afa8575986a8e03628e115e8c7429bfc4f3d1553a0b95a7c2de16147d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 198u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("24785540855217c5bac5e87babe1b8f9ad5ae8fef37bca2eac683c73c23c157d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 600,
                        initial_credit_level: CreditLevel::Six,
                        rank_in_initial_credit_level: 199u32,
                        number_of_referees: 18,
                        current_credit_level: CreditLevel::Six,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a5f75023a527893577defb6137d003bd725c6ba7e5d7d1b31be32dbb7ef192c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 200u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8088efa63facb4fc11352601814ef37b45ca56c114780a3090fe4aa284c3ad5e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 201u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b4a28800bee3024bc1c4102857507efe7018e291a3a77b6b7562d404bc62b006")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 202u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0a6e74984bd2028d0f7b8eb30c426b364e9bca244b053d2f3d8448c1e116267b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 203u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1e5d9740f50123c7e824dcdfd9375dc026d551ec17792a195a602bfd27856e49")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 204u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("88b409eaee61cbbd1a302f9d0add41a5c7e95f66db0f64459e104856160dea64")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 205u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4fce549dc96c71947540d879fa4b6825a96f70e7c6ddb1b3e3e8819f2a9315b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 206u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c0bd30a9108ff95eb7651b6a2f2a96aff65e63d5bb79a31511c91472ec86eb75")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 207u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a0f7126ef6f263b9e417af630c7859c31bf4228bbe240c9b35e10663bdb6e539")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 208u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("00d12933cc0b4244cba085bbe381a750642d44a359160a29e2fbe39cf1277971")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 209u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c41ad8f2acccd07efd206001c8ff6bdd1cfc8d77295dce0541071d11695b762f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 210u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ac2549cfbda27e159a8a7f2bece40944e562b0a19cdf82e640696a7fe8448e14")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 211u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2287592734d1dab8b729471e8d74125070abad8481b5bbc031166a5b734cba14")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 212u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("500a3dd4560c6ebb24cd98785c7df6c3c712b47274746a9e714290088e71b501")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 213u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8c67ae22b188574aaefae75a82aa2ed68813d2a053ad68275f3c84f26637d30c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 214u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a08cde47377b3ed7a7ef6b71ea82f4eff295273bb6ea6b0893ddc4ebcbb73041")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 215u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("88a03f6fd9b1692964816439a07670e481ed6d49c29482f0aab2e32bf54a0312")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 216u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e2fecaaf54427ff49363c8bd981e786f53f7c1030be4923f38670d127f92bf7e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 217u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("26ea659e6d4c5acbddae815e19f722c7253d7b008f3cd969a73fc0458772531d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 218u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7e4c9ddf2c042432963780f2868f022aff76998e4ba1f9d6dc6015ae887d4128")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 219u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fef123a71341cbda4f81c3d623e582d697e6db1f7343115106ec56e288c56758")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 220u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d488f0fd82968320bc6ca9ce6defc5636dd7ab6b4b64975f197dc8567c23fb15")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 221u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7c2e761b02e56aa47f9c61b21c760b709cdf7506d623a91c5bc52115acb8887b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 222u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c2bac7ccff06164f4bbe9a440cf67d3555b57ac3c6e01cf24c725abae73d351b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 223u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("829375f849a507a1d2ec32cc0ab15e53932525c0e3bfd9ab637a46e46d59a171")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 224u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7250a73a9ba84169d6096c2cfda41035f01ebef48d6121115039813a5f218f3d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 225u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fc0886321ec5bdf760f172a88ef7a7fbbb9fc32139ff5c49a4eb6a15ac671526")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 226u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c207e50865324768d431b9ab54ffdf7f2541dcefc18025beae6fd4dc5cfd4f0d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 227u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7cffab5cb9fd15aabb6f409d1253271b552d07237ea6e58be2e541828301503f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 228u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bc5ee59c5d53446be629703af25d35e5509aa77ea995db411eb1d569a4d3ee40")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 229u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1ee33df348bcbf38bf8ad83ed5c90301061fcc861fe26017d18c1841fe234c57")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 230u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("626717a1bebb8f8040f466fcbd4561b2eda6268438a7db0544b21563cac83a42")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 231u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b20f561a75a10fd35d0315733f30c5b708f0c69dcf9d977be173ca08c65d9126")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 232u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d2d3d2daf39b0c76fb9a5c4c9c0b633e3e76af33becd54a07e5e82d8e4e1c76b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 233u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9c024fcd0a514899e58285ddd2aa170b9672d318adc4c9aa057e414f0a33cb25")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 234u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("844a18e360331119199802df9537cd0ee27c6522343c4b829a309790ea5f1175")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 235u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f8e88f1513cff221489656c34f5f7df8e7c0eb7a04e166bd7b8e4b14a2fe7526")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 236u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c0bf35c0f92106822ad10a0193b98151804c73df6de10fef296ff3cf7312cd2a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 237u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9c88f34847a0ae459a16edb4eb17fc1153de8ef5d658d2652d41ad341c55dd7a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 238u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("565f004848d1cb0faaac6a33bcdf211440e0f9d460a3989c3b0d1a0374cb1213")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 239u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c4ea3f36821fe07b02f78d02278fcaaac9bf9cf9f0f4234f01a8a3545e464b76")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 500,
                        initial_credit_level: CreditLevel::Five,
                        rank_in_initial_credit_level: 240u32,
                        number_of_referees: 12,
                        current_credit_level: CreditLevel::Five,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f62cf57c825eb43c190474933138527a8677e65930064195ecafde006089235e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 241u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("58972daae034710ff80a6ebd9c3df978461f5c9fe786418594cdf94f677a293d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 242u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c0c92c12d62a466d5f42eb89a97ff93909b8aa8db54b521bd273f9417a4fb435")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 243u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0af4df48837634d721038d7b5ffb4d733e9d4106bd41f78bb731390e2f2d1959")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 244u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9ce49ce0e56737091d846e6aa2660fc777bf67de11e4729e6df4bf12a7257931")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 245u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1c6a4daa34b784c98812a2cdb180b21538a9ebe6ec379b47115e33b45274fd14")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 246u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4f82ead1ec7b5cb59390ffd543126bed31963f88b9985368cc314a51457382c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 247u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("528a4deab3fa5823232b16640ed6c9134582bf823331ae40c97de1533b798537")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 248u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d2fe0e5028d200ad15d933f4978d18ed6ca9bb00b4380ed597849e98dde1030e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 249u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cae2408f7296062c441ac5dfdf312ffe2a706573f96bd5e632ba19dc5581ad67")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 250u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e0695b0ba321a6197172c43d3ff3fe059ab169a37c75430585f959770b40ee02")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 251u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c24d7898b17e9baf447d48a3b326adb502972f45b354a93d5530449f2387b70d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 252u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("06d33138928fde09ba1a0009df54d9c737ffa8334d11f769f59deabbcb8d8324")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 253u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4e476ddddaf3e35b4193d14fa961b8e22a0b9ea9f02b04bb9bd2f0d0a658567d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 254u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("740b59a7c1c548da867255a240dc966142adb2afdcaffeecff1bb736e09a7c1d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 255u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("46668495f27f7c59caa907b1c3053baa2c4ad09598040185f41833f4b3fdd25b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 256u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6af78bed3c6dd95cee5cc4da7e1e48ad16db28a4d5df36ba29b3a93f1be1b256")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 257u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("30fd0e18b00461fe8cfb754602f6d1b84a807087606ea866bd6cfcc23ac18339")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 258u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b6e352146a04ead09ef5fcfeb451546ba7b0f46a9e930b05260e55eb0d268211")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 259u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c04a80c64802ba76391a5485248115b5a549b9a8b5bc52254dc3d14646e4f14f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 260u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("545cb09a45b1ddc7971ea96d565aedafb2f969f6a2f5e9fbe3088fedfd31191f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 261u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f08355508518cce8237095278162f94740a1ca11beac5f268dc7718f8efba076")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 262u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4e79999178d8281a7b08eb583146c397aaea1079c9fdd29d1ae433d28f61be61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 263u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a254d11258a300e49188a64c7d0cc6bb98bd442c2ce75f9266445e585bd7b59")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 264u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("665efeaee62f895d20a8652b7c4cf783f7633d9f0e38aa6380de8d8f2a316a02")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 265u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("02c52bf6bd7872ea97aeb93dff6a58d70fc6fc7076a3a908426784df8ebafa7a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 266u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1010d3e4ebe4da62850342b26be7aa2cf90c08ad51c431782fc5cd886950000f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 267u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2e76cd6851c2a484d0093dd64c42afdf869bacccde57d10a389283e9ae269262")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 268u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("38d86742d2cb9db849e5ba944216c2f9d7708cad34eedaaf6c064642840ac47b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 269u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e45d8fcdeaeb5b964b377c6b0aed0a66e40537dd7c8c17bfa9709cd5cc3a7f64")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 270u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5044d071ee89ce51e2f2a93b9168190d5b75c2ec144d1e94155e66ef5c163c0a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 271u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d23cfed142ff6d6071dadf4db8c3ea5f043f66ce59b435730ff6e7ee095bbd56")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 272u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("569fc7e017ec997fbb5083845b7e171e2a728951b3e7c1988a6719dafc91bf22")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 273u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7c12841532e897b29dc36e2e137bd63e33556e09b4a519f29106015450cbb07e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 274u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6ed9fe71cb951a63c18a37861a55b42224b4b6961fe07f0b9e02f43d3b49d678")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 275u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("920f796d63767a0276e009c97c4362574a82512afba9834fd00c56d1d02dd55e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 276u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d8eff273cea741920c051aaef2ee741ad6c1494ceaa57e0115a0087f37eed53e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 277u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e26bc8071470f4eec5a9958c56b59e76ee4e16983c8bbede7c6488b905acf47e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 278u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e00076f746503b3d0c6f5f3677622d61080ca3f837c73abc19c4688bc9080179")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 279u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ba35142843874460234d2ca71161b2c3fc9fd0f5904ff5df212b0ad7fe29fc5c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 280u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("42fa284184260b94e86c2b51e7e6fb7534cebda1a5c217ab5ca2f06f6d855530")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 281u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e0e292971b172bb32e51d6f07d109791eede19e413a5e0d054a4d770f0cfe917")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 282u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("608108a5bf5d15894e38c5708817b918cc97ac5b6f23b53d100afd15a13f7065")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 283u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fe64bf6097be361bf3fa3b2726fe8bc25bf0d11734a3f3f952ece9c6d6fa6d56")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 284u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("023e8d57d58142dafa9bb7c6948b991f60e998e8c70e9cbba66ad27f6edd8f62")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 285u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6e0bb3822acba2d7d75156ed41fe1c1718d211b4e96d997625e34742d03d9f5b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 286u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("08abb9bc228dea275ba1fcbfa99ff0b72a1ac1b304e967d210ef05812ebffb56")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 287u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ce4104ab68232f90c62ff15f7a910f6ab4e3b9ed281739d326d208e58624612b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 288u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8c521ebec776bcbf81bf9f12b447005a9768eca54d88aa4f7b801efb212c081e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 289u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("22ab1a930136f572719292bf5a66050c0f3c6c3b42f3d25bacea45b0d87ce436")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 290u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("160747bf8bc0eabb0c75dd0ef3a26c446f3e14d2f7ff1c44d7030a22bf2a216e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 291u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2815a98e0c937f478bb1fa249004299c2e043963bb4e59218d86c783cefadd1e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 292u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ba9c6cee3b23045ef354f16036ce9f4c97909a4a7bb23b3e4a39b946f76b862b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 293u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aa2909ad1c018f3ec1131e27e1c657c5934d112208f5cd90bf070ebb84437328")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 294u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e0c03f628ec7f37b541f87bbed48ab32b6c8cc894754c998ca492d9be34fa57a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 295u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("82555db2eefacab45208d5f1ecf419e8afd0894e0f7560eff32faf205b523f1c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 296u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("744059f65075e563e18e7bb837b67b94d9b4f59a4c43e6f58bfd0114b366e87f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 297u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ce55d2bcbf1d7a6fb66b1a0d85e4bcba26e37f9c16a0bc054e3f99b923f48934")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 298u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("341f6c12c0a4367b0cff409d424ba63f74d6b6b765a9360dd9a6b6340b30062b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 299u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3e0edee6d95630395565eaa266f981967453532895d3ea17cbe297ca9067137e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 300u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b0733bffbc85b964ba7e57c0dda22a6f7642648b539deab7c734562be4cc6b7b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 301u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("268755860985f98859fb41b54a72091b65e003667629debe5426215c9609da2f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 302u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f2230901ac0cfc323f160537f2e6355d459348bc8e7196b9105f594543314418")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 303u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fe4f79adf38e7cf31dbe94d541d32ef4a87e1b7589106ff3fb189ef4c5c2c32c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 304u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f4e96de9ff6229136c5788077a8dffe110e9164f10248544c402f368e8176d0e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 305u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f42b5e72d33429b300701c20c2617a5a54ab82edd5456a66e9562bd5fbe64b45")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 306u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("90acd31199bb77a8b06e5ed1b5398ff207ca0a80dd196b3fb2af6a08955b323b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 307u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("766df11ae83e233f4c377ea5635da16918432e56370b5c5bca192f30cc97ca74")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 308u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c09eebfe50f930bbd188d7b8d5967e6149bed6712ef94e9ea0abcd90da0aa16f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 309u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80cc8759c3a19b4ec395b426261c4a83c1df54f863a99916bfadaa91708c1657")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 310u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4201cf872c8ba57f611c6212835b28bac7ed3c166e83539aa98917b1c5666059")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 311u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0c968e747a3e4fc756fb347741956d40fb9a2df763a20e8aa1b381fa4e8c881d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 312u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("622fc1e6d96ca932c2a0e853d91fe4296b50d69a4768828bc4345d6be893d435")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 313u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("285eb5843c9e6bb322b926497213657c0490c66797591eb934770144022d1614")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 314u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6a471d69c9868683968808e40b99c84dcfc1f5fe6972d50cca70624c4bdd187f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 315u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c2bd654ff54f3d2e8b33bef457654ef13f32217f70ffef5006afc6b18dce7033")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 316u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("22c25c002f289283d4a1c62d03468fa2ab86b9d0b374b05329af27f36c40cf39")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 317u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("304e334da2ed93ce89c205baa4d604550c2307cdbb398365e20cfc2c5ca28a60")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 318u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d2b5e1305172dc008897e7a3e67c717ab60d46e4118381d710e73ea1b25f2938")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 319u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e20ac0056a7004377f52d9eabb8fc8e198361a18f981c2d21a68eeac0c25d422")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 320u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("36a684322d1e2c6f9a99f0e901c2642809a336b5369b98bb0fad8c72d83a455c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 321u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("628d42bf2b63c365ba48ff9f27c700c901ee8add09266dde4fc0cf92e0493015")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 322u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9248ceefc547f20fd65099493cec2d0fc643ca783d123007898e10b62dae8136")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 323u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f403852e31ed5a9b059f83dda73aa856501200e617f6800f16f17f35b76b5344")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 324u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da0fe835ba683c164d85c2fca67e26f4e941266d97fd962c74fe7bce7aefbb5e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 325u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ee35bebdeda802b26e1e1200979b7c28015fc3d7e1082e442da99e27f781a30")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 326u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("02f46b18948b23a599ac02777741b2884187c5f705a05a7da71dd5597abaf814")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 327u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("486479f8640625ce43a8069be8f29b6f7f3f5f19c17ea4db248fe26fc2ffcb17")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 328u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c80fd1b4a5eb423f4e97dffb5e9376773d0f7d1a66472336d12fb71562bb5610")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 329u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6811439c517bffdaaa7099c44f2348c0490b199ccab8544e3d1dfd7c4711dc4b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 330u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("623b899ee71b10d333109e4b754988f6260032fcd47ec5b1cad11b7ca6b0e11e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 331u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("941e32ff7f5c0541e4f836a62694a06600e9ecc690c861ab293b3d1576d4bb13")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 332u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("30c180065db66a071454d2f9a140789389ebcbd59b0dd2245b66800bfe00362b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 333u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("18c7162ae2b916737ea8138bb3f33b64febb4f4d7446c29eb294538b67e00a35")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 334u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b440ff2611819b5901f63c37a099a5298ccbfcf2d51c48c9be74f03a24db0412")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 335u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("eeb59ac718fa5b3743dbad8522da9889d687e7c61b2c13ca5f87f67794ac966f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 336u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("baabfc4e833c85bdbaec472997cf1a78cdbd6f20bc23dca6991a3d87588eaf3d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 337u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7cbd8f355cfadc4aec6b16e9c32005056a0281dfd73e0bdd79129c34c4944d75")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 338u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("86d218850085af3a60cecb5444921a56de8a227cf2c61eebbb8dcaa27673294c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 339u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9a0e6c6c820419439ba9dd58f9fca19d82bfc695eaea2b24985fa958a093c966")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 340u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1695fbb60674232a32fc831c8619de6e5b27d2d73dd86d2ce11be8ce2d9d285b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 341u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52c080b3b178fcab962f62225b822744036898d553cb966fa08429fc61315037")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 342u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0adbab60359462c4da5abf52b5e190cadee878271ee3672ae79b4c451cf87b4f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 343u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("98adf24c540de3aa34d1016cbfb9e867e3b90da63d69e0152f82e74d5f807225")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 344u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a0a863972cae0d55e517e5fcfc37d2fc7a44f0ab52f18a3422dfc6625b361037")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 345u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("229c36e5a8c38299bcd5885b32196847f63c772c514cf1bdd9362993eeca0e68")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 346u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52fa643d6859b76eb720f7f92c12565ac8db171a1c11fa5bc2ee6104212b7e71")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 347u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aa5e609426590eddbbe16b0ad8132b59fe7393023e3f17ee42e849fae8b98e69")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 348u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c631f8b9b59b0a9c96532cd7897984aa4ef9527aa22c3b018016672612f3581c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 349u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("34b15789860f239ade1a0f74015b5e8f5cfb3327efb2057ef568ae257685032f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 350u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9692d470a7a6fd1be9f127cb254ae594452d6d2fe8ef63d96958af1045245d10")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 351u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("946509012e30433ea912e72f48d561881cfcc694f84caa4f0e2fe6454b229920")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 352u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a66afa3c0c3fb8b6290ea7a5999e347b472d6b0a32f8e078bf175f35c158eb08")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 353u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("02cd9980086eb9ad9bd072c7d18a477f1839f176cd610938fa8fd0a59bdf3a68")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 354u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a8278283a1f7ab158c55d4cfd6ab26c218da17b49d706933ecb92f0366d527c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 355u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("28b597ee9ae13bcc2c47d2dcf40655fa87ced8fd2d484a5b64b278860b707b75")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 356u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a69ce511504e737147d8dddd808e2141033ab767c03acb35c0dfa8524e9fe4a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 357u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c085c83e703e1289f626c57dd268e5504ea58e3437cb623e7a86137b7a70fb03")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 358u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de24f9204d9ead2d013cb106ac993096d75a7e2e221b607bd5c178e9ebbfe177")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 359u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c6e315c6e53297f5f309ffdc74d880bd6c79507175a0fb6dd8bba6b378f8500a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 360u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ade8b915067035b909b79cfd5834f2f63d3204e38fb1ae65df919e352ffc215")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 361u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("42cc8f72e30317743fed0870913c29227d71cb48875f43779441539d3d79e170")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 362u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("78e743bc0880f258191f5761b963f967126ab6a279466ff4624c84e5c1eab922")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 363u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c0557b2bf59305b6839788088b46f6eaf5de2d473e67d9627c25af7fb4b49363")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 364u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("663a20af82d294936fb4fbfc4a2b62c178d9003254ff8abfe36e6bc48e9d815a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 365u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0aadfcfabfd4d11ab3d4af281416e51138da7044ced4af78eb22ca3ce43bdb27")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 366u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4f8c94b4946d9cb2fb266e28474b60aa4c9ae66a4a7e941e523c33f5cc2883e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 367u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("660ddba3c50dd4dcb4dd311bb6294cca0c59e3ec1a9a904c727e120c280cba6d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 368u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6e8d1e362c2d886c725c8ea60a7351958792e2242f5df5ac46f2fa8438bc4e1b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 369u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f85af96c39742eb3f164453badaf5e379a5dacac96e0bfa77ce1e308b0ef863d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 370u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("be961f3f5749b13ff6bea67c8ae915103cdedff6409a95c8fca449283474d550")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 371u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e2e5bc0109f9d78101e7da0a1b73b58e5ea8c7b0c6e6960748a2f0a0be089165")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 372u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1c7af39287348219f2af26065768e9a6f979589b429bfe624c533a4595ae852e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 373u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9a21d78dbe8dcf5bfcb4f2f0162b5b7f6deb225ec3751474f473320080d16d24")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 374u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ae36596803cf57908b35c3f6aa61ccc74f9d21f027ccea0d4205613978cf8964")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 375u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("be7597d9dcb96e5a7efe9de24f81df0e410e76d7000df6c52af94df6155b2560")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 376u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0261d6a259e1cf1a353e12720f3af28d2d219d39182d94e948295066a3684137")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 377u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9e5d09ec248a9a287f9b95fe7b7cb9acf8856052e275abe0be554f5c3f87f134")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 378u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e2f768a28f801371e3cd594cfa1722cf3499533c00311f17e6ba4ee45d1c661d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 379u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("627a1d243ea57e806fc8d6212f8f7bd9f5fa54d0a15c0212031b2801f1221737")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 380u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f4c7961f20da3c68e8baff25190ea60bf2e600db738db9883a58fe821e0f673d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 381u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a2477228f711457901a0bae4afa72ac6e0bd792f8e1eb5645d89b856ac48ab50")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 382u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8c0445088c78361153850d0ffb2dfdf125fa25f43122d43fa5f8d52aac23907d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 383u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ca864f313abd596df5f7b659bb14e63e5e1d45bd1fd7a1e47646752b4d572a69")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 384u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("56ccaa8e41f54399ef83a6ce1ebe749788560a97bfbf43728a1eb01d2504d831")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 385u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0275b6423366e2d7ddb9bd04e179b445498e87d929b06e935a72abc9e295b801")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 386u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d0c18021a5a02939bcd30a403b5cfd4696787cd01a8a2a359270eae36d79170e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 387u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("60a2495e74e7cfa9b7a733ec06afce9ff42c45261b4d1c8502cba422ac35d40a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 388u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("04f240dd346d5b8bd15ff1548b6a0aa41e5db3f64b403f1cd318fb42886ef959")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 389u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8caecebfdff3340e916596c072fdae033953d82a2351ef1957662b170ef08b02")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 390u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e817d6505ed2a910e1d1cc62835ac4bbd73b3f91d92563ffc1279efc8e02312c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 391u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9c3e076ad1a51f07def37bb1019d20f67445b0047b442d95b55ddcb08b78d073")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 392u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b2b38c18e3fedfc4bda3c2c4d67729381f3333f9e7013c55edf4aa49f0ea712b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 393u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("726fe4bf60f6ab8ce04953982ede039ee587a2f6056f70fb103b058692de4e3c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 394u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f26be61e304974855f8b72b4c6bb8c4d614899e990a86f9cf2e6925f05407f70")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 395u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("901f67bd59f1a95c29fb6ce14fe2dd9b1f3bdc9e8f7cea8789833c7ec254bb65")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 396u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aa4eea0bb3f3cc8fc3753373ea98192410e3f3dcd364deab65a33d084edab40f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 397u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("106d581083f3aec0a7f564b6a468dea5029bfc6e86c2d89012d87e9b010b360f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 398u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c8d79bbf8548f6c49910bf9730b36dac34cdd3d97b51beb29a079572e3720628")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 399u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da9feb1fefb20c138ffc5d7de95659dcad37a9d7de07988327131e2f69dfa213")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 400u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("62783334a660cc09a5304a1a07d9929fd5ec6b912dab235b1d5f7032b3c97208")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 401u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e27ea330d6a4475ce7c08c07df85f275f74a1b0e6d23bd8a859f06b3683d100d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 402u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("48385aa2faeeaa8f8b7719d7eec371a270dae52d64af0e04ff2add213bb52e6d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 403u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("34f49c198f3ce71b7505a00dee3847140a1beb7772f37a00b68514206dec2b52")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 404u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("640b352fd3ceefba2b9fcf0d64815eceff035aff67c18f1b322580213636da76")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 405u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ed7672e431bf2ecda36327499f95d3e4bb4fe2b87a0e213c92e48b9954dda37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 406u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0092cc6d161ed487d3424c01ebff431e769870b207d204ce9bc6dd7e574cc71e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 407u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("302855910d4957a995cd7c7c3a48f0dbb5ae24505cf3eadd81004b73c1c09750")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 408u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e66d03493ad31f71c0c71607c785eb8121e06f1531e963291bd5075a3b437f11")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 409u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("60a8c92f8757f9c1f51462e900dade0cdd2b3b2f41beff49a76cd277ebbd5c35")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 410u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ccce955924a3631ab670dd13327912e0ddc8c3dd6c956c0668faa60ea164c870")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 411u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("505286926676f589b8cad151a9e6427f9734f7ea4c7b2e62742f893eaffb5430")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 412u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6435d99aece005bc62e05fc24f3441d4be07bb795f2ee3a6ec5d851b0fee3c45")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 413u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f4b8271086e27d1423e867e1e481661a10cd75a9fbaa2559d393a5c00cefd617")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 414u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7817c77d069c693eb2a3a7a48119ed24100597e51c69ca636e5e54b3c8961331")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 415u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("98a134bf26c099b7c48aeea62a26e170865c65640248815d91c4c4411e9bca17")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 416u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("403a7c22dcd1325f1620243d1eab02f52b61b5374c44e74edb40f5d15adfd664")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 417u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f84c4a2e42cd2f1f4e60b204919be8b9bf9780dfe6c9315a3b9fea03852ce653")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 418u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a2b4806983089640cfdceed445466f0d9d2f8212c94aa70060c39e47fd7c913c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 419u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fcf1631f3607e96442ce9b670446814ed7b9896747715b37b9eeedc4f86cd53d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 420u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4259ba58d16992bec71f9f952746be565890c35fcecbdeb4acd638fbbddc8619")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 421u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("28429da73ed1c14f31f9a60526a41340583b3840e9fe3ac39d4375e22b87855c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 422u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0237190bf57e5f01da13be4e29849536ed185eb8dbb62b0c6ba244c773036447")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 423u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("68fdbc2387dc1e8efd4f7411e3acdac1f3ed9be31f4138dbb4c8937a598e8662")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 424u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b6f21c78c1ea0f7484e453f9f4dbe873ce360fe240b31b61a5a460bcbe8db642")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 425u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a8223c730d3f8f8ab37083bd1e18d74e85da9abddaccde1e1dda427d6b396c7f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 426u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c01dd893e5e8b256ee44bf21f77d1310248d3c3559eb4e29a454562f02fbda6f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 427u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c2918e5240217da9ca4c8796d7f76a09f93025c1125e56754b869c659b96950b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 428u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f4c9b722cfb239def6f1e60b3d4170d7c9889c98441303ac8dbb17e422ff287e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 429u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7c9ee104fc9fd4bb1c3835aa2cd7b2dfbf2b117640d51ab7195608e02613da23")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 430u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de21337b70c50e13e4d9bd07156d73a50c7579c8286eead310f266c38180bb57")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 431u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c21ecd1a570c3d9d7c237265648ad7c717e254f74bb743755f4bd54e2f04a11d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 432u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8674dc07907ee846e47f4db5edaa5f95358ffaa8ef29b0707ccae5255ad2401e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 433u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f0813f1c5c21336f971846f6f24cb83c8f3f1c7fc13ee3e859a723348836121e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 434u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7ecb182e56e81025dbf1c98631ad7ccaad9e4dc66aca9bc37ca9f8baf55e3527")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 435u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("067d86853878b315bb754e7bf248c32a0470e790c3cd8c79ff75e634f56e8e28")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 436u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("28f319bd2ef9cd9f83565431e77d1f3300d3eac193d823251c69a37e59d4fa41")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 437u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9ce3b48dbff3766ca21a808d497ab870c119bb1242ada314945afa20680c294a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 438u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("eec7452c89256071162b8094bd731a9e470c16147f76ee59f94e493a528b665b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 439u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d49dab54f0ae14c00c78b634f706576b6694a52917783086b66bc60fca02697f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 440u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52abdce34e9ae55214549743a80aebae3c675145b94f69340f453ccc653e556f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 441u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("72a6243bba96025916e027c2ec260f9de9f47106f4864872e44a224ea70fde38")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 442u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8078d2d321d5e0dfffb1bdc2f726797ae73aa43d23804540d001de50142b7148")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 443u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52a43df99de7cb437e3ce8b1130c61d7eb3a3cb53ef8f82b6425610d24d7ee72")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 444u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec1203b108b1a48255cd226cffac6876685727e04c32bc6eeabfc6ad07506608")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 445u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("22581b29bfcf77ae456d926867faa16622f72d2850aaa9eb2d10298a9808c75c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 446u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d2084dc47c370855411fcca1f263ebe7a2ce074d7a01e62cb17b29edab50ea0d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 447u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("caa4d987c82833c1180e40ada3d4edcfc302b88497ba25757339251bcbde6542")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 448u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("dec460da09e55f53ab7af37e52a31c3ba67afa83b54ebcac38c08fd93784a94d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 449u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a5d02290399e758c9154c870a76bb12b823b9db2cc77e6293611dee3501a648")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 450u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b23f23f2124abe8920ded31a2f510115ef1172e87a36b2756bb3c8443acb2770")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 451u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec6894d8a0c2cfea1b0e1fe9abd3b4118cb2ece6982872852bc4aab195cd4071")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 452u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("349ed1cac9d4f82b279348f323c7792f37aebde4d33e6bf54e1cc4571e6fc34d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 453u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80809e943cc7e20df3f8a9813f5a2a38c08381dad8e4deab85c704cca7ddc533")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 454u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d8d4893a03e1565d341741ad8d35443465c2925a63a945236e3ddd16ef433a41")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 455u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1853ba2814fb4bbdc4340067c91e02a532af460cba88561915feaa27907e7215")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 456u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a0a30edf115cbef3a2191985d2ab177d1b84f3eaf47d3d7158419587c0d2be1e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 457u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aac88736b15e5a45098298340159f0351fb3e59560d675d2478590607cfcf91d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 458u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3cb8d816fcdaa66940fa03f74030d4a925439307ee995b22e4f192d60669c429")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 459u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b02bf3ea45ad381c9540e61a4528e4f5bcb468ae644bc8c5e9ff3efe8ba99f1a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 460u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("08d05b37f6152e4947b8e51a1baa2e098050e433191b8752f815ae6b3ab2ce49")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 461u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c4be1c9ab0ea76617983d1e30c7207396aaddc3e43352b5921f5f512532a872e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 462u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bea03b671d372c9f8c0ff0d2eae18ef1ff01a349d64e0e84c646af0034b5c95a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 463u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("be519bc1d95e7cf7de59fc85b6501df86acca8ffd26eb7eafe79f0bad3755710")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 464u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f003857565d39db06391b34165ef6ae2c55f878502fe6d8eab47bd9446d14e6f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 465u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("84fd334ba0d252e7a941f746b2d0350f1812cbdac38e180e72af4930dcaafe7b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 466u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7c5d4f904c4233befc0a1be1e3c2a15456805d3f648c9c651abaa1378d92c607")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 467u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1ac8ed89846f62dfbe9871f61db006efd4d71cc24551139969f76dae73858e6d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 468u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2ef795c713e90bb38f78205dea8838c11051449a602ac1588883873ab2afdc2c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 469u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d0422303b2388642274e07e3294c6c2a27edaf6dd495a243fbeddad21d93875d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 470u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f4a74e77a8a7a0d506ea7fb70cd0a8496b22d8bb98048965110111a6d1d95812")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 471u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3462769a4c36e84384f7efb2c53e9f7779c5509a2b7704c7eff8e8f9a77db903")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 472u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("eccee0a3da9bf7910d1764b0b6c13615b677753fe56f79cc3a6f4e3bcdcecc04")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 473u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4402c217e38645fcddea0785f68048786ad4a646a1eb828af2c4b7da7912155b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 474u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b49efe5504282df8327a7aba10fca6ffa83668fdc88565edcf1f365a786f2251")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 475u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4e1f4711f371f01015ccc8c8b784df9a0b5a831d079fc1e116e7b7f79b0a815")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 476u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("142f1b380316b10705d818fb66b9f95fdd1de28846149662d5d68fb612174a31")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 477u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4424b91ebd6fd431d7863c20058e8620be9a9346f25cbb4900869e7ce24de7f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 478u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8ec9fe453ac3e773f4bdbee07baa74e257507707e3d9aaedc11d948a98ca245f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 479u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b87387a8750b7f65db2fb633a09c9da89ebc7054dfdaa0a5bd47bdaa88f95f6e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 480u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("70e7033446ab823205acad7d036662199209f7849cfcfa4891c2a1421d73fb1c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 481u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fa6c8547e729b413f22522989155ad9405de5d751011062ef8c309645bf3246b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 482u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b2cc6fe80c2050bb5bed614658591899bdff3ada100aea110831bc5eb645b703")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 483u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f205ed5bdc38e93bda8ede9c526720b93fb1c90fd49a3b9df89f2140ce185c20")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 484u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("12b87da74eba9f8b27dcc51d0c0ee364f05ac9c0506ed4e5839fd38406843c21")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 485u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8a7869b50ed0af61be1c77083484e4e36f9ab678d0db8f8cbb07cb7cf8f3ca60")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 486u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("18e9c4a4edd144ef4e729e160718068b7a2e90ba807ecd51afb5ffec54f2f14f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 487u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a9b85f6b67198eef69e0995348e10397d1a8d83dafe5e2369eff039d574a609")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 488u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d24d7f7edb863620e63ec0be75e5211a612879e016989f285a9b9552b517dc0b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 489u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6428faaf8e9cdf9fad9b6cf90e73619de27d9f6c3691f85013ada075493b9e17")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 490u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5083ae3a49f9836b4113ddec737a4c38ec22664e34f1af0c4ba0136a53648a69")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 491u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8057772579ee5008cff20006c21af36ba7f512fc967112553b3ebaa44411ba49")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 492u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("487c5dc5fa1265a5f1717d8678b85afdd288c1119feb3669c0f097f8321ae25e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 493u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fed811ce54446f8affdd0b0165e272c96934074bf7b2032023588c4af1aa917c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 494u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("82dd0bfb497e3e974ebbb65cbe949320a4d23e5f50df4ea3923a986a5e751b3c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 495u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a31fe6f895267507ac8e2f23112e78d673e87f2a4fa35659428104c31fef22c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 496u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aaf6eae7657436f029c4666b9ee0d20a6296b59aeb32eb332cb360b03ef11a43")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 497u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a3d893e31ca94d6e7c025c9304e628234177bf87be1187de9c61b8340830026")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 498u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2465abc5be29b3697ac9fe121a1f1f518146e7a7a7ca9d3d9ea09108bde8773d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 499u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("be27f032403e67cee9ddc02d07929428be7a5bb406583e1e53caf806fc9c5101")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 500u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8cca1f1eb70667515655d741d85d7ed65fc70993b1bd6c697b18feb0227c8f16")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 501u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9e78eac599fabd5aeabc0e72463a3cbae430f62498f10b219c8722970cc6f533")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 502u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3eefdfbeba6f132d2c5c3d61e0d8055efd11cfb0bd39238d5620d4752f38c037")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 503u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6c0a02b7daf121c3e96328ef3e9152318933dbaebb1c4353f11e229a2bf21513")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 504u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7e8bb4f0571ee8fd21fbb651a6977fb7f123087f15a087be820a1ecdf945f447")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 505u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b44812e56436708b06e449741dd1e7e81ce02f0145839800943e8d8ca9c01032")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 506u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b8ebe2a567e6e22460a20a9af38e7140a1e98ee927bfd1cd7a0f2a0407d22676")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 507u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("30f57239f07e89f940e2f9753660d7a55e328d96999aa7c1ec66b100a158a52d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 508u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a84d3b063859665de3861307eb34672fd63de2b505628113755504c9c188621f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 509u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fce4fcf48a0d5f5d3d80c95050e3d200a6402a249b486a5e22393ebe205c0b1c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 510u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a4c256340aa4c010ca2e2ee7617528bf86e47dfa6afb736187d94a657df8f72")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 511u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d8defe4daaba5935f0f65aeefe1390abaeed73e9463ab858f0d76da6c76f4362")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 512u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6651c417aab06d74c2d474ed127909afe9dfb657c69986e44a51d7b83da6270e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 513u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d8192906c3e71dcee4b4ee095365414ea43351f599d7440ae88474c60e8de372")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 514u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5c6d8468f2596b1a752a36fd708d69e1009cab404943e621abdd21db4eef0066")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 515u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("94e8e297f357b9795eb69611b64af006c6b05441f145c0057eddde5f15e37635")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 516u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("def759b96795455b2dd1cb0b928e0a3a717689107433ca8f2a87ae9a79db1439")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 517u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("009121cf3597b8587dba171880abd1646c675e4766a0a51097da5756d13f9e52")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 518u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5098be80ab34dad7e1c00a5033932b1fa10899a63299a9bdff8de40fbc1db64d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 519u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e4d3ee8084a010fe5cce638b635bd014e41e59b8fcb9808bd330561c9146e93c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 520u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("720f5a8df6fe8c6c8a5ddbd42eba5831ceca03922fafa6c59648a220ea260d7f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 521u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8402ba879731cf680aa4776979da39f51e5a362c6b9617ab7e305aec3ba67d29")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 522u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("20fd176b019ef7cacbc9abcf5994989da5faf2f5e755514f4d3f2cbf3117520e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 523u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4493ebeb43f955d9de84250a5c9f572f4dc58df139a2626f7d6098e69f4d0372")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 524u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("90162842ed417aa10249dd344f519cafbc064c759cca3f6918b9437540fc8a3f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 525u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fe7ba35fcd7d6c6b72088a9574d1c2fc3f7f3403a0f16eda8ad5c8c5e0674467")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 526u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("841230caf4a755a91885c7c682efddb2d0fd42d68bd7f635760f0b7e862c142c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 527u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d2b95020d47e1c23cba459d7a0d38dc37a5e5d5958bb84f0c13dfc7906914848")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 528u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9adcd2dd7da31e8641c5a699239bebc00e1e71c1ba1ff1d432c14e63a26b4773")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 529u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("42c3a29e7f637ed9afeab51104cbf5c396c9e595886ac6cc05662aa2a7079d54")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 530u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("44ae852adcb1b7a9a6757a9fe07f1c109f3491b9231e95e4e271f523c24d952a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 531u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fea191e57aab536800aa842f735e09d3216843ddd27e647fc02b860d060aca6e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 532u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("980e7014e331d1686ae941a17b31d9e8ba8c5a4cd5ef1db1abe8fb5165e73e62")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 533u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b85671203e3d69918b41bb1f6482713cfa5ceb5ea168510ddbb7ef851de2b655")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 534u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bc2e401aaf5cf02a9ba1f6c64f6ebbb6f8dbf51688d1e52bf385551fbed83c58")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 535u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0c6166978a4216d2cc8628d6f25be1cdca73b80b21332009ac1fc0fb7b5ba65d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 536u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52a759bae16a29141863e42d6efc0f89a195dab4cd67b2d989922c9353398346")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 537u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("605133fe1809f3ed9cc79000f4c07ced7aa1ffe06b750aa5e8c9d0effea74b25")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 538u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a3b27d7bbd3d192d3560f5b36ca0d1f58a9ee3150fd9b725a39cc32a7619005")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 539u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f8a26ce1fc35d75ed00faa60c73699cca0b7cd419b643096415ad34c34efe306")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 540u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("74313da37adf87ec3b194d5778d9474ca77f3c3e7d21ca736935d23f95d91f7d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 541u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bc7c31ccdb521106e7bd70cae578b154a58b2d293cc49fd5cfdbd0d2ed455d47")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 542u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("92e6376fa6d69dea0dce4bd9b76b8489abd9f71c98d133a1a52ec6364b233b0a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 543u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("686d38e0f6e07ccb9eb481c10f88ddc38704dd19136f80defa6e614678b12055")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 544u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("38cd702cd0c00574ff0f01b5edc8ef819587066bbd45938285b3e901aa784526")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 545u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2242ce9958aa57b50f7d574b4b49cc327c2fc17f6ba466a67df0c6a1ad595667")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 546u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a46396d24e5d98494b773de8e4358c8364ac6729c7953a7e76bfccdcd9c65056")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 547u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3033356e201b05dbb5d87dffe483577b4b1488efcd04428a719964b74d6fdc5a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 548u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9e1dfaba70a95dcaee12eef54f50d5289c7fef38ad23f084f1aed3617822c039")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 549u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52651d40a80996867653591a57bb100c62984b23c9c6009d65a22b132264df49")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 550u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("48b84d67dd5a19f861a958aeca6c0ca38c44e833b8eb041f7eeb39fd22f1634b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 551u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fe41c9151488a52d79bd91ca35489408f8e4ebde0ede632a29746a35c5616a5b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 552u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("00fb98b5190aab654ff5d5b4fad5ba945fa1509de091545b46e85dba42211b0a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 553u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f420940e52891e092812da33a1cf9e7661ee0a95781ae4e88e6e2a8aaf127a11")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 554u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b464d8fb0736a70fb04fdf6b5931ac9c2d22e434d7746812e2c012956749a148")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 555u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("325d768c4e3d62d18bec4c01d76f2c880cc51bbf252883d8a7e4642ffdcc2d3d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 556u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ae3253a989984122d27e1ca60ff41facdc0c49c7a91fd62bae0f62d4c5694d03")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 557u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6417e00dd22415d9814b9df1fb3f9350bca7fc33bc732bf3f0132bb95dd1a95f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 558u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c8d319233ddc1d82f2cace6745907d4608c936d8a0475bde1c0e93d0ed87bc39")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 559u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("14c75dd2964afe3c0ace53fc27c7e086491002e8cdd58ca1c59dd76483cc425f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 560u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5084c09344f63be100eade724d982d7185b2aff61eadb3c10179f4e985cd1c18")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 561u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6e8dcd2630c7740b345508820cf40b7beeb7317eddf3a94ba81c29edbdc19950")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 562u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bc5b979ab8357c94cdeccf80cb0d645261e2d0611fc46e1f4fef663a8421fe05")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 563u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d091f6702e43385b6b04aae156c1109f3b8d6c58f34b59525ef3e777b4649a23")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 564u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5abcd37420892e2b593977e818eb896d956e75490a4a8435ef79e6ea90f7dc34")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 565u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ca4ade050da1055d4aebec806e37dd735bd143c4ca21405e289150d6d38a8b14")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 566u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("38cc18433f804769e0db38dfffc988960a1be18bb1370f9b2a46d8b9230c8b26")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 567u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c495d034b80b212f26ddb96ef084c7351dd53772c5af3f3680f285100e671f56")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 568u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("04a2168979d551e57ac62eac23e92259016d5b0c6981a2e378d4c2528139a771")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 569u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e6b9f603a80df3f748723d0765cdeb93cc3bb0f19f0420751076f0fcf7ab483e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 570u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5c456cc0ad4f89a91aff95133ded54442fbd3962dc00ef00170c77885b55f727")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 571u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de6177a6c9973b9dc000835490bbfd8cb562719e904cd147c9a7fee3cec5bc56")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 572u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("66002e6d232eca46a26320137695b45db8bb01127a111739c038b801c6ce7171")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 573u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b43177113ea2082e4a113e25fb02d7fc2627b8990e51b2e69da9734353cb4d35")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 574u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8e6c4db4a0fdbdfd8b3e1f1dbed2c02bb50ab1e629ca7a55d148f4bd7375fa38")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 575u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8c1ab2496d9018cc5532b04428bee8bf0749019abd207513cf31672f74e6f714")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 576u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3e2ebfcd28a4bb7798a839e2aca760b4d958d6166de028d6e51ea8f6ef2b367f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 577u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bcc8b0848ae93857a637818a515bc5c7a8313af9ceaa0a6dc79474cdf425607d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 578u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3e7b2d753b84c178eba001b2bf6ed22c067cb6f1682827b33272d2c71384b349")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 579u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4e4f2d9719c0a6c7015c872cb6691de460808178870efc9328028b8c86d5b16c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 580u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da935e5b33fa7f2f7a6e5c4fe9c91640238b038e6697c4b8aebc97803d594202")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 581u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b848b450c4e3add814ee1776942a81ff11c16a953efd01f96454102c5e3cac09")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 582u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3ade84f03bbf6698b6d1be3fefc32d7308aa0cdf0705cab799e3f7c065607c37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 583u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a547421566f2ae810a5b4bd55bbf5fc630d07b5b39bbb7c2d8f3bb722185442")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 584u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fe9b98c4c824f9212254060ffd54d7bcedd53203240a2f61e7fd252b260f275b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 585u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2602c5dd1280b9ca2032899d9d45ae57c1ec532ab4dbc0ff33539bb69c5b8159")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 586u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("54d4b58b7e660528267cc060150d717554d7c8ed2255d11cd85259d078312f77")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 587u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("98525dbbbb4887832f3dde029fb8499dbcdb20831978e50cf570b5e14e059f3a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 588u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("363cd88403913cb80cf718fe8788ca52ebb09f0303bc9b430e721d4468041e5a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 589u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec49a2ef41b66bc5fd664c41b3410d2cff2d39ca0503851d8330dcbba166576e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 590u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b6e17ce9ce7b946a4e96e00d43468ad65ed036afd7ae581ed18b8a2b4c06f11f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 591u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9698cd8383295ce146d08af5a4d6891e932981823f2c88b1a9fc7b8122802b33")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 592u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1e5e20d29cb8094a0595883443c8c3a4d4224b0279d7e26b41dd303287d07a4f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 593u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3a0983bfc36322be306e728576fb1fd74a44118e07d37c10880ac84e9c68864c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 594u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec843422af37e7d3e6a574edd3753232ba2740397dedb7e7722db5e1747fa151")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 595u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e2f273f333e7be30d09b1e525b34a7841870119f78eb7b3c37ca18d265559f17")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 596u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da4381b93b5ffdce16d8b331e8b37ba8bbb8c8ed8c19fa5fa29e58ab00f0b733")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 597u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ca02c57de4bfd9f5382824561fb15201e538c544e64838e11924840fe349ef6d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 598u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("08cbebb7ce2cd834b6d9a814cc029ad2bcd121578bb4082e62db4322ed997165")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 599u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2855bde52f59ede6f509b6f5e129ac692467e5d8ffb00cebf532ce260b0bad07")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 600u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("021a2ab66843bc101858ed6914a1545743b06e97ffda912e31c0562b9a4c5b25")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 601u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c0b819551b78b9080a0834fe45f375d19134e03b7869a362f2906d8de5465455")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 602u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("64b0a9f991b2f2aeeea8dc5415529154974e9837166a29517875d23d29be2322")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 603u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5c8300196be9c5852dec1f9f3c235eaa20ce91c5db70ba2e475b1d0ea3500569")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 604u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6c8b8f87bf830c166cb88481182ef7ad1d2a3f4aafaaefc6cde7b85f738cad1d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 605u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("60b8a5fab6d84d050c511f197227e1e0bc26afd411ea52821370781427b6f632")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 606u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c49a11e7f834d83d82d6ca1ff91c07f5c4a3f69d1089c3f81bf1f4d960283714")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 607u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("30d1b505134e74e855cca1a53148f3524456afc61062e3a765c8c8345b5bcc14")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 608u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("92d36900d55b4d01451b45b2664eab07b6159ee8ed7efd19be0f3c11eec1fe2c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 609u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("488d5f5b59acd62f45b87277f209a14ad2b62bd5adc972a94d2f6e320da6637e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 610u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4e560816dbe0c8bf9e7de683527f59dc3b8e221316dcb9bb250f45aee1d83608")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 611u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("faffc4472531bf0022f95419167535c67195b3250eecefae0e2c5308c104b061")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 612u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f45e460764851ec7f6ae2ad03113acc3f240276c7c5a9a0f4c77c69936f78735")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 613u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80d44655f584eadf846486993f14f4b69c48e729f7683688c2b8c16b18ae6253")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 614u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fcbbcc25ef6ba48fe2763a4d51ee7b5ba661877f8f5f4fd359c9b1113bbdc506")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 615u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b470af80f6c9fa558929355af4591f9dbcdbf3d15538543ec40f10eb0ea62023")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 616u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d401df02f37310b2740696483a619e2743cf109800f0fd2c80538021e39d8b25")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 617u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80fc5942cd16976ad735869996716b70cee2d6502bf418d18b698c845d5f0566")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 618u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ca2ee45ffe02b5b3960b88975bf9b10ba36f31983f805363b426f0ac0318d253")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 619u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8e9120e753069d89896289891a2937c55a97aa65703b665a706c9c1d9e2b6279")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 620u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("929c5323bd0770e3c9dac63ae2b0f868776da7793ef0393c4d8a88dbf20d6c66")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 621u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1c48e9e8fd870b1eb27715e9ab257eb233caebf4a0a9be52646d333fb22e835a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 622u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("30d9b099461015c1d2e3f30925535e0927e421b4798b974186fc718e90c42563")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 623u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4eb300462027acac2fe83711ab30b6345ef7bc56274c66bb61d170be95dd4c7d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 624u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("406eeea3e56db497629f100dfdcc78f57f7d4b04f1c227a2c0be6c25c3eb2310")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 625u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ecebc0d0fb19301638e08d2d0f5fc69a3c4704eb582289d54000f642c32ff75b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 626u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("461e04645526a1e0ef3f3d0ed56a2fb6de550389e0013532b49d74afcd198802")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 627u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ac21064b6b0837c2c8975ad7445fc40a809b29d9c6c789b627af71639f39da23")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 628u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("32cf65550dee4578dbf94ccde319831b1f0d5fdf2ecdcab805e72a116370c04c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 629u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("20eed1023a017c6f6151568fea57f461b0f392d092ad32348addf6a296e62744")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 630u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("24456d18af719c68c5d1a1b92602cd5494a18edc72fc48a7639474c62fdfd802")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 631u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec63e72fd5c28478e85970e9c37a2a0985b18861b9c991f404595693cc2cfb2e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 632u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec84d3f2948890e9a4e071d49b0f0b67cbb828a24dfb6221abe49be44a4bda33")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 633u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9206460bd933801a3eb748ece925405743c50ccbedf10c1fe6ab65a8728da443")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 634u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3ed9d524c4f25ef4fbbb44e7dc6c77177b32d37ecc99bf71149ae23ebe545b55")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 635u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fe7d9e3d4a78843e378ae8dd1a7b3d68ec3d108a31284e8dd479eca590905650")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 636u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c4a25c25a2840277ac691ae95e124c92d298d16287e1ef6b4558d3aaafa7f50f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 637u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("064247572f7dfa99047dfa0fe90213d0507c8f8522c678effd02ab786a5f554e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 638u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("741115a47cbcad1262b8529138f3b259c9017fd280146c4b2ee794b7ee0d1852")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 639u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("eafd2d2972bd5e902a84200ff618bfb1448cbc0a4dc4819594fdbb5980959e61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 640u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4487d28f47f82516f18b6fb2e390085938930abbf8027539cbb4c563592c2369")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 641u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9881defa25a8a9b0b2d8f5c82b621e6c2d6a2dcb2f1636c510cb605dcd73820c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 642u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a96b7793d7066020621a71538d94738785a32702e08ad2b8533e1d0ae599b42")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 643u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ea4e517a3ceea218dec93ba77c26b683dd735db9b8a15831e109ff7b4b6fc03")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 644u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7a70afc3f5c965dc83af7f9891a7a523a62c36dc6c2272499f10eed534daa90a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 645u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52eb5cc8a1fee2059b782f5caab97cd9d7db1e409962b026d5f615693caa4d40")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 646u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9280a154e360b631cb0012ddc4dbf87d724c8aa51ed8f7e829c11d4b65585d13")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 647u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cc089c30b242367be2d40c904ae5907cdc8b840a323b9191cefa164a13bfc14a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 648u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9ef4bc04e66570497d1cf9122ce2fe942b61390089d7c2ed0800e237aada8b54")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 649u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b4acd8260cc19f571f286c1a7b1229918b08f1f9ad590d7f8b9eba70c9a92014")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 650u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("44d2c365bbeb3f61ee926a86a644e325c1489cc8599fe4ac0ef73c1ac5011258")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 651u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("169d70e785fa27353fae7d7d343787c614023cf471c9ee08300483a168338141")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 652u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fad0182eeff08cccdebeae2896814c756ccd4d64da4260f191b53230946e0b67")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 653u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da4a0929f9449056544c3f9a70f476ed5804f66571da3ff0bebc632a19263678")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 654u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3279a8783a3d2222e3561fda804e0570ba0a711cacad98cc7d411558136eb612")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 655u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c66de500bc000358f1c1da0faa4e03b28e2da9aad0d52a1435fb6d12df7c5d4b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 656u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0015de180ab925ed6a2affec2c3a79373be4f736e8d831323174723a4e318714")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 657u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aa14e45262e4806313e8c93bb61fcbaf86c434364ded9abc52490250ba51a922")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 658u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4e7527c9d4e39657bb81de3cbba7c053ce03938c8c81be370bfe2ac20a36d33a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 659u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("524d6e812348cd97bcd9a377e106458970d251716a0fb81ab386866ac0f84f0a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 660u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("768417cc19faf02026ce9663030cd7f3d05437388d91e0e66d504e7a258e7f34")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 661u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9a33d7561a3167413524dc6d58c20289ebf2190df37f8639800d7b03a293bd71")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 662u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("985bf447b68c4b6ef2e1dba25a914786206d6abe5c0e136a5c0b67278e0cc30c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 663u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8e0f52dfcdcbee13df352166dda73319f87d6b3ccd952182c8695d0f60916e2d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 664u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9a5c7a23b69b6a832eb001b06041defdb33416c3e51f7f835255a226bc702d59")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 665u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ba1eca53d654e190d7cd46d2e8260bdb1cb9a4f3ce57cb1994a81a6f726d9025")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 666u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("18e9f6f17cc82fe36c6e284ca732a09875862e1a2a775d26bdf7857ce09d6556")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 667u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2e924a6678f216cbe608d957e35e6405cb96ff9039da17bfe5b11d5b61994772")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 668u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("24beec14d28044babe22ad97414085683394ab42f79994b2a92e4d5967f59922")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 669u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a92c3b6cab40516a62b84006088a2ed5eba1bb9e7fa1c31ee24fb6918424d09")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 670u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cadf9295788d2662b1fe88483a0f19fef4a7c442785cd16bc06396c858ec8c3a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 671u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a0da9a7ceccbe89954159ea8e514fa961501302673e7cb5b615c846afeb0dd2c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 672u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("26463377b3e0fe6b5e9662ccf81fae96597427f2222904fd8d76062bc8313c49")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 673u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0893fe40effcdce13c13902a439e3a68f65f55e9656b9189d80d2031b8113f39")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 674u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de50d5d3ad6d6678cf9215881e14e458dab752a700270442a204501dfeb83704")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 675u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1cb3d19ef7111672e49dcf06f21abfa67606f995ecc86f6e7ac61c660707aa34")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 676u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("08640cc74615964f61adbe2077296bc7880a4965e0ddf27dea734fbc81598908")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 677u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0e07543b6e5f07e92c00bf016e6ad756de24c487415c6ff7fce107cc35a86536")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 678u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("72160d51efeb22882e65c5d41a6c40071784fe12872c8f1e9d060774cb42ea55")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 679u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b4d6cf8237e20d628b25a4d553c46bc66c259ee75316c3ba685f2d040282c25d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 680u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fa16cd0ca67ce936c0208ada8a0e59136e17d20d62b9925dbe0949e3cfe1892e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 681u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("725542af4d6d00d9ae03709355b5bc38a6d4a87ac2f9e09cb6ab830e7045783f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 682u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2083d8e073862008935a5f16f0a9a939febf0461a3f038c32f66b59c5b860d7e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 683u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ce8d554bdce83efb6d5051a2580b368e981a417aaadafaa64384b59a8373c912")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 684u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("58b33308e93a7acfde5ce57b26ae3d312d5f54684e4665be0d00a9b3707d6a47")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 685u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ac226d73c24443c9481694606c2fafdd67ee48b3d6d379ca833cc489b5e1e54b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 686u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("482be8fc592d1d5c1eb510ee334ce874f95b77ef005d58a959d019df914bec61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 687u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c2905f48c5c3c3ab90fd700a7aef25901ae57ed98b3bf5a80e22c77074f41144")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 688u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("18dda1eb0986876e53bd93457c7e30564c1b9e8241eab1b4439dfdc1c0f06043")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 689u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("968045f2451c978c04f70d8f2170f34edce1f278f2de467962c1eda9b575541e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 690u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("20b7a660ecbccbcfdbd3f1f70370df7ab6dc40ace058bdd1f175a0700dacd071")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 691u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7025f3cd2558bdaaaa2c0aa3841f76f5e8ef702c5e95a4592b7ad0e09f893d61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 692u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b07b0de906dfbbec4585f22b6d1ad036434bf2771c0492dc0986dee1e7ab6b2b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 693u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7aa055722b84e83c85ad3be7b232f90807fc4c369a2d3ff205a3b082a02ce675")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 694u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("24d2094a6482c9386e521c894981ba9946152a263510472cbf508506a1564a28")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 695u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f819b6217f9c724162dffcba19f614f4cb2db68d94bdb57986c72a647e8ede0f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 696u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5a07a15acb53b35c6eb31fdd2073d8fdcca57511a6fd6fbb3a3426057dfd0212")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 697u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1c817fea2599de44aab4813786b3d1d53a6417cd2cf590746888f967234d0e15")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 698u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("dab72a0f6af9b44c98cf96c6a7c0f0ec7d8b746b1fbb64f40394d7a68bd14942")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 699u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("507cac0b6022c2843e8f115c5cba7ae568c41f86c4bd2ee16eda668e7f660226")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 700u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e052ea7be93cdb4c977f1ac8abeda149390ffd2ead7a6f307e074daf2dbbd14c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 701u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7a4aea79e86b9a5572709ea121fbb1bdad7233c4265ff7abe00169fea4b07766")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 702u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("700e281ce18e6af75cb3dc3503a443963e1a39a6801c94434150acfc6149a000")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 703u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f63084100c400656b1dccb2a03136e34ce5b50c70c2fb8ab8a871007000af171")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 704u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b4289ce46d35f376dddbaa5d710fcfefc22328486f8ca2dad2d69bde33b2561d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 705u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aaa040bfbb55acc33b9ef1a65798ed17be52e874cf7310a86a422fcdfbeb5341")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 706u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d0e55a068c37bcaf012612cc4555fe65e7d7e9ff1a9a89033c7deb8e3f38e942")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 707u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80e7f27292f494b60e3b127c02aee95b6def1967f5f43e11b2e16d591076e466")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 708u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("22736f3156ba2360abb766138709a37fb9b58755ca3fdc2f765e080c59a3fa1c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 709u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6645131287b56258d486ab8c90f68f5fc968819501d49c39b080185537cf2676")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 710u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0cdffaf6f5ae960137d09fa9fbd788b99f6b6c25e181190dc81d48d3e326793d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 711u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("901df6bf93985526dc04cb228622944c0363d5cc3fc29cd59385ffc9b255870a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 712u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a418ca91c7c5a62b71313e17f3beeff56bf100e34c5a9e498676acac711ba55b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 713u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("98837be598dfa49a7e6e18fdc12a3f216bd133faed83102bbf6d0eb4ad62c130")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 714u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bcc09677b64b070f30c7197683c69ece77aa83a27accb47956adf73d7cd72c4f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 715u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ca0cb6e1e601d3d4a9709635aa757c7db2838a898bf4f1e834bf8d536eb9672d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 716u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a26f1d1a5a4614daec4af21ad628e9d1e242ad73c1d0acb0d44b55e2adb05b21")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 717u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a0668b1b92775db1733cc655d9be4aec8f24c2391a71ec2cc827f3ad27ac6954")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 718u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("104726ef02f42f7b4bf9525a148ea6039951e786e4c1cd544959def39abd9138")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 719u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1e11980d486fc4643a9d48e8aea998446e52437a72557d5072b64d0d1a84605b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 720u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("94f6cd926b5ab1ea7ca7f880f46d7ee295098324dea8296af622109949428c46")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 721u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6a0223fa1d41c9a1e8dcf20a6984e5f86750a8124925cb56061aa1ff0d78df3c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 722u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8a5756cdd69ba957a64656595f8e032ed4a37af882e46b2644d5e95e5c6a5935")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 723u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4d0a849bd41ebb8c08de5f1b17b2ab32b8ed3649b7ad80746a64db265c6541a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 724u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("20dd9122a0e74c35c3b8cd569f062d12432f4b793ab2eaf73c3a9a74f3ff852c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 725u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a5a2f9dba1f3e9e8c09e66ae6b3b8f05d587807df782739124b340cb0eda319")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 726u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ac763823489eb76eac7cf252a40b054d4a4249913fa245ac30d6fdcf6a744214")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 727u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4faf24480fd16a60ba6f2b0d5c7509568a75ea55a36ea81e20a706d217bab2c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 728u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("849dc5a6f92998b353baee4f8829f1bc6bceaf0f2d601cd8477b3bbdd2cb9d03")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 729u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("28b8d325e6621a6d9d24e64a52ca11259ec140324c53e3463301a5e777108f49")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 730u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a795e42ef2ca73da7c652f24a80c8e0e05740bab8409b3485181b2fee0d463d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 731u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("564548432e9ffc76a34859a07354517dad2dc118417fb9ffd7150dc395049d70")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 732u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d8484444b8cb850da1901beade8eb66f2849dac7009a0b48c0b5bee559ce7f1e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 733u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e41437a0cf004ece6d25bf99ab3df0ea34512885eca6b99b13eb8229a0364327")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 734u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c4800e02263e24fea031ef971c52162639b4844bd9ea5a83358ea4ce7b3cf37c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 735u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da228ac393511d388307a799dfa6e46856bb4f8ba4c5a3b9baf8426329626a77")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 736u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3a6efa296147df3171d87a7127b975453577505cdd2a7ac7fea6aeaf11c98372")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 737u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ef6365e64735a22caaffe3036a067d6b9bd0d9c0bc73e98570c8a31a0d3550d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 738u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c06458f0057a737ea59236c49e966a5df189fdfd98a6d9db04ad747fa5f99018")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 739u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1899ae5f563993da4d048b563756d92cf32b12d1d9756dc6f3f5e02603b0910b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 740u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("686909d46396bb1cdca53571a59d5ffc97596de98c6104244dcf7f4826861914")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 741u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0418e0e233cdd6c7373a214a6c912e4b6136ab86d906422bfa0c7ea7eb750c23")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 742u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aaece69f784dda80534332dc1eaf032f4c80d06797e2fab6925ab0b99a7c2b0e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 743u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("941d2fe8711927d48fd2cf236fd127992f3ebc476b9050f96069daaefb47b640")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 744u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("54d9f858d0729e707d8fea0386c32267486d9775dfb1dce7a526688c0180d954")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 745u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("045a3d8378eb85d28f1e0adfd0fdb42c76160280647dd1b8b8ba81f012c17d37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 746u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ea0fd697e4fbbe37269b804799b0dd242872896e895c1e56e23513f7e28e420e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 747u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c6daed5a7dd48f902ab87ab8c8e47e54cc4262a7dafbd8abd1a92fbb0f42f27c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 748u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1a6af2a77a499afbd085b7e2c45fbe89fbee4a4665c47465ca689a691445d57d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 749u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a0993e7819cbd3c4923dda01e0460cc9f25868322d8cde1ad111bf703e45465f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 750u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("96044b0645c86b877907e277460bb5ffde3f04f4b822cadf245ce8ad1218827d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 751u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("52933dcc1fddf08b99ba8a27d9c98ff23a2e656d9ef73a6537e250c1ea338d5c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 752u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0c56671b25af4a11e18e8e071d880f3cde2d9a5cf602dff35a5430d2bc9e0646")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 753u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("84452b0ad11a30dc6023c903f146166bb7420e1e4e1ef1847e20f7c1f8348145")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 754u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("84f107e9da0ee6dd4f5453c6216af71e0de62f2e1cf662c2a1cb49cd5c7a6b33")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 755u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2e6c2fb71bbfe5092835c201dd2af98f685d1f1edd360f0e2f1c1889c705d710")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 756u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6a6f3de160b9713ed41bffaf0bfd64a474ad488bff9be8a8afadc46e94bef24a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 757u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("86ccd7dc5e7d07e5061dd18d572f5a9615b9e64649c8117332f4f0fcdf3ca166")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 758u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("40b0effe5bbc483c8eab02d66160f2e67e50ec6d7b55c422220374f07c2efc4c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 759u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("be66febb880b66723a3df649d0844f5653682ab69367bf12a2e638cf32b9de61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 760u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cc78927e8a9034bb249b2ba2fa67e6f1e6863a9f6fa9378d2711b7c643863d01")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 761u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f24934e9e595f5c8f55665eff204ee4961c9a604f6d6033b8430ad56eac67534")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 762u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("02a6bcc576a8be1db9d16d81e79c7b0f0f3469d8870bd8a731d26968daec243c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 763u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4055bae778472d424d73d871103e795b4204bb2c0257d2515faf53f265848c2b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 764u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("acaabe138fcc696743d062b7a7ef076b3fbf91a027550caa2f49198200e9e004")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 765u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f66e30829d46bff1dbd3213285f428bcf1c8d331b413fe92abf02dcf3b03d778")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 766u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9cb3e2095f654b8621364329e4477554bbf517cd10364521fe0220b4d0b2b703")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 767u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("449e155fb486f8b29e0d68f4feea316f9f1069a3596faacfbe4ecf899134675e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 768u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8a0dea6433be6b7ccd3090d843ee007841d9db18a413fe9469a0a670977b7551")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 769u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2024b3505efa9263811020964294c6866ed7ee5e44295f5bc0f73dbc11afa71a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 770u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d011510eb12e01462feec23dc6ad19c843749f1274f3ba4d4f0d8c4acd2dd24e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 771u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e22aad784d7f14762fefb01886f0c5ed5806c3d3307e6212cfa54575b7adef63")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 772u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da189630fb12b1f85bf34c1a9bbc4227ea7678a952eb88c4b97189780faf7d1e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 773u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("eeddf3cdc9dbb71684276772d2822e08f3d2c788f7db695ce74643fc17cf1e65")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 774u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("84cc583a493d0cb63d2abf782de0223791d71400408d905b69b255f6910f6f78")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 775u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("eed2035a3d74c39b094293ee270701bfb91b4f44dbf098ac9247718a3c013846")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 776u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4af1517cb1ec454256dbfe7719be47bf1d50f1911c68e74de072cb72a6f95253")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 777u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("04958869cefcacd840ce4219c85be212702760eb87ec028f54f589539cd97f77")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 778u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ec185078727b68f78925fa678c35154889a4b1c34801af4c42a682fbbac36774")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 779u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5cc323d0e4ddc55f005e4a5507fca6a549641867795ada96c02ed116be334b69")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 780u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6a371a6c79ac0df8b5b810201ba380133de270db3641c7189c35f99ffa982863")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 781u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("46570981de4f9b712ff7a95fe13609d174600eb3fe084a2bb7c8285fcd27482d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 782u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("58f6aaf8ebeaa5c8781561e86f777af40882d9ad5239481130dd379ba153cb44")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 783u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("145165cdf09f019b04f9f6f01dcddafcb0bfab1d30c27e47085ab0681b6c7960")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 784u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("680d4a7bcd84a4e34431f23ef0183ae17b405f0076b0390fd3c364d116be1946")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 785u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5e1cde9ef3cfc07680ea70e992cd00cb4ce4d3fd7c49a3b89e741d1cedd5941d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 786u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a959abaceac91526f76e4556bb6518d4f2f4299f2ea94450dbead3c58379462")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 787u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4ae4f725b09dde62ee4727930843a7f77d55f2783d45d15a61e220319cb4023")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 788u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c07e75615e65d624c05edb181c8521d8e470b2133695dcf602c912adaf29896e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 789u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("debd0617aa7782db74735f6837f93f1dda08124420cb77c11c110774246d786a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 790u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5a17fe6bebbf0f8218d47149cd15ab851825a3234da6fb500c2a63c66f4c6977")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 791u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e06350af04736f4c3630eee2bbcbd89592a64246717b99d2d572197c778f9c6e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 792u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f416d7514104a913ce2cb5182e75c2ed47755dda10efba7937838ace74b5c853")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 793u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9ae4a74dc26d65f1529e772ea3a38eb52de765c17a9fb7d79444004a5e3b4e0d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 794u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("648ae7fcb27733a1e45bf16dcfefcdb116b7c262064fb66a2a500d366123de3b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 795u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ac528b378690b85237fa71293b4097d39484eee37e39304d2f986ec8d4ea5871")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 796u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("26f00f0b23e9c4e3d5b56a41411aead581ae3583bbc0c9d3f3815a53dc97eb29")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 797u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("94b265369795160613ca6b03d7a4204fcedd9fa9d6d0fac92e0d3761f3596952")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 798u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("00afb866549d823bc113f21d94a419bb5ac568565e94d3fceba014ee87e1da43")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 799u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a857145830cd7c4154eb3b5fbf83f1767f38743c70838fb759dffe5407bc5d4a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 800u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b4081ae2383760e25e57edd6436f62860581caaa0543a62a3b4f450ebe6a0d5b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 801u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("98f71072fab179deec3086c8d6ffc117a56215f9fa4df74940611af3b0882c0f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 802u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cedc386ff57650b0b29d0630b2fe9135bf11c5ac52d2d3fc8471380f9a7e7b7c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 803u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4e9b057c1e1d2a8602d59201533e1ad76dc54fa0202327ad5a94e355445d7345")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 804u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e62a6d6a5f65eea53c2ce3ec97a367217ed4ce2ce9c1dbd49ff20b4a5ace7d70")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 805u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("26e6c27f0a9b495b8bfb97b86a3dd0e91a4bdb3eea218d78d76cca00e5d0bd19")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 806u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7a6ef25913e79d425f0956cce3e46148b007206d7ed18b618cc402f956492628")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 807u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a4f9278d7eb5febaf7f19dea5bd42d6480f6a2f6b316c4d3c6f180ea51c0a64")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 808u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de90cd3530f4850e56787de79eb1309fd083d93011c2648843f7e49969300917")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 809u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("161756ec91e7dbfc381c3986861b958d71e74a5b079574eb9ad344ebb9c4d962")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 810u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2632847c54175fd6d7cdccdec53c960ede7b4a7ee359873e8325cc1fc67c2a6f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 811u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6ea723a5f6252119a07d93856233bef23fd4ade638dc63b5cec5efe1b30ab063")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 812u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8c565e828c87fafae303f99f3280ca02245ffaf8469797cb589b75f380877415")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 813u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f400e48b6a862173a3b1dacfed04b4a1f430f8ae48757874d7738205c538307c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 814u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9e1b6b258bc2e4641acce786233486595b2370b8d8478de97b3f725d693f9500")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 815u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b0d968c9ef4a7efb50c3680a84a8dd313f3fe87b975b1d9540ba9eef1b881873")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 816u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("14740c8a9ab94dc5f26f9753b2559a4a17acb39bc0079136302b54304f1fd35f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 817u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1ac661c6f3f4c94c9b6aae881d616a7a024f17584f38062769a2770cbcd44978")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 818u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("527532495a9681dab0c0f4c5f6ec8df2201bcc8c6d6113a9df4f2e949853ee7d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 819u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0e3a06d6ba4eafd76b49afff2533a303174c07180ec2f6c3b1e2cc880a9b6c41")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 820u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("201cbb44d8a79e33b802d7705ff61039e297e9d567bd48fa187f71be9d512f4b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 821u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a64a747d1cd23146c7419bf1678a32ea1933911ea7cdd89cb63d54628f412249")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 822u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f608f16ca92b81b26c85e20c22a142dfc5c0d927639f44c19d3d72d4cbe86341")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 823u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4cc870152d246aef310c1fd80e19f26f78e3c017be82287241c6d4acb0c2e72d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 824u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("12521b04f099f5cc84a72fdd66e612ed33c15a8d4d0fb6fc43b184e27561db3e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 825u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("50876d9e2a7871c8a92d11239c5c06457e4302f58b5ba96577d4bf49a8ef8278")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 826u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d6a46d8866fcc51a96dd15dcb17617f1bcadd102de07aaaa24444238a2d42d3c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 827u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c68a37885b3e166e7db5e4c40bab02a6304a59f8eadba2b579ae8b2f11e1657a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 828u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c847fcc1517560e8ee7ac5ff533496ddb4110c9157af136ebe1816b2d57f8a4c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 829u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da674c2e4aa194bcf266e3315bb9d8fcb3b43cf39fe20046fc5bffbbdd41ed26")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 830u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ba5c8059bf5db872e7eac55e6272403c42c412c7aa99ac8009b2c9ce530b9974")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 831u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ecdd593e700db64a1adb94e19d2d845ff7dbbd49dd670ed1a5a5e4f4e50c6b24")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 832u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1c9285d4aa27d28bf7ed40aa3876b22d286bc29363f7f5bb253f66bb173ff573")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 833u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("06fcc3d350eb4c73e2d611e9c874b344f86e78def6856770a7a6fbb3a1fe0f4e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 834u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2833884187e0d3b2f6199aed81a05d5ff015e82936d5b735944324386d401010")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 835u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("127121f2a2ce546ae5c31fa8353980fb818cf148404735c7396d9aadd479d029")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 836u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6c51817a97155d3ddd7a3e9339d3fc1d23074187f2c35afdbbc99b7c36847759")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 837u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6e108783906ca6af326651b4590a3e14ba7525959c128028562daa3ba5bcb979")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 838u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de7216c5b6b90613b31b52ab9ae6c6a9de7558adb8b28c72c964b1892c8ee302")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 839u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f2f0085376ab189d25212fa42c8e030c27972ad8d67f9add19643f73e6ba2039")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 840u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d0e541fdea116ec8813a263e16cec29f45bbb336bff83e2c60ed8e06be996d2d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 841u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e0a9fc99e4f13571c580271752cdf0fdc92d910ce7de05a61a54c3b7b0698b6b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 842u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("aaa9dfc7fff64591e9800dc31c8a579a975bf8589a24c387601e1045018ed231")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 843u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("58a8655d87950c5469ff69e75b47143c335c1d13bda6d5205626d9e4fb8ee112")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 844u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("72bff610c37832544d0bfcc0e2f1606e235ef539c77b40234274007faf51004b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 845u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b20b8e0fa453f07ab33afad273248b3d363f4202421ae7aa1a44998a27cfcc5b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 846u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4a5bbd542d15233fd784c216b243b08633f07d54db60a8541d77086dc8e8bb78")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 847u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("42366927c368fb0034c11ec140364c90f9fc91c6c46764646f5f64d0d4974e21")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 848u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f02430f5ee4a7f20885f323ddc929968470ea52ffccfab0d73b1f2e309bf2f7f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 849u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("dcd157037312dc1cfc3ea8e6bf4972d862aea6ea562ff3251ec49eb5e7ee593c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 850u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d006aea21a3b46d0ea3522eb91967b550398510ca657e9cec94569c43c33907b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 851u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ce0936646a5917e274e5791b8f9bb337d1e20f4dd35419d565e97a5c218dcf7e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 852u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c66cad0158349befd405c656c42c29c28a6780d60cee751028cd0f4a4e86fe4f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 853u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("947696210e21cd2f00c79ce8c37bbdf4b39e2acae18ed98085ce0fef8ac08c71")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 854u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("927ce61487c8469314fb051ea19e6cc97718412ca20f5b16bb9d44c2a6bb000f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 855u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("26b8a0d68c1c56c17437cdbd28bd76d47bce31a4851ef56f049a25eab7512134")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 856u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4c8b4de286f7152f85f50ea45f9e4ea7ef2b79d1f954b1187093b57fdfdab26e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 857u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("12fc6ba296ecbfefc5e51832bdaec7e5de4f51179b518e031f0ceb574066215c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 858u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5213b781726a7834297c2adf4d63881f8b77357c56375d9bbbf120a0fc65c547")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 859u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5ebe61a07678b926ab846ded9c7733497a4738030cd7c82cea1c779e674c7366")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 860u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ae7359bb4f171a682d1de208aca04a3236212d262757576f177daae4e346b15")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 861u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("60697df0df2e24c91f04dbde5f029cf1e9e9d10202015e44d8d1f2a889bfea18")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 862u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("88357aacea0bd9cb35b72c533915ed7243589d94142798c7bc7593fe6114d152")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 863u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5048a43dbf1ca85c9107b881a5d8152c1fe5f9e1a2669d2a09282da70cf4107d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 864u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("00490d7a34a96d2958eb11f9270f84dc062c837b34becedaf7545dde07ce3930")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 865u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fae9a9a932a8438338e5bfc5d098ee803c20849ab9af22a0b26200064c11415e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 866u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("548cf7787c8aeecb4840499f54dc2fa1961b80e262a76491c536e71547f52c48")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 867u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3eb7982b7fa70e1f9c4fec6abee71487fe5988f14a8b710bd32a424e457e0136")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 868u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("da0bb888368a14a07e3cb18d2ab274bce66b9ad5bba66f9e9b5295f040a4635b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 869u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("448f0d04874f00c1b80e25ad74bd5b4d9f916fd26ea71380356acf6011283949")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 870u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8017848eb16129f1d47b1280b1d863e03099ad26619ccab85154feb2b475e40b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 871u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9669af893f05850bd6eeb5a2098e21812cc51d9dec834d9e8018367ef0849328")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 872u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("983c35e4cc19c5aa0d65069651e6223fbbfec2649bb17088ac895df9d2adf638")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 873u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8454b9441513738347e561f1b56b839e53e3a1512f9e7af4ac080c9dc8b61168")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 874u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5c7c11f23c7bd3c4e89df81d963e602cc244008c7c42c54e9db4847478bb6262")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 875u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("eef3b08d8761468bd142cd94d370f094ad13eccf8d5b894ead5c2346b2d6a041")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 876u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d64ad08dd5d8edea2cffc85d8f1e62e882d4a76eed8bd4060ecfd4eea9e25613")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 877u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a6e7fe4041c3b9860d9190bafd473e9f9f7212c9fffee533afa6132bd0e3ac60")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 878u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("68cd76a60fb76e5630f61d48a3ea059eb6a89c67ebda70c8678feab82182e626")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 879u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0401923dd36b86d199dd2c324f6c415127669a15665b18f16269370bd9c9685f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 880u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e8517fdc2caf5dc59cd6f3e4e88724a3e5c8f7725978e30d859cce87ed38265a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 881u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80d7c51a1944a0b642a8eb53c196623844195effddf672605b6d38219554977a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 882u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5c873e9001a46d9551df1a26809486cf90d8d2b3d920848d136525300533774e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 883u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("668b6eb4710a995790cc661b9f823bc917b94253979a1bf41b7ab1c5d59d6b7c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 884u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("36b51f319a75ef5d3cdbf723a8a96dbb27ebc019f3f9fa758be692c6f6a4c917")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 885u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c083a52a89598902afdeed7030b49877f1aa3708b18ee7cd714f5621ca00c441")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 886u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6c67fcf90da1c7e6ac3a83fc174cc825cc2f2a26d047741c564e1a8121ebdc13")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 887u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1461dab0614cde217562d12f62a75245c61ba0e034b7b159561d3e729d177f15")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 888u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c28058de451a42213e9550fb355853bf7bfc82e5f9189f8d13f209e9258c3d27")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 889u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e2ca1cd603d2cfbd7e683112cf3c38e1a67f0394ac17cd446e9df8191012163b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 890u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("740745f66ccdd2496a430c2169b880a637a67cd7be669f6e71db887687511822")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 891u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("be78830d6b8f5460c303bdb9f2823d66363f606f3b855a3096e4ac6190903b2a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 892u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("44fba2f08ac576e3d7336e2237babab01d64a598b4501c314f8c429f3385f834")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 893u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("98768963f566d922c345084c12c155a7bebaa99e26f3fb8723b7019ee2203309")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 894u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("208d78002d948676183858f13cd4191a6b6fce2aafaaa3cb7162aa9184b2c345")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 895u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a69864fc12e28ee91432c2567a56924cce413de435526831ea757a359571406")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 896u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ee741fec7c075f031cdfa7a0e3da6527dd68424b986ba19ed006f2cbe2312f0c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 897u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a26efc1fcbbb48e54e6395da5d242507c848375deccdbaa38328d1410e6dee1a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 898u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("386054257dba6eba613b3cae15b965f7968035302362c07665fd611d46e5ea61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 899u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4a1497c6400ef85b3cff2ff010e6ec30a2cb40bfdc81960f1411f2cce526c66")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 900u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("14ad4f55015511ec7290fedd10fd6593ddd761f8b427d58e113dd6cdc6b5e579")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 901u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c49c1c4a30ec7556e929344c68d8732bd2a9f1f26a30b09a3e93053a2a353d58")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 902u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a61e1d3b339a8e1aa7b984bf4cb222b715b1856019d4ae1eb3e8620dcbe8b257")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 903u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d2d60899b2a50be862e26fa320f342155bd353ce51c185538c25ffacbb9bbe09")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 200,
                        initial_credit_level: CreditLevel::Two,
                        rank_in_initial_credit_level: 904u32,
                        number_of_referees: 2,
                        current_credit_level: CreditLevel::Two,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("08cbb2abb538334e947b2f8a4b95d0a07ba91841d3a6f11b7f294565bdb0a063")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 905u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8c20230dc744e01eb39f5c3d57ce63f3db166c43e2a0e6fdafaf6d38dfb62104")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 906u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2e2d2221a438bffdacf64afad66b0bd28b442cba963858258970b909dbeaf947")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 907u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b05f5a5da493249ad6cc2160a4fe609d4ad3aa95686282df6790b1a5d2faab21")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 908u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6c98c4b37f9a14285b1f0dfee36dda929564eefcb7e19011bb1d5af34c4fb03b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 909u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("223d53b1cc8b63d0c5b9869a7f600d8defc1934c56d444414c609d1da35d8764")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 910u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ba03fb5e1e7f25bfbe268f2cc4c58997f3b98ce0ea059d69b4b26ea184da3566")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 911u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b01987d3ceca73bb7ca5d8f6ae21a9024f3b4d48e2963b2fdd00bf511c99aa4a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 912u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1eab37bde11b10447257641febf6e8472b1d2eaa43758e6cf3046174ac5b9749")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 913u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c699ba4eed7bf3eb7d767f89da4665a7a2beef03f3389c9e1793d705c6874f06")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 914u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0ee964666d6e80eff9620c2019da442e1439c6d2d218a9cb50ee61a8657ad727")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 915u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8899985b3c5da97d855d9372cf622f313411c8f7c52f1796a0cb36d3c209f773")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 916u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1866910c138522ec2d6d85edd9125618ed3262cabe2537a54db8bf314799fe6a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 917u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a4c2649b355078b222711b4ce4668274130d3e17d7e19f5d1818d5aa95f95c28")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 918u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("16dd1af91643ab65e4385b41d840aea9639bbc5e7706594d33351461ef99f438")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 919u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("42f8dca1b94368480550d8158603f53e717086daeb52111b8c4731847a280d7f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 920u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("42411df34b4ba5a13a3bca5c54537e008f21bd245f1bfdc4647f12b630749a14")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 921u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a6399623fa8ae3b85d681a8ccfacd031f9138e9b80e8503f269d832c3a8ffd6b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 922u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7e84696c20b956fa0bde46911a195a1545e815680d259a3f74446adce90a8b37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 923u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("64c6f1c84fc03b2ba0c0170a15472d7dbd4f8b9f081d493e3f19ae7c14486b7a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 924u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("20180258469af4a86c6a24452e556c96129fe93540078d7b0e9a41c29234d023")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 925u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("dc68f67b12bdfb7ea96696c48d08e579ec84339f7007142e690410adc59e6119")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 926u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5a6bc3456fc75d153530cf89f8c4e4cf13e4c7dfe4ab337483e3b97052bba25f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 927u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("007d7416e77bdeb69eb97d89d6839cc9c03e58e26d8e1747b1949b9972314e24")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 928u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ccb336f31b28855180c40b21fbac95c5165449d5102dd538ab7ed6e0f9a23f15")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 929u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b6b3cf52a61c5b5506fa7267b5d825c16376e53375d22a08a5e4f93cabb15131")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 930u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("14c211b1a9e8c10f69b641cd29e25482d832d4e47bcd0e81584119aa2f666d61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 931u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("bacd977ac237203039877bf77791a5f1063f95a03db3caea42680aa949490c13")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 932u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("80558e1bf202329536840745f22ec37aaa1662afdd4aa312a2bfca4ad8bb6f6a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 933u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("fc5c75b22635343fee7db6e52eeec649623cff5f69755911f8c89c7ec4070956")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 934u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3242aa5b54ad4bcf6dee84f93b2e2d415783b11632173adc525d90854eb7be59")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 935u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a441bb4bd9b5d7756aa6ffc66a53c88b84b56dbd7b9a9a9bd15195349752df37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 936u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8af3e04d93719b3fd17c1216201d6123394956c01e3a50b5bec90d075a6c5f61")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 937u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5c811907a62f0bc6d3bf3175b6e460a2c07f9aaf3b02e9d1ed903d87efc06c42")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 938u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a445f507868041ccb9f40e8169419e13aa1cd8aeeb9a64c74d2486aa3f5ad82d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 939u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("f41a4302bcd6b636c1e0791fb46961c4462520051c3344aec6ec61e5abc96c41")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 940u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("b07ce3081492bebae1ea35f050687ac9bdb5a48c92c62d004b6f266226293220")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 941u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("624a232c5a3e20532126e69d3639991b49e44cd281a2ec7570fb16cc6d06f57b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 942u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de2a5da9833a05f841929901c6937d942083b5ec4502441ebbb417a443935657")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 943u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a053bc98b0e92fba895458079fa132a057864e449055e949e90a11baedf1914b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 944u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("76aeb335a93ebff9bf641229b9f8750e45459639ac4f0157bf2ec012b53e3141")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 945u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3c6f98934cd90a32abee0c2d02ab3e2a652761956cba71d63f1334d1a6cdab0e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 946u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0c7b751904e583aba1fe613be153b364a3d050aafbbcee251483415e007ef570")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 947u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9c32f3bb5f4d54490ffc7c4bbf1d7b4c9c14c3fc71d9e37a3cd589d4bbca924f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 948u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1ed15a1d2d09eba08744a91b3efc4e2df4be69dff3dfa811b4c08a1ae37ae344")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 949u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9a919bddff2b66a6ad1773be454b7f8b76db23cdda7192202a6dc757c37e3f0c")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 950u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3ae626ec064569caaba6dd95e796bc24929268f311daf1f40a26fdd1f45b603a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 951u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1013184d65c66fcc664ea58b592c6335ba45df2c048e333cd8801062c327f15e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 952u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("148e0a490ef1dd2645c7f8498834a2dc30dfa48eb7f4c71d1a88d8fc245c4506")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 953u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("64e3a7658fff7d57f03538639a597a4cb5f54892c692069128556cc763d7f91d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 954u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("82775628e9cd6e36b6513cdab982ca7aeddc95a77a11c29ae3e11f76f9aaf807")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 955u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("de27a9dcfdab64eb7594ab38c9a105bcf6afd85237596dce9d433d7b3e4ec362")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 956u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("0a0f55af7f819dc20db15440a2585beeaddb8527cd8841b248d9c61fc99a1a3f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 957u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("4ca92a325fee500bb08427241f35f897a02ff4f0e24c6dc6ef06f41d8ca28357")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 958u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2ad1c67376b0be4a85867d20622cc35875a2251ad3b1dd8bfdc73cbc951ca357")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 959u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d2db834f1b3ca5316e2a509427dff7892e9fb1b95f181a14f353a85ac2c03575")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 960u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("18ace7cd5df50b006693526324f089a306ffadd0bc676235c8b391febb857b05")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 961u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d8de3feb42ebd81b643f10c91e3c3ad94d836bb950004722749d0242bd83c373")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 962u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3e73799cd391fad3467c86131e0d4c5c217281d13895e220912c0e32c3cbfc59")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 963u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("8add14737fde13a96e2bc9e5569ed405ceb49811bb249da98d906b02f8d7d052")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 964u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("942e1e095d3c5dd055054ddb2e9ebb4808c2ee51a25f884aa02a83ab92802a69")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 965u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("14987aed032cd886c66c6ceccb9215c4d79f6ae9c139784b6602074fd06c7565")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 966u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ced621a834f3c176bc0decb35b94ed25a338c3e47a2fabc3e36d59da2cd4c630")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 967u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3279628073aa7eb5582903bc1299eb8187066f8a39bb17c885d797e0da4c2c25")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 968u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2a2060c79552263a100922d931fffbbb4fe5ab8e26cf5fb74ce8a0a064a0d373")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 969u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("48325ca09ab97181a288836a58ae60d6ac4a19cc21a71e033b229e0d60d47a74")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 970u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("1ce74afffc24aa10621c622c63ee3b11eef0f331e959c776c9f010755d907e28")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 971u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ce1841629cb3208d27839c0f218b3f833b00f123139ad929900415c338d8ef01")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 972u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("3c20cc7051bec91166adfc8d33166c901e00dd2979c9c9ce91a248bb9c6fc35b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 973u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("c451132420b1684ff0c1838a313041ef5c7cd412c889c67cbb7965c7231e1051")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 974u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e68fffc5af759c391170c8d9e51497c522d41d75a936ac7ef1f338079f18b538")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 975u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d20df086f4a189242ae7057ba7e63e7d6efa6adc83a8dac4c59f4407d0bc5752")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 976u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cc3c2e4acda9f7ef24dcc2237cdfe339ffd866858d0a6ad19f908d3f14401147")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 977u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d0728970c08ceba7e1ac63c2baf2d803d0ad3e3a4177798e7073fc34c1fea145")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 978u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("cadb06453ac048600bb78a52818693a3d8062963f5fb87f9b5f76b107f0d505b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 979u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6271b5b6459b8bac2eb3b71452983b36d516a6a239577ae4e5db180ecc363668")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 980u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("10ae99f1c8816d4536b97ce72c2c4dbca3ca9ca849be75f26d7ee3c9c2030f6e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 981u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a8b4d610e9b56561d243a6c4c5da1dba2ca1758d0ce9006e20f47448d2398a12")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 982u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("7a14bff914c24008358f51dc9999cf7191b801e6ca4016ff49c81f55c5bffc37")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 983u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6abc64131a73ec068aa300a7d97c246e95c6ea2a839a0b67605347acd3addd00")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 984u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("a01c1f391e89e4fcf1c23b54ad0c041acf767d977ed46524a2cebaf039af0802")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 985u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("d4d47f8a294a1db14cdc2451fd2f38888f97adb83da7c12c18f5b765428e6321")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 986u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6a7e44bdaf2684318bc11bca5b1d5a3972bed7002ffcae0bb56ee630d3e0664a")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 987u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("047fd3ca98ba2f08d7861dc01de79c8b473557bac755321e57fc1a5869ab596b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 988u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("9262574d33d88ac357f4dfe443b0888420cd5f0f0f3a8fe6ca643e255f0f011d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 989u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("787e7a0393dd04ffd72b9594151300b5cf6bb7224cbeb22c823be9626f10bf44")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 990u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("22ef6082ee0a51bd24f5dbfe7b36d514de7cffc58f32a6a7a73d2cd4b3cf183f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 991u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e436d77df80444843fb58feff1a29bf469902df300abde6e0c5790d007b0887f")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 992u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("e8fe305fe48fd05618dc56c3937cf98b5a3d17361184ca32af3e7eab4c091f6b")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 993u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6cd94ac970a63acab0f322de3216817af4fe8dc57fd2561bcd5d2c3c73a30073")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 994u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("803623e46704f2b1aca1613500eb2aa45a45cca31d76baeaa9801a97758fe203")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 995u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("ce5963eec700b342ab21f73b39d713f17886bf4924f00bc9dd65b463d8d61711")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 996u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("2200cc9204034e7ffd15aee6375a230a9ab37f7ca749d882423df54ca3694314")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 997u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("808611cb0123e7b727d6d0b66d577304bd6594cffde58c6440c64a50f5135301")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 998u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("6acd089fc663f206bdb787ccec677846693d21bd60df4364f1af5c0980a4134e")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 999u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("566a2871641a9fa7be3c28ffcbc064bc97c1d813cfcfcf1e9f702c92e44edd0d")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 100,
                        initial_credit_level: CreditLevel::One,
                        rank_in_initial_credit_level: 1000u32,
                        number_of_referees: 1,
                        current_credit_level: CreditLevel::One,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("327a801d6ff9dac479976e9462581f48663bf353b9f0c7e1429f16500c19e902")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 300,
                        initial_credit_level: CreditLevel::Three,
                        rank_in_initial_credit_level: 1001u32,
                        number_of_referees: 3,
                        current_credit_level: CreditLevel::Three,
                        reward_eras: 270,
                    },
                ),
                (
                    hex!("5619381eed74163066c1b094cd7c05df0cd993dda6e6bb050712ea4af97ed867")
                        .into(),
                    CreditData {
                        campaign_id: 0,
                        credit: 400,
                        initial_credit_level: CreditLevel::Four,
                        rank_in_initial_credit_level: 1002u32,
                        number_of_referees: 7,
                        current_credit_level: CreditLevel::Four,
                        reward_eras: 270,
                    },
                ),
            ],
        }),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        true,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        None,
        Some(chain_spec_properties()),
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            other_authority_keys()[0].clone(),
            other_authority_keys()[1].clone(),
            other_authority_keys()[2].clone(),
        ],
        testnet_root_key(),
        Some(vec![
            other_authority_keys()[0].1.clone(),
            other_authority_keys()[1].1.clone(),
            other_authority_keys()[2].1.clone(),
            testnet_root_key(),
            other_authority_keys()[0].0.clone(),
            other_authority_keys()[1].0.clone(),
            other_authority_keys()[2].0.clone(),
        ]),
        false,
    )
}

/// customize tokenDecimals
pub fn chain_spec_properties() -> json::map::Map<String, json::Value> {
    let mut properties: json::map::Map<String, json::Value> = json::map::Map::new();
    properties.insert(
        String::from("ss58Format"),
        json::Value::Number(json::Number::from(42)),
    );
    properties.insert(
        String::from("tokenDecimals"),
        json::Value::Number(json::Number::from(18)),
    );
    properties.insert(
        String::from("tokenSymbol"),
        json::Value::String(String::from("DPR")),
    );
    properties
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        None,
        Some(chain_spec_properties()),
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::service::{new_full_base, new_light_base, NewFullBase};
    use sc_service_test;
    use sp_runtime::BuildStorage;

    fn local_testnet_genesis_instant_single() -> GenesisConfig {
        testnet_genesis(
            vec![authority_keys_from_seed("Alice")],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            None,
            false,
        )
    }

    /// Local testnet config (single validator - Alice)
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis_instant_single,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    /// Local testnet config (multivalidator Alice + Bob)
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        sc_service_test::connectivity(
            integration_test_config_with_two_authorities(),
            |config| {
                let NewFullBase {
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                    ..
                } = new_full_base(config, |_, _| ())?;
                Ok(sc_service_test::TestNetComponents::new(
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                ))
            },
            |config| {
                let (keep_alive, _, _, client, network, transaction_pool) = new_light_base(config)?;
                Ok(sc_service_test::TestNetComponents::new(
                    keep_alive,
                    client,
                    network,
                    transaction_pool,
                ))
            },
        );
    }

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    #[test]
    fn test_staging_test_net_chain_spec() {
        staging_testnet_config().build_storage().unwrap();
    }
}
