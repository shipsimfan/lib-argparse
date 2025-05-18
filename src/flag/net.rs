use crate::{ArgumentSource, Error, Flag, FlagInfo, InvalidAddressError, Result};

macro_rules! impl_net {
    ($($t: ty),*) => {$(
        impl Flag for $t {
            fn parse(
                this: &mut Option<Self>,
                source: &mut dyn ArgumentSource,
                info: &FlagInfo<Self>,
                long: bool
            ) -> Result<()> {
                if this.is_some() {
                    return Err(Error::repeated_flag(info, long));
                }

                let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

                *this = Some(value.as_str()?.parse().map_err(|_| {
                    Error::invalid_flag_value(info, long, InvalidAddressError)
                })?);
                Ok(())
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
