// It is forbidden to change without official permission from the Module maker. 

use std::{process::Command, path::Path, fs};

fn run_cmd(command: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn shell(message: &str) {
    let cmd = format!(
        "cmd notification post -S bigtext -t '♨️ Angle Connector ' 'Tag' '{}' > /dev/null 2>&1",
        message
    );
    let _ = run_cmd(&cmd);
}

pub fn clear_angle() {
    println!(
        "Description: 
        This function is responsible for resetting all ANGLE-related system settings.
        It scans the global settings database for keys related to ANGLE and graphics drivers,
        then deletes them to ensure a clean slate before applying new configurations. 
        This step is crucial to avoid conflicts between old settings and the new rendering backend."
    );

    let commands = [
        "for key in $(cmd settings list global | grep angle | cut -f1 -d=); do cmd settings delete global \"$key\"; done",
        "for key in $(cmd settings list global | grep driver | cut -f1 -d=); do cmd settings delete global \"$key\"; done",
        "setprop debug.graphics.angle.developeroption.enable false",
    ];
    for cmd in commands {
        run_cmd(cmd);
    }
    shell("Success: Reset angle settings to default...");
    
    let cmd = format!("cmd activity start -a android.intent.action.VIEW -d https://www.instagram.com/pai_calll?igsh=OGZnYmZ5OGdiMG9r > /dev/null 2>&1");
    let _ = run_cmd(&cmd);
}

pub fn angle_vulkan() {
    println!(
        "Description: 
        This function configures the Android device to use ANGLE with the Vulkan backend.
        It begins by clearing any existing ANGLE or graphics driver settings, then sets
        new global values to enable ANGLE and Vulkan rendering through 'skiavk'.
        Additional steps include granting necessary permissions to the ANGLE package, 
        and optimizing the system by stopping user apps and clearing cache for better performance.
        Recommended for devices with stable Vulkan support seeking improved graphical performance."
    );

    let commands = [ 
        "cmd settings put global angle_gl_driver_all_angle 1",
        "cmd settings put global angle_debug_package org.chromium.angle",
        "setprop debug.angle.libs.suffix angle_in_apk",
        "pm grant com.android.angle android.permission.WRITE_SECURE_SETTINGS",
        "setprop debug.graphics.angle.developeroption.enable true",
        "setprop debug.hwui.renderer skiavk",
        "setprop debug.hwui.use_vulkan 1",
        "sync",
    ];
    for cmd in commands {
        run_cmd(cmd);
    }

    println!("\n[Clear Cache]\n  Clearing application caches...");

    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("pm list package -3 | cut -f2 -d:")
        .output()
    {
        let list = String::from_utf8_lossy(&output.stdout);

        for package in list.lines() {
            if ["me.piebridge.brevent"]
                .contains(&package)
            {
                continue;
            }

            let cache_path = format!("rm -rf /sdcard/Android/data/{}/cache", package);
            if Path::new(&cache_path).exists() {
                let _ = fs::remove_dir_all(&cache_path);
            }

            let cmds = [                
                format!("cmd activity profile stop --user 0 {}", package),
                format!("cmd activity make-uid-idle --user 0 {}", package),
                format!("cmd activity force-stop --user 0 {}", package),
            ];
            for cmd in cmds.iter() {
                run_cmd(cmd);
            }
        }
        shell("Successfully Applied custom Angle Vulkan");
        
        let cmd = format!("cmd activity start -a android.intent.action.VIEW -d https://www.instagram.com/pai_calll?igsh=OGZnYmZ5OGdiMG9r > /dev/null 2>&1");
        let _ = run_cmd(&cmd);
    } 
} 

pub fn angle_opengles() {
    println!(
        "Description:
        This function sets up the Android device to utilize ANGLE with the OpenGL ES backend.
        It is ideal for devices that have compatibility issues or instability with Vulkan.
        The routine begins by purging previous driver settings, then applies OpenGL-specific props,
        such as forcing the use of GLES and setting ANGLE driver properties accordingly. 
        As with the Vulkan setup, it also grants permissions and clears app cache to maximize efficiency.
        Useful for improving rendering on mid-range or older Android devices."
    );

    let commands = [ 
        "cmd settings put global angle_gl_driver_all_angle 1",
        "cmd settings put global angle_debug_package org.chromium.angle",
        "setprop debug.angle.libs.suffix angle_in_apk",
        "pm grant com.android.angle android.permission.WRITE_SECURE_SETTINGS",
        "setprop debug.graphics.angle.developeroption.enable true",
        "setprop debug.hwui.renderer skiagl",
        "setprop debug.hwui.force_opengl 1",
        "sync",
    ];
    for cmd in commands {
        run_cmd(cmd);
    }

    println!("\n[Clear Cache]\n  Clearing application caches...");

    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("pm list package -3 | cut -f2 -d:")
        .output()
    {
        let list = String::from_utf8_lossy(&output.stdout);

        for package in list.lines() {
            if ["me.piebridge.brevent"]
                .contains(&package)
            {
                continue;
            }

            let cache_path = format!("rm -rf /sdcard/Android/data/{}/cache", package);
            if Path::new(&cache_path).exists() {
                let _ = fs::remove_dir_all(&cache_path);
            }

            let cmds = [                
                format!("cmd activity profile stop --user 0 {}", package),
                format!("cmd activity make-uid-idle --user 0 {}", package),
                format!("cmd activity force-stop --user 0 {}", package),
            ];
            for cmd in cmds.iter() {
                run_cmd(cmd);
            }
        }
        shell("Successfully Applied custom Angle OpenGLES");
        
        let cmd = format!("cmd activity start -a android.intent.action.VIEW -d https://www.instagram.com/pai_calll?igsh=OGZnYmZ5OGdiMG9r > /dev/null 2>&1");
        let _ = run_cmd(&cmd);
    }
}

pub fn start_angle() {
    println!(
        "Description:
        This function attempts to launch the main activity of the ANGLE application manually.
        It also grants WRITE_SECURE_SETTINGS permission to the ANGLE package to ensure full
        system access is available for the renderer configurations.
        This is particularly useful for testing whether ANGLE is installed and operational,
        or for verifying changes after switching graphics backends."
    );
    
    let commands = [ 
        "cmd activity start -n com.android.angle/com.android.angle.MainActivity",
        "pm grant com.android.angle android.permission.WRITE_SECURE_SETTINGS",
    ];
    for cmd in commands {
        run_cmd(cmd);
    }
    shell("Successfully launched Angle application"); 
}