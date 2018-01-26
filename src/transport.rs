use std::{str, io};
use bytes::{BufMut, BytesMut};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;


pub struct LineCodec;

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> io::Result<()> {
        dst.extend(item.as_bytes());
        dst.extend(b"\n");
        Ok(())
    }
}


impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&c| c == b'\n') {
            let line = buf.split_to(i);
            // chop \n
            buf.split_to(1);

            match str::from_utf8(&line) {
                Ok(line_str) => Ok(Some(line_str.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-9")),
            }
        } else {
            Ok(None)
        }
    }
}


pub struct LineProto;
impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = String;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = String;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}
