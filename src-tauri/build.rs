use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // 设置本地化
    setup_localization();

    // 设置加密出口合规声明
    setup_encryption_compliance();

    // 设置 Tauri 构建配置
    tauri_build::build();
}

fn setup_localization() {
    // 设置 Info.plist 的本地化配置
    // Tauri 会读取这些环境变量
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=12.0");

    // 确保本地化资源存在
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let resources_dir = PathBuf::from(&manifest_dir).join("resources");

    // 创建本地化目录（如果不存在）
    let zh_cn_dir = resources_dir.join("zh_CN.lproj");
    let zh_hans_dir = resources_dir.join("zh-Hans.lproj");
    let en_dir = resources_dir.join("en.lproj");

    if !zh_cn_dir.exists() {
        fs::create_dir_all(&zh_cn_dir).unwrap();
    }
    if !zh_hans_dir.exists() {
        fs::create_dir_all(&zh_hans_dir).unwrap();
    }
    if !en_dir.exists() {
        fs::create_dir_all(&en_dir).unwrap();
    }

    // 创建 InfoPlist.strings 文件（如果不存在）
    let zh_content = r#"/* Localized versions of Info.plist keys - Chinese Simplified */

CFBundleName = "CaoGit";
CFBundleDisplayName = "CaoGit";
CFBundleGetInfoString = "CaoGit Git 管理器";
NSHumanReadableCopyright = "Copyright © 2024-2025 CaoGit. All rights reserved.";
"#;

    let zh_strings = zh_cn_dir.join("InfoPlist.strings");
    if !zh_strings.exists() {
        fs::write(&zh_strings, zh_content).unwrap();
    }

    // zh-Hans 使用相同的中文内容
    let zh_hans_strings = zh_hans_dir.join("InfoPlist.strings");
    if !zh_hans_strings.exists() {
        fs::write(&zh_hans_strings, zh_content).unwrap();
    }

    let en_strings = en_dir.join("InfoPlist.strings");
    if !en_strings.exists() {
        let content = r#"/* Localized versions of Info.plist keys - English */

CFBundleName = "CaoGit";
CFBundleDisplayName = "CaoGit";
CFBundleGetInfoString = "CaoGit Git Manager";
NSHumanReadableCopyright = "Copyright © 2024-2025 CaoGit. All rights reserved.";
"#;
        fs::write(&en_strings, content).unwrap();
    }

    println!("cargo:rerun-if-changed=resources/zh_CN.lproj");
    println!("cargo:rerun-if-changed=resources/zh-Hans.lproj");
    println!("cargo:rerun-if-changed=resources/en.lproj");
}

fn setup_encryption_compliance() {
    // 设置加密出口合规声明
    // CaoGit 仅使用标准的 HTTPS 加密和 macOS Keychain，属于豁免范围
    // 参考: https://developer.apple.com/documentation/security/complying_with_encryption_export_regulations

    // 注意: Tauri 2.0 通过 tauri.conf.json 中的 bundle.macOS.info_plist 来配置
    // 这里添加环境变量作为备用方案
    println!("cargo:rustc-env=TAURI_MACOS_USES_NON_EXEMPT_ENCRYPTION=false");
}
