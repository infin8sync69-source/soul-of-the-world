//! End-to-end test of the `bucks` binary: create, add device, sign, verify, remove, rotate.
use serde_json::Value;
use std::path::Path;
use std::process::Command;

fn bucks(home: &Path, args: &[&str]) -> (bool, Value, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_bucks"))
        .arg("--home")
        .arg(home)
        .args(args)
        .output()
        .unwrap();
    let v = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    (
        out.status.success(),
        v,
        String::from_utf8_lossy(&out.stderr).into(),
    )
}

#[test]
fn full_identity_lifecycle() {
    let home = std::env::temp_dir().join(format!("bucks-cli-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);

    let (ok, st, err) = bucks(
        &home,
        &[
            "id",
            "new",
            "--device-name",
            "phone",
            "--home-node",
            "https://node.example",
        ],
    );
    assert!(ok, "{err}");
    let id = st["bucks_id"].as_str().unwrap().to_string();
    let phone = st["devices"][0]["id"].as_str().unwrap().to_string();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(home.join("keys.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600, "key file must be private");
    }

    let (ok, _, _) = bucks(&home, &["id", "new", "--device-name", "again"]);
    assert!(!ok, "must refuse to overwrite an identity");

    let (ok, added, err) = bucks(&home, &["device", "add", "--name", "laptop"]);
    assert!(ok, "{err}");
    let laptop = added["added"].as_str().unwrap().to_string();

    let (ok, e1, err) = bucks(
        &home,
        &[
            "event",
            "sign",
            "--device",
            &phone,
            "--kind",
            "bucks.post.create",
            "--payload",
            "hello",
        ],
    );
    assert!(ok, "{err}");
    let (ok, e2, _) = bucks(
        &home,
        &[
            "event",
            "sign",
            "--device",
            &phone,
            "--kind",
            "bucks.post.create",
            "--payload",
            "again",
        ],
    );
    assert!(ok);
    assert_eq!(e2["seq"], 1, "stream continues");
    let (ok, v, err) = bucks(
        &home,
        &["event", "verify", "--event", e1["event"].as_str().unwrap()],
    );
    assert!(ok && v["valid"] == true, "{err}");

    let (ok, _, err) = bucks(&home, &["device", "remove", "--id", &phone]);
    assert!(ok, "{err}");
    let (ok, _, err) = bucks(
        &home,
        &["event", "verify", "--event", e1["event"].as_str().unwrap()],
    );
    assert!(
        !ok && err.contains("device not authorised"),
        "events of a removed device stop verifying: {err}"
    );

    let (ok, _, err) = bucks(
        &home,
        &[
            "event",
            "sign",
            "--device",
            &laptop,
            "--kind",
            "bucks.post.create",
        ],
    );
    assert!(ok, "{err}");

    let (ok, rotated, err) = bucks(&home, &["rotate"]);
    assert!(ok, "{err}");
    assert_eq!(rotated["bucks_id"], id.as_str(), "rotation keeps the id");
    let (ok, shown, _) = bucks(&home, &["id", "show"]);
    assert!(ok);
    assert_eq!(shown["seq"], 3);
    assert_eq!(shown["bucks_id"], id.as_str());

    let (ok, verified, _) = bucks(
        &home,
        &["verify-log", home.join("log.json").to_str().unwrap()],
    );
    assert!(ok);
    assert_eq!(verified["head"], shown["head"]);

    std::fs::remove_dir_all(&home).unwrap();
}
