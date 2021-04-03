use super::*;

use ::amq_protocol::frame::ProtocolVersion;

/*
    On the wire:
        * Receive AMQP-Protocol-Header from the client.

    Description:
        Ensure that the client runs a compatible protocol version.
*/

/// Perform the phase of AMQP-Protocol-Header
/// On success returns the `ProtocolVersion` chosen by the client.
pub async fn run<S>(framing: &mut AmqpFraming<S>) -> Result<ProtocolVersion, ConnectionError>
where
    S: IoStream,
{
    let frame = util::receive_frame(framing).await?;

    match frame {
        AMQPFrame::ProtocolHeader(protocol_version) => check_protocol_version(protocol_version),
        unexpected => Err(ConnectionError::unexpected_frame(
            "ProtocolHeader",
            &format!("{}", unexpected),
        )),
    }
}

fn check_protocol_version(pv: ProtocolVersion) -> Result<ProtocolVersion, ConnectionError> {
    match (pv.major, pv.minor, pv.revision) {
        (0, 9, rev) if rev >= 1 => Ok(pv),

        _unsupported => Err(ConnectionError::UnsupportedProtocolVersion(pv)),
    }
}
