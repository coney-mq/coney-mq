#[derive(Debug, PartialEq, Eq)]
pub(super) enum ChanState {
    Closed,
    Open,
    Closing,

    Invalid,
}
