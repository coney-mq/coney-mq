use super::*;

use ::amq_protocol::protocol::connection::AMQPMethod;
use ::amq_protocol::protocol::connection::Secure;
use ::amq_protocol::protocol::connection::Start;
use ::amq_protocol::protocol::connection::StartOk;
use ::amq_protocol::protocol::AMQPClass;

const MAX_CHALLENGES_COUNT: usize = 10;

/*
    On the wire:
        * Send Method:Connection/Start;
        * Receive Method:Connection/StartOk.

    Description:
        The server states major and minor versions of the protocol.

        The server exposes its properties:
            - software type: platform, product, version;
            - server capabilities;
            - instance id (cluster name);
            - etc.

        The server states avaialble authentication mechanisms.

        The server states supported locale.

        The client exposes its properties:
            - capabilities;
            - software type: platform, product, version.

        The client chooses authentication mechanism and provides authentication data.

        The client chooses the locale.
*/

/// Perform the phase of Connection/Start
/// On success returns the `Identity` yielded by the authentication procedure.
pub async fn run<S>(
    framing: &mut AmqpFraming<S>,
    authc: &dyn Authc,
) -> Result<String, ConnectionError>
where
    S: IoStream,
{
    let start = Start {
        version_major: 0,
        version_minor: 9,
        server_properties: Default::default(),
        mechanisms: "PLAIN".into(),
        locales: "en_US".into(),
    };
    let method = AMQPMethod::Start(start);
    let class = AMQPClass::Connection(method);
    let frame = AMQPFrame::Method(util::CTL_CHANNEL_ID, class);
    let () = framing
        .send(frame)
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?;

    let frame = util::receive_frame(framing).await?;

    match frame {
        AMQPFrame::Method(channel_id, AMQPClass::Connection(AMQPMethod::StartOk(start_ok))) => {
            process_start_ok(framing, authc, channel_id, start_ok).await
        }
        unexpected => Err(ConnectionError::unexpected_frame(
            "Method.Connection/Start-Ok",
            &format!("{}", unexpected),
        )
        .into()),
    }
}

async fn process_start_ok<S>(
    framing: &mut AmqpFraming<S>,
    authc: &dyn Authc,
    channel_id: u16,
    start_ok: StartOk,
) -> Result<String, ConnectionError>
where
    S: IoStream,
{
    use ::authc::AuthcFailure;
    use ::authc::ProcedureReply;

    let () = util::expect_control_channel(channel_id)?;
    let mech_name = start_ok.mechanism.as_str();
    let mut procedure = authc
        .select_mech(mech_name)
        .ok_or(AuthcFailure::unsupported_mechanism(
            start_ok.mechanism.as_str(),
        ))
        .map_err(ConnectionError::Authc)?;

    let mut response = start_ok.response.as_str().to_owned();

    for _ in 0..MAX_CHALLENGES_COUNT {
        match procedure
            .response(&response)
            .await
            .map_err(ConnectionError::Authc)?
        {
            ProcedureReply::Failure => {
                return Err(ConnectionError::Authc(AuthcFailure::invalid_creds()).into())
            }
            ProcedureReply::Success(identity) => {
                return Ok(identity);
            }
            ProcedureReply::Challenge(challenge) => {
                let secure = Secure {
                    challenge: challenge.into(),
                };
                let method = AMQPMethod::Secure(secure);
                let class = AMQPClass::Connection(method);
                let frame = AMQPFrame::Method(util::CTL_CHANNEL_ID, class);

                let () = framing
                    .send(frame)
                    .await
                    .map_err(Into::into)
                    .map_err(ConnectionError::IO)?;

                let frame = util::receive_frame(framing).await?;

                match frame {
                    AMQPFrame::Method(
                        channel_id,
                        AMQPClass::Connection(AMQPMethod::SecureOk(secure_ok)),
                    ) => {
                        let () = util::expect_control_channel(channel_id)?;
                        response = secure_ok.response.as_str().to_owned();
                        continue;
                    }
                    unexpected => {
                        return Err(ConnectionError::unexpected_frame(
                            "Method.Connection/Secure-Ok",
                            &format!("{}", unexpected),
                        )
                        .into())
                    }
                }
            }
        }
    }

    Err(ConnectionError::AuthcTooManyChallenges)
}
