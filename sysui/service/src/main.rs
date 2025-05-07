mod angle;
mod utilasi;

use angle::*;
use std::{env, process::exit};
use utilasi::usage;

fn main() {
    /*
    Angle sebagai penghubung renderer di Android
    Contoh kasus:
    - Game seperti PUBG dan Genshin hanya mendukung OpenGL ES.
    - Device mendukung Vulkan, tapi game tidak bisa langsung pakai.

    Maka: 
    [Game] <--OpenGLES--> [ANGLE] <--Vulkan--> [Device]
    Dengan ANGLE, game yang tidak support Vulkan bisa tetap memakai Vulkan lewat bridging.
    Cek log: logcat -d | grep ANGLE
    */
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        exit(1);
    }

    match args[1].as_str() {
        "-d" => {
            println!("\n- Applying Vulkan backend via ANGLE...\n");
            angle_vulkan();
        }
        "-L" => {
            println!("\n- Applying OpenGLES backend via ANGLE...\n");
            angle_opengles();
        }
        "-S" => {
            println!("\n- Launching ANGLE application...\n");
            start_angle();
        }
        "-R" => {
            println!("\n- Resetting all ANGLE modifications...\n");
            clear_angle();
        }
        "-h" | "--help" => usage(),
        other => {
            println!("Unknown option: {}", other);
            println!("Valid options are: -d, -L, -S, -R, -h");
            exit(1);
        }
    }

    println!("\n⚠️ This module is protected by copyright and is\n\
              intended for use by regular users only. Any use of\n\
              this module, including its code, design, or features,\n\
              by other developers without written permission from\n\
              the copyright owner is strictly prohibited.\n\
              ________________________________________________(+)\n");
}