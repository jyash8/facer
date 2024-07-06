use clap::{value_parser, Arg, Command};

const PAYLOAD_SIZE: usize = 16;
const CHARACTER_DEVICE: &str = "/dev/acer-gkbbl-0";
const PAYLOAD_SIZE_STATIC_MODE: usize = 4;
const CHARACTER_DEVICE_STATIC: &str = "/dev/acer-gkbbl-static-0";

fn main() {
    let app = Command::new("facer")
        .arg(
            Arg::new("mode")
                .short('m')
                .default_value("3")
                .value_parser(value_parser!(u8))
                .help(
                    r#"Effect modes:
    0 -> Static [Accepts ZoneID[1,2,3,4] + RGB Color]
    1 -> Breath [Accepts RGB color]
    2 -> Neon
    3 -> Wave
    4 -> Shifting [Accepts RGB color]
    5 -> Zoom [Accepts RGB color]"#,
                ),
        )
        .arg(
            Arg::new("zone")
                .short('z')
                .default_value("1")
                .value_parser(value_parser!(i32))
                .help("specifies the zone. Possible values: 1,2,3,4}"),
        )
        .arg(
            Arg::new("brightness")
                .short('b')
                .default_value("100")
                .value_parser(value_parser!(u8))
                .help("Speed of effects"),
        )
        .arg(
            Arg::new("speed")
                .short('s')
                .default_value("4")
                .value_parser(value_parser!(u8))
                .help("Speed of effects"),
        )
        .arg(
            Arg::new("direction")
                .short('d')
                .default_value("1")
                .value_parser(value_parser!(u8))
                .help("specifies the direction"),
        )
        .arg(
            Arg::new("color")
                .short('c')
                .num_args(3)
                .default_values(["255", "255", "255"])
                .value_parser(value_parser!(u8))
                .help("specifies the speed"),
        );

    let matches = app.get_matches();
    let get_val = |id: &str| -> u8 { *(matches.get_one::<u8>(id).unwrap()) };
    let mode = get_val("mode");
    let color = matches.get_many("color").unwrap().collect::<Vec<&u8>>();

    if mode == 0 {
        // Static coloring mode
        let mut payload = [0u8; PAYLOAD_SIZE_STATIC_MODE];

        let zone = *(matches.get_one::<i32>("zone").unwrap());
        if zone < 1 || zone > 4 {
            println!("Invalid zone entered. Possible values are 1,2,3,4 from left to right");
            std::process::exit(0);
        }

        payload[0] = 1 << (zone - 1);
        payload[1] = *color[0];
        payload[2] = *color[1];
        payload[3] = *color[2];
        std::fs::write(CHARACTER_DEVICE_STATIC, payload).expect("Failed to write payload");

        let mut payload = [0u8; PAYLOAD_SIZE];
        payload[2] = *(matches.get_one::<u8>("brightness").unwrap());
        payload[9] = 1;
        std::fs::write(CHARACTER_DEVICE, payload).expect("Failed to write payload.");
    } else {
        //Dynamic colouring mode
        let mut payload = [0u8; PAYLOAD_SIZE];

        payload[0] = mode;
        payload[1] = get_val("speed");
        payload[2] = get_val("brightness");
        payload[3] = if mode == 3 { 8 } else { 0 };
        payload[4] = get_val("direction");
        payload[5] = *color[0];
        payload[6] = *color[1];
        payload[7] = *color[2];
        payload[9] = 1;

        std::fs::write(CHARACTER_DEVICE, payload).expect("Failed to write payload.");
    }
}
