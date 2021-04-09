use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use zentachain_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, ContractsConfig,
	ValidatorSetConfig, ImOnlineConfig, SessionConfig, SessionKeys, StakingConfig, BabeConfig,
	MembershipConfig, ElectionsPhragmenConfig, CollectiveConfig 
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::{ChainType, Properties};
use zentachain_runtime::constants::currency::CHAINS;
use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "CHAIN";

/// Specialized `ZentachainChainSpec`. This is a specialization of the general zentachain ChainSpec type.
pub type ZentachainChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
type AccountPublic = <Signature as Verify>::Signer;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ZentachainChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ZentachainChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(DEFAULT_PROTOCOL_ID),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ZentachainChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ZentachainChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
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
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(DEFAULT_PROTOCOL_ID),
		// Extensions
		None,
	))
}


// Zajin PoA
fn session_keys(
	aura: AuraId,
	grandpa: GrandpaId,
) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

pub fn zajin_staging_config() -> Result<ZentachainChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Zajin wasm binary not available".to_string())?;

    Ok(ZentachainChainSpec::from_genesis(
        // Name
        "zentachain",
        // ID
        "zajin_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    (	// 5FHWaxnsx6b4iBVZpGyS7TCWNuewnU23RShqf13vRuDkzYKb
                        hex!["8e78f3225727510e1b240fc0832b8cd46ce858db168c5e454900608b25ee7957"].unchecked_into(), // AuraId SR25519 //1//aura
			// 5HqKu9Xt9ez3NT4Re5M2QK1G7BwjyFANW7sVRsS3Pix23hGb
                        hex!["ff354bb6467687851c311924e7a1881ee96e45856e6190d84a7c0589c5650ca6"].unchecked_into(), // GrandpaId ED25519 //1//grandpa
                    ),
                    (	// 5Exb5Ngv2Ai2jsqeeiKiKjg8jKtV5Wt3oNiouUc4sZM4aYKS
                        hex!["800a66ba8d5727c58363a6cdff70958ec266492a6b786da1377a158779849a74"].unchecked_into(), // AuraId SR25519 //2//aura
			// 5EgzCJrov65KSM2FrXBjuwq284jq8ukgYQGEP2EDd5X29d6h
                        hex!["74244d5a760dae039ceb2d626ce0e1e4ecc4d9e42f759a8d7168c0af514fb73c"].unchecked_into(), // GrandpaId ED25519 //2//grandpa
                    ),
                ],
		 	// 5EJMmxL26AuD5w6AkDfr2zRgfaLCQWVYX5QUFdoJ7Vp2pZhk
                hex!["62e24d8f32504fd9c050b25667bc541c02809f2b4b66fcbc7daeb3025e93924e"].into(), // Sudo Account
                vec![hex!["62e24d8f32504fd9c050b25667bc541c02809f2b4b66fcbc7daeb3025e93924e"].into(), // Funded Account
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some(DEFAULT_PROTOCOL_ID),
        // Properties
        Some(zentachain_properties()),
        // Extensions
        None,
    ))
}

pub fn zajin_testnet_config() -> Result<ZentachainChainSpec, String> {
    ZentachainChainSpec::from_json_bytes(&include_bytes!("../../service/zajin-spec.json")[..])
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool,
) -> GenesisConfig {
	//Additional genesis variables

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		collective: Some(CollectiveConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		elections_phragmen: Some(ElectionsPhragmenConfig {
			members: vec![],
		}),
		membership: Some(MembershipConfig {
			phantom: Default::default(),
			members: Default::default(),
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		staking: Some(StakingConfig {
			canceled_payout: Default::default(),
			force_era: Default::default(),
			history_depth: Default::default(),
			invulnerables: vec![],
			minimum_validator_count: initial_authorities.len() as u32,
			slash_reward_fraction: Perbill::from_percent(10),
			validator_count: initial_authorities.len() as u32 * 2,
			stakers: vec![],
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					session_keys(
						x.0.clone(),
						x.1.clone()
					)
				)
			}).collect::<Vec<_>>(),
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		validator_set: Some(ValidatorSetConfig {
			validators: vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
		}),
		pallet_contracts: Some(ContractsConfig {
                current_schedule: pallet_contracts::Schedule {
                    enable_println,
                    ..Default::default()
            },
        }),
    }
}
