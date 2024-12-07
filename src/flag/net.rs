use crate::{messages::errors::*, ArgumentSource, Error, Flag, FlagInfo, Result};
use i18n::translation::m;

#[derive(Debug)]
pub struct InvalidAddress;

impl std::error::Error for InvalidAddress {}

impl std::fmt::Display for InvalidAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        m!(AddressInvalid).fmt(f)
    }
}

impl Flag for std::net::IpAddr {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|_| Error::invalid_flag_value(info, long, InvalidAddress))
    }
}

impl Flag for std::net::Ipv4Addr {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|_| Error::invalid_flag_value(info, long, InvalidAddress))
    }
}

impl Flag for std::net::Ipv6Addr {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|_| Error::invalid_flag_value(info, long, InvalidAddress))
    }
}

impl Flag for std::net::SocketAddr {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|_| Error::invalid_flag_value(info, long, InvalidAddress))
    }
}

impl Flag for std::net::SocketAddrV4 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|_| Error::invalid_flag_value(info, long, InvalidAddress))
    }
}

impl Flag for std::net::SocketAddrV6 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|_| Error::invalid_flag_value(info, long, InvalidAddress))
    }
}
