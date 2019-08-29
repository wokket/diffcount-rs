use gtk::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

mod main_window;
use main_window::MainWindow;

mod channel;
use channel::Channel;

mod state;
use state::State;

fn main() {
    // Start up the GTK3 subsystem.
    gtk::init().expect("Unable to start GTK3. Error");
    // Create the main window.
    let gui = Arc::new(MainWindow::new());

    // Create our channel state
    let mut channels: HashMap<i8, Channel> = HashMap::new();
    for i in 1..gui.num_channels + 1 {
        channels.insert(i, Channel::new(i));
    }

    let state: Arc<State> = Arc::new(State {
        channels: RefCell::new(channels),
    });

    update_ui(&gui, &state);

    for i in 1..gui.num_channels + 1 {
        let btn = gui.get_button(&i);
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);

        btn.connect_clicked(move |_| {
            let mut channels = state.channels.borrow_mut();

            let channel = channels
                .get_mut(&i)
                .expect(&format!("Unable to find channel at index {}", i));
            // actually bump up the value
            channel.increment();

            let count = channel.get_count();
            //let total = state.total_count();
            let total = channels.iter().map(|(_, c)| c.get_count()).sum();
            gui.update_channel(&i, count, calc_percentage(count, total));
        });
    }

    gui.start();
    gtk::main();
}

fn update_ui(gui: &Arc<MainWindow>, state: &State) {
    for (i, channel) in state.channels.borrow().iter() {
        gui.update_channel(&i, channel.get_count(), 0.0);
    }
}

fn calc_percentage(x: i32, t: i32) -> f32 {
    if t == 0 {
        return 0.0;
    }

    return x as f32 / t as f32;
}
