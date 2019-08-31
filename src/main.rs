use gtk::prelude::*;
use std::cell::RefCell;
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
    let mut channels: Vec<Channel> = Vec::new();
    for i in MainWindow::channel_range() {
        channels.push(Channel::new(i));
    }

    let state: Arc<State> = Arc::new(State {
        channels: RefCell::new(channels),
        alarm_count: 10,
    });

    gui.update(&state);

    for i in MainWindow::channel_range() {
        let btn = gui.get_button(i);
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);

        btn.connect_clicked(move |_| {
            {
                //one block for the mut borrow in order to increment the data
                let mut channels = state.channels.borrow_mut();
                let channel = channels
                    .get_mut(i as usize)
                    .expect(&format!("Unable to find channel at index {}", i));
                // actually bump up the value
                channel.increment();
            } //drop the mut borrow here

            gui.update(&state); //this takes a non-mut borrow
        });
    }

    gui.start();
    gtk::main();
}
