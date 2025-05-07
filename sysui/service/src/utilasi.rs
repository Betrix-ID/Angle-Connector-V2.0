pub fn usage() {
    let script_version = "2.0 [ Angle Custom Settings ]"; 
    println!(r#"♨️ Angle Connector {} - Automatic Device Optimization Utility

Usage:
    Angle Connector [OPTION]

Options:
    -d        Apply ANGLE with Vulkan rendering (for supported devices).
    -L        Apply ANGLE with OpenGL ES rendering (for compatibility).
    -R        Reset all ANGLE settings and modifications.
    -S        Start/Launch the ANGLE application.
    -h, --help  Show this help message and exit.

Description:
    Angle Connector is a lightweight Rust utility designed to enhance graphic 
    compatibility and rendering performance on Android devices. It automatically 
    clears outdated graphic configurations, enables the ANGLE graphics driver 
    (either Vulkan or OpenGL ES backend), and synchronizes system properties 
    to apply changes effectively. By stopping background processes, clearing 
    app cache, and adjusting the rendering pipeline, Angle Connector allows 
    non-Vulkan-supported apps to run using Vulkan through ANGLE, improving both 
    visual quality and system efficiency.         

Examples:
    Apply Vulkan backend:
        SpeedsterX -d
    Apply OpenGL ES backend:
        SpeedsterX -L
    Launch ANGLE application:
        SpeedsterX -S
    Reset all modifications:
        SpeedsterX -R

Requirements:
    - Root access may be required.
    - Device must support 'adb shell' and required shell utilities.

More Info:
    ANGLE Overview:              https://chromium.googlesource.com/angle/angle
    Android Performance Docs:    https://developer.android.com/topic/performance
    Support Group:               https://t.me/speedsterx_support
    Community Discussions:       https://forum.xda-developers.com"#, script_version);
}