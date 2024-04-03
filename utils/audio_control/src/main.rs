use std::process::Command;
use std::{env, u32};

enum Filter {
    Input,
    Output,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let device = &args[1].to_string();
    let action = &args[2].to_string();
    let value = &args[3].to_string();

    let source_ids_input = get_ids("--list-sources", Filter::Input);
    //let source_ids_output = get_ids("--list-sources", Filter::Output);
    let sink_ids = get_ids("--list-sinks", Filter::Output);
    let default_sink_id = get_ids("--get-default-sink", Filter::Output);

    match device.as_str() {
        "speaker" => {
            let v = value.clone().parse::<u8>();
            if v.is_ok() {
                output_action(default_sink_id, sink_ids, action.to_string(), v.unwrap());
            }
        },
        "microphone" => {
            let v = value.clone().parse::<u8>();
            if v.is_ok() {
                input_action(default_sink_id, source_ids_input, action.to_string(), v.unwrap());
            }
        },
        _ => { }
    }
}

fn get_ids(arg: &str, filter: Filter) -> Vec<u32> {
    let data = String::from_utf8(Command::new("pamixer").arg(arg).output().unwrap().stdout).unwrap();

    let c_data = data.clone();
    let lines: Vec<&str> = c_data.split("\n").collect();

    let mut ids: Vec<u32> = Vec::new();

    let filter_str: &str = match filter {
        Filter::Input => "_input",
        Filter::Output => "_output",
    };

    for i in lines {
        let mut j = i.split_whitespace();
        let id_val = j.next();
        let id_type = j.next();

        if id_val.is_some() {
            match id_val.unwrap().parse::<u32>() {
                Ok(id) => {
                    if id_type.unwrap().contains(filter_str) {
                        ids.push(id);
                    }
                },
                Err(_) => { continue;},
            }
        }
    }

    ids
}

fn output_action(default_sink_id: Vec<u32>, ids: Vec<u32>, action: String, value: u8) {
    if default_sink_id.len() == 1 {
        let default_id = default_sink_id[0];
        for i in ids {
            if i != default_id {
                mute(i, "sink");
            }
        }

        if action == "toggle_mute" {
            toggle_mute(default_id, "sink");
        }

        if action == "vol_up" {
            vol_up(default_id, "sink", value);
        }

        if action == "vol_down" {
            vol_down(default_id, "sink", value);
        }
    }
}

fn input_action(default_sink_id: Vec<u32>, ids: Vec<u32>, action: String, value: u8) {
    if default_sink_id.len() == 1 {
        let sink_id = default_sink_id[0];
        let mut mic_id: u32 = 0;
        if ids.contains(&(sink_id+1)) {
            for i in ids {
                if i != (sink_id+1) {
                    mute(i, "source");
                } else {
                    mic_id = i;
                }
            }
        }

        if action == "toggle_mute" {
            toggle_mute(mic_id, "source");
        }

        if action == "vol_up" {
            vol_up(mic_id, "source", value);
        }

        if action == "vol_down" {
            vol_down(mic_id, "source", value);
        }

        if action == "get_vol" {
            get_vol(mic_id, "source");
        }

        if action == "is_mute" {
            get_is_mute(mic_id, "source");
        }
    }
}

fn toggle_mute(id: u32, dev: &str) -> bool {
    Command::new("pamixer").arg(format!("--{}", dev)).arg(id.to_string()).arg("-t").status().is_ok()
}

fn mute(id: u32, dev: &str) -> bool {
    Command::new("pamixer").arg(format!("--{}", dev)).arg(id.to_string()).arg("-m").status().is_ok()
}

fn vol_up(id: u32, dev: &str, vol: u8) -> bool {
    Command::new("pamixer").arg(format!("--{}", dev)).arg(id.to_string()).arg("-i").arg(vol.to_string()).arg("--allow-boost").status().is_ok()
}

fn vol_down(id: u32, dev: &str, vol: u8) -> bool {
    Command::new("pamixer").arg(format!("--{}", dev)).arg(id.to_string()).arg("-d").arg(vol.to_string()).arg("--allow-boost").status().is_ok()
}

fn get_vol(id: u32, dev: &str) -> bool {
    let vol = String::from_utf8(Command::new("pamixer").arg(format!("--{}", dev)).arg(id.to_string()).arg("--get-volume").output().unwrap().stdout).unwrap();
    print!("{}", vol);
    true
}

fn get_is_mute(id: u32, dev: &str) -> bool {
    let vol = String::from_utf8(Command::new("pamixer").arg(format!("--{}", dev)).arg(id.to_string()).arg("--get-mute").output().unwrap().stdout).unwrap();
    print!("{}", vol);
    true
}
