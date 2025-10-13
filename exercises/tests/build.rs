//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
  // === For tests7: 设置环境变量 TEST_FOO 为当前时间戳 ===
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 使用 rustc-env 将 TEST_FOO 注入到编译期环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // === For tests8: 启用条件编译 feature "pass" ===
    // 使用 rustc-cfg 添加 cfg 标记，让 #[cfg(feature = "pass")] 生效
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
