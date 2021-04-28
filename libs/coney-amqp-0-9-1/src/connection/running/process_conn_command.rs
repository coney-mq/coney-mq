use super::*;

pub(super) async fn process_conn_command<S>(
    _framing: &mut AmqpFraming<S>,
    _context: &mut ConnContext,
    _command: ConnCommand,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    unimplemented!()
}
