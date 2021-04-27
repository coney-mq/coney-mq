use super::*;

/*

The control channel's main responsibility is to close the connection when it's asked to.

Thus it only expects the following PDUs:
- Connection/Close
- Connection/Close-Ok

*/

#[derive(Debug)]
pub struct ControlChannel {}
