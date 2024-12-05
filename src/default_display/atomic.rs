use crate::DefaultDisplay;
use std::sync::atomic::Ordering;

impl DefaultDisplay for std::sync::atomic::AtomicBool {
    type Display<'a> = bool;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicI8 {
    type Display<'a> = i8;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicI16 {
    type Display<'a> = i16;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicI32 {
    type Display<'a> = i32;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicI64 {
    type Display<'a> = i64;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicIsize {
    type Display<'a> = isize;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicU8 {
    type Display<'a> = u8;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicU16 {
    type Display<'a> = u16;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicU32 {
    type Display<'a> = u32;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicU64 {
    type Display<'a> = u64;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}

impl DefaultDisplay for std::sync::atomic::AtomicUsize {
    type Display<'a> = usize;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.load(Ordering::Relaxed)
    }
}
