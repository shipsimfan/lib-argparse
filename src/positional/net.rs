use crate::{Argument, Error, InvalidAddressError, Positional, PositionalInfo, PositionalResult};

macro_rules! impl_net {
    ($($t: ty),*) => {$(
        impl Positional for $t {
            fn parse(
                this: &mut Option<Self>,
                argument: Argument,
                info: &PositionalInfo<Self>,
            ) -> PositionalResult {
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
