use bitcoin::Amount;

/// Assert that the call returns the specified error message.
#[macro_export]
macro_rules! assert_error_message {
    ($call:expr, $code:expr, $msg:expr) => {
        match $call.unwrap_err() {
            Error::JsonRpc(JsonRpcError::Rpc(ref e))
                if e.code == $code && e.message.contains($msg) => {}
            e => panic!(
                "expected '{}' error for {}, got: {}",
                $msg,
                stringify!($call),
                e
            ),
        }
    };
}

/// Quickly create a BTC amount.
pub fn btc<F: Into<f64>>(btc: F) -> Amount {
    Amount::from_btc(btc.into()).unwrap()
}
