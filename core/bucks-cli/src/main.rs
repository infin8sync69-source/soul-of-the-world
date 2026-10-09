//! `bucks`: a development and testing CLI for the identity core.
//!
//! WARNING: private keys are stored as plain hex in `keys.json` (file mode 0600). This tool is
//! for developing and testing the protocol. It is NOT a wallet and must never hold a real
//! identity. Apps use hardware-backed keystores through the core's `Signer` interface.

use bucks_core::{replay, Device, Event, EventDraft, Op, OpBody, Signer, State};
use clap::{Parser, Subcommand};
use p256::ecdsa::SigningKey;
use rand_core::{OsRng, RngCore};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(
    name = "bucks",
    version,
    about = "Development CLI for Bucks identities (test keys only)"
)]
struct Cli {
    /// Directory holding keys.json, log.json and streams.json.
    #[arg(long, global = true, default_value = "./bucks-dev")]
    home: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Identity commands.
    Id {
        #[command(subcommand)]
        cmd: IdCmd,
    },
    /// Device commands.
    Device {
        #[command(subcommand)]
        cmd: DeviceCmd,
    },
    /// Replace the primary rotation key (the backup key is kept).
    Rotate,
    /// Event commands.
    Event {
        #[command(subcommand)]
        cmd: EventCmd,
    },
    /// Verify a log file (JSON with an "ops" array of hex strings) and print its state.
    VerifyLog { file: PathBuf },
}

#[derive(Subcommand)]
enum IdCmd {
    /// Create a new identity with a primary and a backup rotation key and one device.
    New {
        #[arg(long)]
        device_name: String,
        /// Home node URLs to list in the inception (repeatable).
        #[arg(long = "home-node")]
        home_nodes: Vec<String>,
    },
    /// Verify the local log and print the current state.
    Show,
}

#[derive(Subcommand)]
enum DeviceCmd {
    /// Create a device key and add it to the identity.
    Add {
        #[arg(long)]
        name: String,
    },
    /// Remove a device by its hex id.
    Remove {
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum EventCmd {
    /// Sign an event with a device key, continuing that device's stream.
    Sign {
        #[arg(long)]
        device: String,
        #[arg(long)]
        kind: String,
        #[arg(long, default_value = "")]
        payload: String,
    },
    /// Verify a hex-encoded event against a log (default: the local log).
    Verify {
        #[arg(long)]
        event: String,
        #[arg(long)]
        log: Option<PathBuf>,
    },
}

type Res<T> = Result<T, String>;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unhex(s: &str) -> Res<Vec<u8>> {
    let s = s.trim();
    if s.len() % 2 != 0 || !s.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("invalid hex".into());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

fn unhex_n<const N: usize>(s: &str) -> Res<[u8; N]> {
    unhex(s)?
        .try_into()
        .map_err(|_| format!("expected {N} bytes"))
}

fn read_json(p: &Path) -> Res<Value> {
    let t = fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
    serde_json::from_str(&t).map_err(|e| format!("{}: {e}", p.display()))
}

fn write_json(p: &Path, v: &Value, secret: bool) -> Res<()> {
    let t = serde_json::to_string_pretty(v).map_err(|e| e.to_string())? + "\n";
    fs::write(p, t).map_err(|e| format!("{}: {e}", p.display()))?;
    #[cfg(unix)]
    if secret {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(p, fs::Permissions::from_mode(0o600)).map_err(|e| e.to_string())?;
    }
    #[cfg(not(unix))]
    let _ = secret;
    Ok(())
}

struct Home {
    dir: PathBuf,
}

impl Home {
    fn keys(&self) -> PathBuf {
        self.dir.join("keys.json")
    }
    fn log(&self) -> PathBuf {
        self.dir.join("log.json")
    }
    fn streams(&self) -> PathBuf {
        self.dir.join("streams.json")
    }
    fn load_ops(path: &Path) -> Res<Vec<Op>> {
        let v = read_json(path)?;
        let arr = v["ops"]
            .as_array()
            .ok_or("log file needs an \"ops\" array")?;
        arr.iter()
            .map(|x| {
                Op::from_bytes(&unhex(x.as_str().ok_or("op must be a hex string")?)?)
                    .map_err(|e| e.to_string())
            })
            .collect()
    }
    fn save_ops(&self, ops: &[Op]) -> Res<()> {
        write_json(
            &self.log(),
            &json!({ "ops": ops.iter().map(|o| hex(&o.to_bytes())).collect::<Vec<_>>() }),
            false,
        )
    }
    fn history(&self) -> Res<(Vec<Op>, Vec<State>)> {
        let ops = Self::load_ops(&self.log())?;
        let h = replay(&ops).map_err(|e| e.to_string())?;
        Ok((ops, h))
    }
    fn key(v: &Value) -> Res<SigningKey> {
        let b = unhex(v.as_str().ok_or("key must be hex")?)?;
        SigningKey::from_slice(&b).map_err(|_| "invalid secret key".into())
    }
    fn rotation_primary(&self) -> Res<SigningKey> {
        Self::key(&read_json(&self.keys())?["rotation"][0])
    }
    /// Append an op signed by the primary rotation key, verify the whole log, then save.
    fn append(&self, body: OpBody) -> Res<State> {
        let (mut ops, h) = self.history()?;
        let last = h.last().ok_or("empty log")?;
        let op = Op::sign(
            last.seq + 1,
            Some(last.head),
            body,
            &self.rotation_primary()?,
        )
        .map_err(|e| e.to_string())?;
        ops.push(op);
        let st = replay(&ops)
            .map_err(|e| e.to_string())?
            .pop()
            .ok_or("empty log")?;
        self.save_ops(&ops)?;
        Ok(st)
    }
}

fn new_key() -> SigningKey {
    SigningKey::random(&mut OsRng)
}

fn new_device_id() -> [u8; 16] {
    let mut id = [0u8; 16];
    OsRng.fill_bytes(&mut id);
    id
}

fn secret_hex(k: &SigningKey) -> String {
    hex(&k.to_bytes())
}

fn state_json(st: &State) -> Value {
    json!({
        "bucks_id": st.id.to_string(),
        "did": st.id.did(),
        "short_code": st.id.short_code(),
        "seq": st.seq,
        "head": hex(&st.head),
        "rotation_keys": st.rotation_keys.iter().map(|k| k.did_key()).collect::<Vec<_>>(),
        "devices": st.devices.iter().map(|d| json!({"id": hex(&d.id), "name": d.name, "key": d.key.did_key()})).collect::<Vec<_>>(),
        "home": st.home,
    })
}

fn run(cli: Cli) -> Res<Value> {
    let home = Home { dir: cli.home };
    match cli.cmd {
        Cmd::Id {
            cmd: IdCmd::New {
                device_name,
                home_nodes,
            },
        } => {
            if home.keys().exists() || home.log().exists() {
                return Err(format!("{} already holds an identity", home.dir.display()));
            }
            fs::create_dir_all(&home.dir).map_err(|e| e.to_string())?;
            let (primary, backup, device_key) = (new_key(), new_key(), new_key());
            let device = Device {
                id: new_device_id(),
                key: device_key.public_key(),
                name: device_name,
            };
            let body = OpBody::Inception {
                rotation_keys: vec![primary.public_key(), backup.public_key()],
                devices: vec![device.clone()],
                home: home_nodes,
            };
            let op = Op::sign(0, None, body, &primary).map_err(|e| e.to_string())?;
            let st = replay(std::slice::from_ref(&op))
                .map_err(|e| e.to_string())?
                .remove(0);
            write_json(
                &home.keys(),
                &json!({
                    "warning": "DEVELOPMENT KEYS IN PLAIN TEXT. Never use for a real identity.",
                    "rotation": [secret_hex(&primary), secret_hex(&backup)],
                    "devices": { hex(&device.id): secret_hex(&device_key) },
                }),
                true,
            )?;
            home.save_ops(&[op])?;
            write_json(&home.streams(), &json!({}), false)?;
            Ok(state_json(&st))
        }
        Cmd::Id { cmd: IdCmd::Show } => {
            Ok(state_json(home.history()?.1.last().ok_or("empty log")?))
        }
        Cmd::Device {
            cmd: DeviceCmd::Add { name },
        } => {
            let k = new_key();
            let device = Device {
                id: new_device_id(),
                key: k.public_key(),
                name,
            };
            let st = home.append(OpBody::AddDevice {
                device: device.clone(),
            })?;
            let mut keys = read_json(&home.keys())?;
            keys["devices"][hex(&device.id)] = json!(secret_hex(&k));
            write_json(&home.keys(), &keys, true)?;
            Ok(json!({ "added": hex(&device.id), "state": state_json(&st) }))
        }
        Cmd::Device {
            cmd: DeviceCmd::Remove { id },
        } => {
            let device_id = unhex_n::<16>(&id)?;
            let st = home.append(OpBody::RemoveDevice { device_id })?;
            let mut keys = read_json(&home.keys())?;
            if let Some(m) = keys["devices"].as_object_mut() {
                m.remove(&hex(&device_id));
            }
            write_json(&home.keys(), &keys, true)?;
            Ok(json!({ "removed": hex(&device_id), "state": state_json(&st) }))
        }
        Cmd::Rotate => {
            let mut keys = read_json(&home.keys())?;
            let backup = Home::key(&keys["rotation"][1])?;
            let fresh = new_key();
            let st = home.append(OpBody::RotateKeys {
                rotation_keys: vec![fresh.public_key(), backup.public_key()],
            })?;
            keys["rotation"][0] = json!(secret_hex(&fresh));
            write_json(&home.keys(), &keys, true)?;
            Ok(state_json(&st))
        }
        Cmd::Event {
            cmd:
                EventCmd::Sign {
                    device,
                    kind,
                    payload,
                },
        } => {
            let device_id = unhex_n::<16>(&device)?;
            let (_, h) = home.history()?;
            let st = h.last().ok_or("empty log")?;
            let keys = read_json(&home.keys())?;
            let key = Home::key(&keys["devices"][hex(&device_id)])
                .map_err(|_| "no local key for that device".to_string())?;
            let mut streams = read_json(&home.streams())?;
            let s = &streams[hex(&device_id)];
            let seq = s["seq"].as_u64().unwrap_or(0);
            let prev = match s["prev"].as_str() {
                Some(p) => Some(unhex_n::<32>(p)?),
                None => None,
            };
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_millis() as u64;
            let draft = EventDraft {
                kind,
                author: st.id,
                device: device_id,
                seq,
                prev,
                ctx: st.seq,
                ts,
                payload: payload.into_bytes(),
            };
            let ev = draft.sign(&key).map_err(|e| e.to_string())?;
            ev.verify(&h)
                .map_err(|e| format!("self-check failed: {e}"))?;
            streams[hex(&device_id)] = json!({ "seq": seq + 1, "prev": hex(&ev.hash()) });
            write_json(&home.streams(), &streams, false)?;
            Ok(json!({ "event": hex(&ev.to_bytes()), "hash": hex(&ev.hash()), "seq": seq }))
        }
        Cmd::Event {
            cmd: EventCmd::Verify { event, log },
        } => {
            let ops = Home::load_ops(&log.unwrap_or_else(|| home.log()))?;
            let h = replay(&ops).map_err(|e| e.to_string())?;
            let ev = Event::from_bytes(&unhex(&event)?).map_err(|e| e.to_string())?;
            ev.verify(&h).map_err(|e| e.to_string())?;
            Ok(
                json!({ "valid": true, "hash": hex(&ev.hash()), "kind": ev.draft.kind, "seq": ev.draft.seq }),
            )
        }
        Cmd::VerifyLog { file } => {
            let ops = Home::load_ops(&file)?;
            let st = replay(&ops)
                .map_err(|e| e.to_string())?
                .pop()
                .ok_or("empty log")?;
            Ok(state_json(&st))
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(v) => {
            println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
