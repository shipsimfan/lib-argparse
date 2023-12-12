use argparse::{FlagArgument, Parser};
use macros::{help_flag, parsing_flag, simple_flag, version_flag};
use std::{num::NonZeroU8, path::PathBuf, str::FromStr};

pub struct Arguments {
    data_path: PathBuf,
    worker_threads: u8,
    max_receive_size: u16,
    max_transmit_size: u16,
}

struct ThreadCount(NonZeroU8);

struct PacketSize(u16);

const MINIMUM_MAX_PACKET_SIZE: u16 = 576;

#[test]
fn integration() {
    let flags: &[&dyn FlagArgument<Arguments>] = &[
        &simple_flag!(, "data" 1*"PATH" "missing PATH after --data" [
            "Sets the path where data will be stored",
            "Defaults to \"scopes\""
        ] |options: Arguments, mut values| {
            options.data_path = values.swap_remove(0).into();
        }),
        &parsing_flag!(, "workers" "COUNT" "missing COUNT after --workers" [
            "Sets the number of worker threads",
            "Defaults to a system provided value, usually the number of cores - 2"
        ] |options: Arguments, count: ThreadCount| {
            options.worker_threads = count.0.get();
        }),
        &parsing_flag!(, "max-transmit-size" "SIZE" "missing SIZE after --max-transmit-size" [
            "Sets the maximum size of transmitted packets",
            concat!("Defaults to ", stringify!(DEFAULT_MAX_PACKET_SIZE)),
        ]  |options: Arguments, size: PacketSize| {
            options.max_transmit_size = size.0;
        }),
        &parsing_flag!(, "max-receive-size" "SIZE" "missing SIZE after --max-receive-size" [
            "Sets the maximum size of received packets",
            concat!("Defaults to ", stringify!(DEFAULT_MAX_PACKET_SIZE)),
        ] |options: Arguments, size: PacketSize| {
            options.max_receive_size = size.0;
        }),
        &help_flag!("h", "help"),
        &version_flag!(, "version" concat!("Hostess v", env!("CARGO_PKG_VERSION"))),
    ];

    let parser = Parser::new()
        .name(&"Test")
        .description(&"A test program")
        .flags(flags);

    assert!(parser
        .parse(
            Arguments {
                data_path: PathBuf::new(),
                worker_threads: 0,
                max_receive_size: 0,
                max_transmit_size: 0,
            },
            [format!("--workers"), format!("2")].into_iter(),
            None,
        )
        .is_ok());
}

impl FromStr for ThreadCount {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ThreadCount(s.parse().map_err(|_| {
            "invalid worker count, must be an integer between 1 and 65535"
        })?))
    }
}

impl FromStr for PacketSize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR_MESSAGE: &str = "invalid packet size, must be an integer between 576 and 65535";

        let count = s.parse().map_err(|_| ERROR_MESSAGE)?;

        if count < MINIMUM_MAX_PACKET_SIZE {
            return Err(ERROR_MESSAGE);
        }

        Ok(PacketSize(count))
    }
}
