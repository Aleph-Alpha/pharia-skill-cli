use std::{env, fs::File, io::Write, net::TcpListener, path::Path};

use assert_cmd::Command;
use pharia_kernel::{AppConfig, Kernel, NamespaceConfigs};
use predicates::str::contains;
use tokio::sync::oneshot;

#[test]
fn invalid_args() {
    let mut cmd = Command::cargo_bin("pharia-skill-cli").unwrap();
    let cmd = cmd
        .arg("publish")
        .arg("-R")
        .arg("dummy-registry")
        .arg("-r")
        .arg("dummy-repo")
        .arg("-u")
        .arg("dummy_user")
        .arg("-p")
        .arg("dummy_token")
        .arg("dummy.wasm");
    cmd.assert().failure();
}

fn wasm_file() -> &'static Path {
    let path = Path::new("./skills/test-skill.wasm");
    if !path.exists() {
        let mut file = File::create(path).unwrap();
        let content = wat::parse_str("(module)").unwrap();
        file.write_all(&content).unwrap();
    }
    path
}

#[test]
fn publish_minimal_args() {
    drop(dotenvy::dotenv());
    let path = wasm_file();
    let mut cmd = Command::cargo_bin("pharia-skill-cli").unwrap();
    let cmd = cmd
        .arg("publish")
        .arg(path)
        .arg("-t")
        .arg("0.0.1")
        .env(
            "SKILL_REGISTRY",
            env::var("SKILL_REGISTRY").expect("SKILL_REGISTRY must be set."),
        )
        .env(
            "SKILL_REPOSITORY",
            env::var("SKILL_REPOSITORY").expect("SKILL_REPOSITORY must be set."),
        )
        .env(
            "SKILL_REGISTRY_USER",
            env::var("SKILL_REGISTRY_USER").expect("SKILL_REGISTRY_USER must be set."),
        )
        .env(
            "SKILL_REGISTRY_TOKEN",
            env::var("SKILL_REGISTRY_TOKEN").expect("SKILL_REGISTRY_TOKEN must be set."),
        );
    cmd.assert().success();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn run_skill() {
    // given a Pharia Kernel instance
    let kernel = TestKernel::with_defaults().await;

    // when running a skill
    drop(dotenvy::dotenv());
    let mut cmd = Command::cargo_bin("pharia-skill-cli").unwrap();
    let cmd = cmd
        .arg("run")
        .arg("-n")
        .arg("dev/greet-skill")
        .arg("-i")
        .arg("Homer")
        .arg("-l")
        .arg(format!("http://127.0.0.1:{}", kernel.port()))
        .env(
            "PHARIA_AI_TOKEN",
            env::var("PHARIA_AI_TOKEN").expect("PHARIA_AI_TOKEN must be set."),
        );

    // then the output must contain the expected value
    cmd.assert().stdout(contains("Homer"));

    kernel.shutdown().await;
}

struct TestKernel {
    kernel: Kernel,
    shutdown_trigger: oneshot::Sender<()>,
    port: u16,
}

impl TestKernel {
    async fn new(app_config: AppConfig) -> Self {
        let (shutdown_trigger, shutdown_capture) = oneshot::channel::<()>();
        let shutdown_signal = async {
            shutdown_capture.await.unwrap();
        };
        let port = app_config.kernel_address.port();
        let kernel = Kernel::new(app_config, shutdown_signal).await.unwrap();
        Self {
            kernel,
            shutdown_trigger,
            port,
        }
    }

    async fn with_defaults() -> Self {
        let port = free_test_port();
        let metrics_port = free_test_port();
        let app_config = AppConfig {
            kernel_address: format!("127.0.0.1:{port}").parse().unwrap(),
            metrics_address: format!("127.0.0.1:{metrics_port}").parse().unwrap(),
            namespaces: NamespaceConfigs::dev(),
            ..Default::default()
        };
        Self::new(app_config).await
    }

    fn port(&self) -> u16 {
        self.port
    }

    async fn shutdown(self) {
        self.shutdown_trigger.send(()).unwrap();
        self.kernel.wait_for_shutdown().await;
    }
}

/// Ask the operating system for the next free port
fn free_test_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}
