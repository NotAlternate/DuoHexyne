use phf::phf_map;

#[allow(non_upper_case_globals)]
pub const commands: phf::Map<&'static str, &'static str> = phf_map! {
    "help" => "\x1b[1mDuo\x1b[35m-\x1b[0m\x1b[1mH≡xyne v0.1.0\x1b[0m\nA \x1b[4mcompiled\x1b[0m programming language.\n\nProgram parameters:\n    \x1b[1mDuoHexyne\x1b[0m <INPUT> [FLAGS]\n\nProgram Flags:\n    \x1b[1m-h\x1b[0m & \x1b[1m--help\x1b[0m  ::  Shows DuoHexyne's description and usage.",
};

#[allow(non_upper_case_globals)]
pub const errors: phf::Map<&'static str, &'static str> = phf_map! {
    "noParam" => "\x1b[31;1m≡x\x1b[0m No Arguments passed.",

    // File related
    "noInputFilename" => "\x1b[31;1m≡x\x1b[0m No Input filename given to compile.",
    "noOutputFilename" => "\x1b[31;1m≡x\x1b[0m No Output filename given for flag `-o | --output`",
    "fileReadErr" => "\x1b[31;1m≡x\x1b[0m Unable to read input file.",
};