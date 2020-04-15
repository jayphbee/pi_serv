use apm::allocator::ENABLE_ALLOC;
use apm::common::{NetIPType, NetProtocolType, SysStat};
use atom::Atom;
use hex::FromHex;
use js_httpc;
use pi_crypto::digest::{digest, DigestAlgorithm};
use pi_crypto::signature::ECDSASecp256k1;
use regex::Regex;
use serde_json::Value;
use std::process::Command;
use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Arc;
use std::vec::Vec;
use time::now_millisecond;
use timer::{FuncRuner, TIMER};

#[derive(Debug)]
pub struct License {
    os: String,             // 系统
    license: String,        // 授权码
    overtime: AtomicU64,    // 到期时间
    client_id: String,      // 客户端唯一标识
    heartbeat: AtomicUsize, // 心跳间隔毫秒
    heart_count: AtomicU8,  // 尝试次数/已尝试次数
    heart_error: AtomicU8,  // 尝试次数/已尝试次数
}

impl License {
    #[cfg(any(windows))]
    pub fn new(license: String) -> Arc<Self> {
        let uuid = get_uuid();
        Arc::new(License {
            os: String::from("Windows"),
            license: license,
            overtime: AtomicU64::new(0),
            client_id: uuid,
            heartbeat: AtomicUsize::new(1 * 60 * 60 * 1000),
            heart_count: AtomicU8::new(3),
            heart_error: AtomicU8::new(0),
        })
    }
    #[cfg(any(unix))]
    pub fn new(license: String) -> Arc<Self> {
        let uuid = get_uuid();
        Arc::new(License {
            os: String::from("Linux"),
            license: license,
            overtime: AtomicU64::new(0),
            client_id: uuid,
            heartbeat: AtomicUsize::new(1 * 60 * 60 * 1000),
            heart_count: AtomicU8::new(3),
            heart_error: AtomicU8::new(0),
        })
    }
    // 定时处理
    pub fn set_timer(license: &mut Arc<License>, timeout: usize) {
        let mut license = Arc::clone(license);
        let runner = FuncRuner::new(Box::new(move || {
            License::verify(&mut license);
            let timeout = license.heartbeat.load(Ordering::Relaxed);
            License::set_timer(&mut license, timeout);
        }));
        TIMER.set_timeout(runner, timeout as u32);
    }

    // 验证授权
    pub fn verify(license: &mut Arc<License>) {
        let (cpu, mem, disk, socket_count) = get_info();
        let license = license.clone();
        let mut body =
            js_httpc::HttpClientBody::json(Atom::from("license"), license.license.clone());
        body.add_json_kv("client_id".to_string(), license.client_id.clone());
        body.add_json_kv("sys".to_string(), license.os.clone());
        body.add_json_kv("mem".to_string(), mem.to_string());
        body.add_json_kv("cpu".to_string(), cpu.to_string());
        body.add_json_kv("disk".to_string(), disk.to_string());
        body.add_json_kv("connect".to_string(), socket_count.to_string());

        let options = js_httpc::HttpClientOptions::normal(true, false, false, 0, 10000);
        let mut client =
            js_httpc::create_http_client("license_client".to_string(), options).unwrap();
        js_httpc::post(
            &mut client,
            Atom::from("https://license.yinengyun.com:10443/license/verify"),
            body,
            Box::new(
                move |result: Result<
                    (Arc<httpc::HttpClient>, httpc::HttpClientResponse),
                    String,
                >| {
                    match result {
                        Err(_s) => {
                            let heart_count = license.heart_count.load(Ordering::Relaxed);
                            let heart_error = license.heart_error.load(Ordering::Relaxed);
                            if heart_error >= heart_count {
                                ENABLE_ALLOC.store(false, Ordering::Relaxed);
                            } else {
                                let new_error = heart_error + 1;
                                license.heart_error.store(new_error, Ordering::SeqCst);
                            }
                        }
                        Ok((_, mut resp)) => {
                            let text = resp.text().unwrap();
                            match check_license(text) {
                                Ok((overtime, heartbeat, heart_count)) => {
                                    let new_overtime = overtime;
                                    // 更新license到期时间
                                    license.overtime.store(new_overtime, Ordering::SeqCst);
                                    // 重置失败尝试次数
                                    license.heart_error.store(0, Ordering::SeqCst);
                                    // 更新心跳间隔
                                    license
                                        .heartbeat
                                        .store(heartbeat as usize, Ordering::SeqCst);
                                    // 更新心跳尝试次数
                                    license
                                        .heart_count
                                        .store(heart_count as u8, Ordering::SeqCst);
                                    let now = now_millisecond();
                                    if new_overtime < now {
                                        ENABLE_ALLOC.store(false, Ordering::Relaxed);
                                    }
                                }
                                Err(_s) => {
                                    let heart_count = license.heart_count.load(Ordering::Relaxed);
                                    let heart_error = license.heart_error.load(Ordering::Relaxed);
                                    if heart_error >= heart_count {
                                        ENABLE_ALLOC.store(false, Ordering::Relaxed);
                                    } else {
                                        let new_error = heart_error + 1;
                                        license.heart_error.store(new_error, Ordering::SeqCst);
                                    }
                                }
                            };
                        }
                    }
                },
            ),
        );
    }
}

// 获取系统信息
fn get_info() -> (usize, u64, u64, usize) {
    let sys = SysStat::new();
    // CPU核
    let cpu = SysStat::processor_count(&sys);
    // 内存信息
    let (mem, _, _, _, _, _) = SysStat::memory_usage(&sys);
    // 磁盘信息
    let (_, _, _, _, _, disk) = SysStat::disk_usage(&sys)[0];
    // 连接数
    let socket_size = SysStat::sockets_size(&sys, NetIPType::All, NetProtocolType::All);

    return (cpu, mem, disk, socket_size);
}

// 验证签名
fn check_license(text: String) -> Result<(u64, u64, u64), String> {
    let license_data: Value = serde_json::from_str(&text).unwrap();
    let code = license_data["return_code"].as_str();
    match code {
        Some(code2) => {
            if code2 == "0" {
                let data = &license_data["data"];
                let sign = license_data["sign"].as_str().unwrap();
                // 验证签名
                let license = &data["license"].as_str().unwrap();
                let overtime = &data["overtime"].as_u64().unwrap();
                let timestamp = &data["timestamp"].as_u64().unwrap();
                let heartbeat = &license_data["heartbeat"].as_u64().unwrap();
                let heart_count = &license_data["heart_count"].as_u64().unwrap();
                let mut msg = "license=".to_string();
                msg = msg
                    + license
                    + "&overtime="
                    + &overtime.to_string()
                    + "&timestamp="
                    + &timestamp.to_string();
                let hash = digest(DigestAlgorithm::SHA256, msg.as_bytes());
                let secp = ECDSASecp256k1::new();
                let pk = Vec::from_hex("0440990801123db1cfe33d43ca968d684f76e9f23b9dff885eacb94c9c896b38fba6259a275d0ef6e18464b9800193034d5cb90a1640a9b6190f118943a662137d").unwrap();
                let sign_bin = Vec::from_hex(&sign).unwrap();
                if secp.verify(&hash, sign_bin.as_ref(), pk.as_ref()) {
                    return Result::Ok((overtime.clone(), heartbeat.clone(), heart_count.clone()));
                } else {
                    return Result::Err("sign error".to_string());
                }
            } else {
                return Result::Err("code error".to_string());
            }
        }
        None => {
            return Result::Err("code error".to_string());
        }
    }
}

// 获取client UUID
#[cfg(any(windows))]
fn get_uuid() -> String {
    let outpub = Command::new("wmic")
        .args(&["csproduct", "get", "UUID"])
        .output();
    match outpub {
        Ok(outpub2) => {
            let text = String::from_utf8(outpub2.stdout).ok().unwrap();
            let re = Regex::new(r"(\w+\-){4}\w+").unwrap();
            let mat = re.find(&text).unwrap();

            return mat.as_str().to_string();
        }
        Err(_) => {
            // 获取uuid失败
            return "".to_string();
        }
    }
}

// 获取client UUID
#[cfg(any(unix))]
fn get_uuid() -> String {
    let outpub = Command::new("dmidecode")
        .args(&["-s", "system-uuid"])
        .output();
    match outpub {
        Ok(outpub2) => {
            let text = String::from_utf8(outpub2.stdout).ok().unwrap();
            let re = Regex::new(r"(\w+\-){4}\w+").unwrap();
            let mat = re.find(&text).unwrap();

            return mat.as_str().to_string();
        }
        Err(_) => {
            // 获取uuid失败
            return "".to_string();
        }
    }
}
