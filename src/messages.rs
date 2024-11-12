#![allow(missing_docs)]

//i18n::include_fluent!("fluent");

pub mod supported_languages {
    pub static EN: &'static ::i18n::locale::LanguageTag<'static> = &::i18n::locale::LanguageTag {
        language: unsafe { ::i18n::locale::Language::new_unchecked("en".as_bytes()) },
        script: None,
        region: None,
        variants: ::i18n::locale::Variant::from_slice(&[]),
    };

    pub static FR: &'static ::i18n::locale::LanguageTag<'static> = &::i18n::locale::LanguageTag {
        language: unsafe { ::i18n::locale::Language::new_unchecked("fr".as_bytes()) },
        script: None,
        region: None,
        variants: ::i18n::locale::Variant::from_slice(&[]),
    };
}

pub mod errors {
    use super::supported_languages::*;

    #[doc = "Arguments for [`INVALID_UTF8`]"]
    #[allow(non_camel_case_types)]
    pub struct INVALID_UTF8<'a> {
        pub string: &'a str,
    }

    impl<'a> INVALID_UTF8<'a> {
        fn en<'b>(args: &INVALID_UTF8<'b>, f: &mut ::core::fmt::Formatter) -> core::fmt::Result {
            f.write_fmt(core::format_args!("invalid UTF-8 \"{}\"", args.string))
        }

        fn fr<'b>(args: &INVALID_UTF8<'b>, f: &mut ::core::fmt::Formatter) -> core::fmt::Result {
            f.write_fmt(core::format_args!("UTF-8 invalide \"{}\"", args.string))
        }
    }

    impl<'a> i18n::translation::MessageKey for INVALID_UTF8<'a> {
        type Arguments<'b> = INVALID_UTF8<'b>;

        fn default_message<'b>() -> i18n::translation::Message<Self::Arguments<'b>> {
            Self::en
        }

        fn try_get_message<'b>(
            language: &i18n::locale::LanguageTag,
        ) -> Option<i18n::translation::Message<Self::Arguments<'b>>> {
            if language == EN {
                return Some(Self::en);
            }

            if language == FR {
                return Some(Self::fr);
            }

            None
        }
    }
}
