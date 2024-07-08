extern crate prost_build;

fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&[
        "proto/actuators.proto",
        "proto/altimeter.proto",
        "proto/battery_state.proto",
        "proto/bezier_curve.proto",
        "proto/bezier_trajectory.proto",
        "proto/bit32_flag.proto",
        "proto/can.proto",
        "proto/frame.proto",
        "proto/header.proto",
        "proto/imu.proto",
        "proto/input.proto",
        "proto/led_array.proto",
        "proto/led.proto",
        "proto/magnetic_field.proto",
        "proto/nav_sat_fix.proto",
        "proto/nav_sat_status.proto",
        "proto/odometry.proto",
        "proto/point.proto",
        "proto/pose.proto",
        "proto/pose_with_covariance.proto",
        "proto/pwm.proto",
        "proto/quaternion.proto",
        "proto/safety.proto",
        "proto/sim_clock.proto",
        "proto/snap_setpoint.proto",
        "proto/status.proto",
        "proto/time.proto",
        "proto/twist.proto",
        "proto/twist_with_covariance.proto",
        "proto/vector3.proto",
        "proto/wheel_odometry.proto",
    ], &["proto"])?;
    Ok(())
}
