use package::{PackageReader, PackageWriter};
use socket::PeerSocket;

pub struct Session {
    reader: PackageReader,
    writer: PackageWriter,

    soecket: PeerSocket
}