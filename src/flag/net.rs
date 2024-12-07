use crate::{ArgumentSource, Error, Flag, FlagInfo, InvalidAddressError, Result};

macro_rules! impl_net {
    ($($t: ty),*) => {$(
        impl Flag for $t {
            fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
                let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

                value
                    .as_str()?
                    .parse()
                    .map_err(|_| Error::invalid_flag_value(info, long, InvalidAddressError))
            }
        }
    )*};
}

impl_net!(
    std::net::IpAddr,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::net::SocketAddr,
    std::net::SocketAddrV4,
    std::net::SocketAddrV6
);
