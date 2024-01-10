use itertools::Itertools;

pub fn split_lines_by_byte(bytes: &[u8], splitter: u8) -> Vec<&[u8]> {
    bytes.split(|byte| *byte == splitter).collect()
}

pub fn split_lines_by_bytes<'a>(bytes: &'a[u8], splitter: &[u8]) -> Vec<&'a[u8]> {
    let splitter_positions: Vec<usize> = bytes.windows(splitter.len())
        .positions(|window| window == splitter)
        .collect();

    let mut start = 0;
    let mut split_slices = Vec::new();

    for pos in splitter_positions {
        let byteslice_split = &bytes[start..pos];
        start = pos+2;
        split_slices.push(byteslice_split)
    };
    split_slices
}

#[macro_export]
macro_rules! create_enum_and_matchers {
    ($enum_name:ident, $($enum_options:ident),*) => {
        #[allow(dead_code)]
        #[derive(Debug, Default)]
        pub enum $enum_name {
            #[default]
            $($enum_options,)*
        }

        impl $enum_name {
            #[allow(dead_code)]
            pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
                match bytes {
                    $(
                        bytes if bytes == stringify!($enum_options).as_bytes() => Some($enum_name::$enum_options),
                    )*
                    _ => None
                }
            }
        }

        impl ToString for $enum_name {
            #[allow(dead_code)]
            fn to_string(&self) -> String {
                match self {
                    $(
                      $enum_name::$enum_options => stringify!($enum_options).to_string(),
                    )*
                }
            }
        }
    };

    ($enum_name:ident, $($enum_options:ident, $enum_values:expr, $enum_str:expr),*) => {
        #[allow(dead_code)]
        #[derive(Debug, Default)]
        pub enum $enum_name {
            #[default]
            $($enum_options,)*
        }

        impl $enum_name {
            #[allow(dead_code)]
            pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
                match bytes {
                    $(
                        bytes if bytes == stringify!($enum_options).as_bytes() => Some($enum_name::$enum_options),
                    )*
                    _ => None
                }
            }

            #[allow(dead_code)]
            pub fn from_int(value: u32) -> Option<Self> {
                match value {
                    $(
                        $enum_values => Some($enum_name::$enum_options),
                    )*
                    _ => None
                }
            }

            #[allow(dead_code)]
            pub fn to_int(&self) -> u32 {
                match self {
                    $(
                        $enum_name::$enum_options => $enum_values,
                    )*
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}",
                    match self {
                        $(
                            $enum_name::$enum_options => $enum_str.to_string(),
                        )*
                    }
                )
            }
        }
    };
}