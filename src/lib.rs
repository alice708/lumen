use std::env;
use tokio::sync::mpsc::{self, UnboundedReceiver};

slint::include_modules!();

enum Command {
    Power(bool),
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let ui = MainWindow::new()?;
    let (tx, rx) = mpsc::unbounded_channel::<Command>();

    ui.on_power_toggled({
        let tx = tx.clone();
        move |on| {
            let _ = tx.send(Command::Power(on));
        }
    });

    rt.spawn(worker(rx, ui.as_weak()));

    ui.run()?;
    Ok(())
}

fn set_status(weak: &slint::Weak<MainWindow>, msg: String) {
    let _ = weak.upgrade_in_event_loop(move |ui| ui.set_status(msg.into()));
}

async fn connect() -> Result<tapo::ColorLightHandler, Box<dyn std::error::Error>> {
    let user = std::env::var("TAPO_USERNAME")?;
    let pass = std::env::var("TAPO_PASSWORD")?;
    let ip = std::env::var("IP_ADDRESS")?;
    Ok(tapo::ApiClient::new(user, pass).l630(ip).await?)
}

async fn worker(mut rx: UnboundedReceiver<Command>, weak: slint::Weak<MainWindow>) {
    let device = match connect().await {
        Ok(d) => {
            set_status(&weak, "Connected".into());
            d
        }
        Err(e) => {
            set_status(&weak, format!("Connect failed: {e}"));
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
            Ok(()) => set_status(&weak, "OK".into()),
            Err(e) => set_status(&weak, format!("Error: {e}")),
        }
    }
}
