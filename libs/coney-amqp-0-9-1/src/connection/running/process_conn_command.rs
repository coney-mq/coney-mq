use super::*;

pub(super) async fn process_conn_command<S>(
    framing: &mut AmqpFraming<S>,
    context: &mut ConnContext,
    command: ConnCommand,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    unimplemented!()
}
