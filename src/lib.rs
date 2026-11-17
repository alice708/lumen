use std::{env, net::IpAddr};
use tokio::sync::mpsc::{self, UnboundedReceiver};

slint::include_modules!();

#[derive(Debug, Clone, Copy)]
enum Command {
    Power(bool),
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let ui = MainWindow::new()?;

    let ips = env::var("BULB_IPS")?
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<IpAddr>()
                .map_err(|e| format!("Bad IP {s:?}: {e}"))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let user = env::var("TAPO_USERNAME")?;
    let pass = env::var("TAPO_PASSWORD")?;

    let mut senders = Vec::with_capacity(ips.len());
    for (idx, ip) in ips.iter().enumerate() {
        let (tx, rx) = mpsc::unbounded_channel::<Command>();
        senders.push(tx);
        rt.spawn(worker(
            idx,
            user.clone(),
            pass.clone(),
            ip.to_string(),
            rx,
            ui.as_weak(),
        ));
    }
    let senders = std::sync::Arc::new(senders);

    ui.on_power_toggled( move |on| broadcast(&senders, Command::Power(on)) );

    ui.run()?;
    Ok(())
}

fn broadcast(senders: &[mpsc::UnboundedSender<Command>], cmd: Command) {
    for tx in senders {
        let _ = tx.send(cmd.clone());
    }
}

fn set_status(weak: &slint::Weak<MainWindow>, msg: String) {
    let _ = weak.upgrade_in_event_loop(move |ui| ui.set_status(msg.into()));
}

async fn worker(
    idx: usize,
    user: String,
    pass: String,
    ip: String,
    mut rx: UnboundedReceiver<Command>,
    ui: slint::Weak<MainWindow>,
) {
    let device = match tapo::ApiClient::new(user, pass).l630(ip).await {
        Ok(d) => d,
        Err(e) => {
            set_status(&ui, format!("Bulb {}: connect failed: {e}", idx + 1));
            return;
        }
    };

    // One task owns the bulb, so commands are applied one at a time, in order.
    while let Some(cmd) = rx.recv().await {
        let result = match cmd {
            Command::Power(true) => device.on().await,
            Command::Power(false) => device.off().await,
        };
        match result {
            Ok(()) => set_status(&ui, "OK".into()),
            Err(e) => set_status(&ui, format!("Error: {e}")),
        }
    }
}
