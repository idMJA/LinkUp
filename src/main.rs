mod config;
mod ngrok;
mod webhook;

use anyhow::{Context, Result, anyhow};
use config::{Config, NgrokInstance, Settings};
use log::{error, info, warn};
use ngrok::NgrokManager;
use std::collections::HashSet;
use std::path::PathBuf;
use tokio::signal;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};
use webhook::WebhookNotifier;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(&config_path).context("Failed to load configuration")?;
    let (check_interval, auto_restart_enabled) = init_logging(&config.settings);

    info!("Starting LinkUp - Ngrok Manager");
    info!("Loaded configuration from: {}", config_path.display());
    info!("Found {} ngrok instance(s)", config.ngrok_instances.len());

    let notifier = WebhookNotifier::new(config.webhooks.clone());
    let mut manager = NgrokManager::new();
    let valid_instances = configure_instances(&mut manager, &config)?;

    info!("Starting all ngrok instances...");
    manager.start_all().await?;
    sleep(Duration::from_secs(5)).await;

    let mut notified_instances = HashSet::new();
    notify_initial_tunnels(
        &manager,
        &notifier,
        &valid_instances,
        &mut notified_instances,
    )
    .await;

    let health_check_handle = spawn_health_monitor(
        manager,
        notifier,
        notified_instances,
        check_interval,
        auto_restart_enabled,
    );

    info!("LinkUp is running. Press Ctrl+C to stop.");

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Received shutdown signal, stopping...");
            health_check_handle.abort();
            info!("All instances stopped. Goodbye!");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {err}");
        }
    }

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        return Ok(PathBuf::from(&args[1]));
    }

    let current_dir_config = PathBuf::from("config.toml");
    if current_dir_config.exists() {
        return Ok(current_dir_config);
    }

    if let Ok(home) = std::env::var("HOME") {
        let home_config = PathBuf::from(home).join(".config/linkup/config.toml");
        if home_config.exists() {
            return Ok(home_config);
        }
    }

    let etc_config = PathBuf::from("/etc/linkup/config.toml");
    if etc_config.exists() {
        return Ok(etc_config);
    }

    Err(anyhow!(
        "Config file not found. Please create config.toml in one of these locations:\n\
         - Current directory\n\
         - ~/.config/linkup/config.toml\n\
         - /etc/linkup/config.toml\n\
         Or specify the path as a command line argument."
    ))
}

fn init_logging(settings: &Settings) -> (Duration, bool) {
    unsafe {
        std::env::set_var("RUST_LOG", &settings.log_level);
    }
    env_logger::init();
    (
        Duration::from_secs(settings.check_interval_seconds),
        settings.auto_restart,
    )
}

fn configure_instances(manager: &mut NgrokManager, config: &Config) -> Result<Vec<NgrokInstance>> {
    let mut valid_instances = Vec::new();
    for instance in &config.ngrok_instances {
        match manager.add_instance(instance.clone()) {
            Ok(()) => {
                info!("Configured ngrok instance: {}", instance.name);
                valid_instances.push(instance.clone());
            }
            Err(e) => {
                error!("Skipping invalid instance '{}': {e}", instance.name);
            }
        }
    }

    if valid_instances.is_empty() {
        Err(anyhow!(
            "No valid ngrok instances configured. Please check your config."
        ))
    } else {
        Ok(valid_instances)
    }
}

async fn notify_initial_tunnels(
    manager: &NgrokManager,
    notifier: &WebhookNotifier,
    instances: &[NgrokInstance],
    already_sent: &mut HashSet<String>,
) {
    for instance in instances {
        match manager.get_tunnels(&instance.name).await {
            Ok(tunnels) if tunnels.is_empty() => {
                warn!("No tunnels found for instance '{}'", instance.name);
            }
            Ok(tunnels) => {
                info!(
                    "Tunnels for '{}': {} tunnel(s)",
                    instance.name,
                    tunnels.len()
                );
                let _ = notifier
                    .notify_tunnel_created(&instance.name, &tunnels)
                    .await;
                already_sent.insert(instance.name.clone());
            }
            Err(e) => {
                error!("Failed to get tunnels for '{}': {e}", instance.name);
                let error_message = format!("Failed to start tunnels: {e}");
                let _ = notifier.notify_error(&instance.name, &error_message).await;
            }
        }
    }
}

fn spawn_health_monitor(
    mut manager: NgrokManager,
    notifier: WebhookNotifier,
    mut already_sent: HashSet<String>,
    check_interval: Duration,
    auto_restart_enabled: bool,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            sleep(check_interval).await;

            let health = manager.check_health();
            for (name, is_healthy) in health {
                if is_healthy {
                    continue;
                }

                error!("Instance '{name}' is not healthy");

                if !auto_restart_enabled {
                    continue;
                }

                warn!("Auto-restarting instance '{name}'");
                let _ = notifier.notify_restart(&name).await;

                match manager.restart_instance(&name).await {
                    Ok(()) => {
                        info!("Successfully restarted instance '{name}'");
                        sleep(Duration::from_secs(5)).await;
                        if !already_sent.contains(&name) {
                            match manager.get_tunnels(&name).await {
                                Ok(tunnels) if tunnels.is_empty() => {}
                                Ok(tunnels) => {
                                    let _ = notifier.notify_tunnel_created(&name, &tunnels).await;
                                    already_sent.insert(name.clone());
                                }
                                Err(e) => {
                                    error!(
                                        "Failed to fetch tunnels for '{name}' after restart: {e}"
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to restart instance '{name}': {e}");
                        let error_message = format!("Failed to restart: {e}");
                        let _ = notifier.notify_error(&name, &error_message).await;
                    }
                }
            }
        }
    })
}
