use super::*;

use ::amq_protocol::protocol::connection::AMQPMethod;
use ::amq_protocol::protocol::AMQPClass;
// use ::amq_protocol::protocol::connection::Open;
use ::amq_protocol::protocol::connection::OpenOk;
use ::mq::vhost::VHost;

/*
    On the wire:
        * Receive Method:Connection/Open
        * Send Method:Connection/OpenOk

    Description:
        The clien chooses the VHost.

        The server agrees.

    TODO:
        Authorisation: access to vhost based on the identity
*/

pub async fn run<S>(
    framing: &mut AmqpFraming<S>,
    backend: &dyn Backend,
) -> Result<(String, Arc<dyn VHost>), HandshakeError>
where
    S: IoStream,
{
    let frame = util::receive_frame(framing).await?;

    match frame {
        AMQPFrame::Method(channel_id, AMQPClass::Connection(AMQPMethod::Open(open))) => {
            let () = expect_control_channel(
                channel_id,
                open.get_amqp_class_id(),
                open.get_amqp_method_id(),
            )?;

            let vhost_name = open.virtual_host.as_str();
            let vhost_api = backend
                .vhost_select(vhost_name)
                .await
                .map_err(|source| HandshakeError::ISE {
                    props: AmqpFrameProps {
                        channel_id,
                        class_id: open.get_amqp_class_id(),
                        method_id: open.get_amqp_method_id(),
                    },
                    source,
                })?
                .ok_or_else(|| HandshakeError::NoSuchVHost(vhost_name.to_owned()))?;

            let vhost_name = vhost_name.to_owned();

            let open_ok = OpenOk {};
            let method = AMQPMethod::OpenOk(open_ok);
            let class = AMQPClass::Connection(method);
            let frame = AMQPFrame::Method(channel_id, class);

            let () = framing
                .send(frame)
                .await
                .map_err(Into::into)
                .map_err(HandshakeError::SendError)?;

            Ok((vhost_name, vhost_api))
        },

        unexpected =>
            return Err(HandshakeError::UnexpectedFrame {
                expected: "Method.Connection/Open",
                props: From::from(&unexpected),
            }),
    }
}
