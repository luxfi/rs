use primitive_types::U256;

pub const KIB: u64 = 1024; // 1 kibibyte
pub const MIB: u64 = 1024 * KIB; // 1 mebibyte
pub const GIB: u64 = 1024 * MIB; // 1 gibibyte

pub const NANO_LUX: u64 = 1;
pub const MICRO_LUX: u64 = 1000 * NANO_LUX;
pub const MILLI_LUX: u64 = 1000 * MICRO_LUX;

/// On the X-Chain, one LUX is 10^9 units.
/// On the P-Chain, one LUX is 10^9 units.
/// ref. <https://snowtrace.io/unitconverter>
pub const LUX: u64 = 1000 * MILLI_LUX;

pub const KILO_LUX: u64 = 1000 * LUX;
pub const MEGA_LUX: u64 = 1000 * KILO_LUX;

/// On the C-Chain, one LUX is 10^18 units.
/// ref. <https://snowtrace.io/unitconverter>
pub const LUX_EVM_CHAIN: u64 = 1000 * MEGA_LUX;

/// Converts the nano LUX to LUX unit for X and P chain.
pub fn cast_xp_nlux_to_lux(nlux: U256) -> u64 {
    // ref. "ethers::utils::Units::Ether"
    let lux_unit = U256::from(10).checked_pow(U256::from(9)).unwrap();
    let luxs = nlux.checked_div(lux_unit).unwrap();
    if luxs >= U256::from(u64::MAX) {
        u64::MAX
    } else {
        let converted = luxs.as_u64();
        if converted >= u64::MAX as u64 {
            u64::MAX
        } else {
            converted as u64
        }
    }
}

/// RUST_LOG=debug cargo test --package lux-types --lib -- units::test_cast_xp_nlux_to_lux --exact --show-output
#[test]
fn test_cast_xp_nlux_to_lux() {
    assert_eq!(cast_xp_nlux_to_lux(U256::max_value()), u64::MAX);
    assert_eq!(cast_xp_nlux_to_lux(U256::from(u64::MAX)), 18446744073);
    assert_eq!(cast_xp_nlux_to_lux(U256::from(100)), 0);
}

/// Converts the X/P chain LUX unit to nano-LUX.
/// On the X and P-Chain, one LUX is 10^9 units.
/// ref. <https://snowtrace.io/unitconverter>
/// If it overflows, it resets to U256::MAX.
pub fn cast_lux_to_xp_nlux(lux: U256) -> U256 {
    // ref. "ethers::utils::Units::Ether"
    let lux_unit = U256::from(10).checked_pow(U256::from(9)).unwrap();
    if let Some(nluxs) = lux.checked_mul(lux_unit) {
        nluxs
    } else {
        U256::max_value()
    }
}

/// RUST_LOG=debug cargo test --package lux-types --lib -- units::test_cast_lux_to_xp_nlux --exact --show-output
#[test]
fn test_cast_lux_to_xp_nlux() {
    assert_eq!(cast_lux_to_xp_nlux(U256::max_value()), U256::max_value());
    assert_eq!(
        cast_lux_to_xp_nlux(U256::from(1)),
        U256::from_dec_str("1000000000").unwrap()
    );
    assert_eq!(
        cast_lux_to_xp_nlux(U256::from(10)),
        U256::from_dec_str("10000000000").unwrap()
    );
    assert_eq!(
        cast_lux_to_xp_nlux(U256::from(500)),
        U256::from_dec_str("500000000000").unwrap()
    );
}

/// Converts the nano LUX to LUX/i64 unit for C-chain and other EVM-based subnets.
///
/// On the C-Chain, one LUX is 10^18 units.
/// ref. <https://snowtrace.io/unitconverter>
///
/// If it overflows, it resets to i64::MAX.
pub fn cast_evm_nlux_to_lux_i64(nlux: U256) -> i64 {
    // ref. "ethers::utils::Units::Ether"
    let lux_unit = U256::from(10).checked_pow(U256::from(18)).unwrap();
    let luxs = nlux.checked_div(lux_unit).unwrap();
    if luxs >= U256::from(u64::MAX) {
        i64::MAX
    } else {
        let converted = luxs.as_u64();
        if converted >= i64::MAX as u64 {
            i64::MAX
        } else {
            converted as i64
        }
    }
}

/// RUST_LOG=debug cargo test --package lux-types --lib -- units::test_cast_evm_nlux_to_lux_i64 --exact --show-output
#[test]
fn test_cast_evm_nlux_to_lux_i64() {
    assert_eq!(cast_evm_nlux_to_lux_i64(U256::max_value()), i64::MAX);
    assert_eq!(cast_evm_nlux_to_lux_i64(U256::from(i64::MAX)), 9);
    assert_eq!(cast_evm_nlux_to_lux_i64(U256::from(100)), 0);
}

/// Converts the EVM LUX unit to nano-LUX.
/// On the C-Chain, one LUX is 10^18 units.
/// ref. <https://snowtrace.io/unitconverter>
/// If it overflows, it resets to U256::MAX.
pub fn cast_lux_to_evm_nlux(lux: U256) -> U256 {
    // ref. "ethers::utils::Units::Ether"
    let lux_unit = U256::from(10).checked_pow(U256::from(18)).unwrap();
    if let Some(nluxs) = lux.checked_mul(lux_unit) {
        nluxs
    } else {
        U256::max_value()
    }
}

/// RUST_LOG=debug cargo test --package lux-types --lib -- units::test_cast_lux_to_evm_nlux --exact --show-output
#[test]
fn test_cast_lux_to_evm_nlux() {
    assert_eq!(cast_lux_to_evm_nlux(U256::max_value()), U256::max_value());
    assert_eq!(
        cast_lux_to_evm_nlux(U256::from(1)),
        U256::from_dec_str("1000000000000000000").unwrap()
    );
    assert_eq!(
        cast_lux_to_evm_nlux(U256::from(10)),
        U256::from_dec_str("10000000000000000000").unwrap()
    );
    assert_eq!(
        cast_lux_to_evm_nlux(U256::from(500)),
        U256::from_dec_str("500000000000000000000").unwrap()
    );
}
