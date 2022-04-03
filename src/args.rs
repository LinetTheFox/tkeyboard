use crate::settings::Settings;

use clargs::{parse, ParsingConfig };

pub fn parse_args(settings: &mut Settings) {
    let mut cli_parse_config = ParsingConfig::new();
    cli_parse_config.add_param("w".to_string(), false);
    cli_parse_config.add_flag("v".to_string());
    let args = parse(std::env::args(), &mut cli_parse_config).unwrap();

    if args.has_flag("v") {
        println!("tkeyboard v0.1.0");
        std::process::exit(0);
    }
    if args.get_param("w").is_some() {
        let num_str = args.get_param("w").unwrap();
        let num = num_str.parse::<usize>().unwrap_or_else(|_| {
            panic!("Value for -w key is not a valid number!")
        });
        println!("Setting the word count to {}", num);
        settings.word_count = num;
    }
}
