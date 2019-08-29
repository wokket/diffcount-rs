use gtk::prelude::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

mod main_window;
use main_window::MainWindow;

mod channel;
use channel::Channel;

struct State {
    channels: RefCell<HashMap<i8, Channel>>,
}

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
        {
            let btn = gui.get_button(&i);
            let gui = Arc::clone(&gui);
            let state = Arc::clone(&state);

            btn.connect_clicked(move |_| {
                let mut channels = state.channels.borrow_mut();

                let channel = channels
                    .get_mut(&i)
                    .expect(&format!("Unable to find channel at index {}", i));
                channel.increment();

                gui.update_channel(&i, channel.get_count());
            });
        }
    }

    gui.start();
    gtk::main();
}

fn update_ui(gui: &Arc<MainWindow>, state: &State) {
    for (i, channel) in state.channels.borrow().iter() {
        gui.update_channel(&i, channel.get_count());
    }
}
