use std::env;
use tokio::sync::mpsc::{self, UnboundedReceiver};

slint::include_modules!();

enum Command {
    Power(bool),
    Brightness(u8),
    ColorTemp(u16),
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let ui = MainWindow::new()?;
    let (tx, rx) = mpsc::unbounded_channel::<Command>();

    ui.on_power_toggled({
        let tx = tx.clone();
        move |on| { let _ = tx.send(Command::Power(on)); }
    });
    ui.on_brightness_committed({
        let tx = tx.clone();
        move |v| { let _ = tx.send(Command::Brightness(v as u8)); }
    });
    ui.on_temp_committed({
        let tx = tx.clone();
        move |v| { let _ = tx.send(Command::ColorTemp(v as u16)); }
    });

    rt.spawn(worker(rx, ui.as_weak()));

    ui.run()?;
    Ok(())
}

fn set_status(weak: &slint::Weak<MainWindow>, msg: String) {
    let _ = weak.upgrade_in_event_loop(move |ui| ui.set_status(msg.into()));
}

async fn worker(mut rx: UnboundedReceiver<Command>, weak: slint::Weak<MainWindow>) {
    let connect = async {
        let user = env::var("TAPO_USERNAME")?;
        let pass = env::var("TAPO_PASSWORD")?;
        let ip = env::var("IP_ADDRESS")?;
        // Use the same constructor as in hello bulb.
        let device = tapo::ApiClient::new(user, pass).l630(ip).await?;
        Ok::<_, Box<dyn std::error::Error>>(device)
    };

    let device = match connect.await {
        Ok(d) => { set_status(&weak, "Connected".into()); d }
        Err(e) => { set_status(&weak, format!("Connect failed: {e}")); return; }
    };

    // One task owns the bulb, so commands are applied one at a time, in order.
    while let Some(cmd) = rx.recv().await {
        let result = match cmd {
            Command::Power(true) => device.on().await,
            Command::Power(false) => device.off().await,
            Command::Brightness(b) => device.set_brightness(b).await,
            Command::ColorTemp(t) => device.set_color_temperature(t).await,
        };
        match result {
            Ok(()) => set_status(&weak, "OK".into()),
            Err(e) => set_status(&weak, format!("Error: {e}")),
        }
    }
}