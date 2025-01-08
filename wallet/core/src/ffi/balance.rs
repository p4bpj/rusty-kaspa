use crate::imports::*;
use crate::result::Result;
use crate::utxo::balance as native;
use kaspa_consensus_core::config::constants::consensus;
use kaspa_consensus_core::network;

///
/// Represents a {@link UtxoContext} (account) balance.
///
/// @see {@link IBalance}, {@link UtxoContext}
///
/// @category Wallet SDK
///
#[derive(uniffi::Object)]
pub struct Balance {
    inner: native::Balance,
}

#[uniffi::export]
impl Balance {
    /// Confirmed amount of funds available for spending.
    pub fn mature(&self) -> u64 {
        self.inner.mature.into()
    }

    /// Amount of funds that are being received and are not yet confirmed.
    pub fn pending(&self) -> u64 {
        self.inner.pending.into()
    }

    /// Amount of funds that are being send and are not yet accepted by the network.
    pub fn outgoing(&self) -> u64 {
        self.inner.outgoing.into()
    }

    pub fn to_balance_strings(&self, network_type: kaspa_consensus_core::network::NetworkType) -> Result<BalanceStrings> {
        Ok(native::BalanceStrings::from((Some(&self.inner), &network_type, None)).into())
    }
}

impl From<native::Balance> for Balance {
    fn from(inner: native::Balance) -> Self {
        Self { inner }
    }
}

///
/// Formatted string representation of the {@link Balance}.
///
/// The value is formatted as `123,456.789`.
///
/// @category Wallet SDK
///
#[derive(uniffi::Object)]
pub struct BalanceStrings {
    inner: native::BalanceStrings,
}

#[uniffi::export]
impl BalanceStrings {
    pub fn mature(&self) -> String {
        self.inner.mature.clone()
    }

    pub fn pending(&self) -> Option<String> {
        self.inner.pending.clone()
    }
}


impl From<native::BalanceStrings> for BalanceStrings {

    fn from(inner: native::BalanceStrings) -> Self {
        Self { inner }
    }
}


// pub struct NetworkType {
//     inner: kaspa_consensus_core::network::NetworkType
// }


