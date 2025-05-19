use crate::{Argument, Error, InvalidAddressError, Positional, PositionalInfo, PositionalResult};

macro_rules! impl_net {
    ($($t: ty),*) => {$(
        impl Positional for $t {
            fn parse<'a>(
                this: &mut Option<Self>,
                argument: Argument<'a>,
                info: &PositionalInfo<Self>,
            ) -> PositionalResult<'a> {
                match argument.as_str()?.parse() {
                    Ok(value) => *this = Some(value),
                    Err(_) => {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidAddressError,
                        ))
                    }
                }
                PositionalResult::Next
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
