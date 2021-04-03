use super::*;

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

pub async fn run<S>(framing: &mut AmqpFraming<S>) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    
    unimplemented!()
}
