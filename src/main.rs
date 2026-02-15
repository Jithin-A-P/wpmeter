use std::thread;
use std::time::Duration;

use rdev::{listen, Event, EventType};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::event::Event as TaoEvent;
use tray_icon::{TrayIconBuilder, menu::Menu, menu::MenuItem, menu::MenuEvent};

mod wpm;
use wpm::WpmCalculator;

#[derive(Debug)]
enum UserEvent {
    KeyPress,
    UpdateDisplay, // Periodic update to decay WPM when not typing
}

fn main() {
    // 1. Create the event loop
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // 2. Spawn the global input listener thread
    thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyPress(_) = event.event_type {
                let _ = proxy.send_event(UserEvent::KeyPress);
            }
        };

        if let Err(error) = listen(callback) {
            eprintln!("Error: {:?}", error);
        }
    });

    // 3. Setup Tray Icon
    let tray_menu = Menu::new();
    let quit_i = MenuItem::new("Quit", true, None);
    tray_menu.append(&quit_i).unwrap();

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("WPM Meter")
            .with_title("0 WPM") // Initial title
            .build()
            .unwrap(),
    );

    // 4. WPM State
    let mut wpm_calculator = WpmCalculator::new();

    // 5. Timer for periodic decay (if user stops typing, WPM should drop)
    
    // Using a separate thread for the ticker
    let proxy_clone = event_loop.create_proxy();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let _ = proxy_clone.send_event(UserEvent::UpdateDisplay);
        }
    });

    // 6. Run Event Loop
    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Handle Menu Events (Quit)
        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_i.id() {
                *control_flow = ControlFlow::Exit;
                tray_icon = None; // Drop tray icon
            }
        }

        match event {
            TaoEvent::UserEvent(user_event) => {
                match user_event {
                    UserEvent::KeyPress => {
                        wpm_calculator.add_keypress();
                    }
                    UserEvent::UpdateDisplay => {
                        // Just trigger a recalculation/update
                    }
                }
                
                // Update the tray icon
                let wpm = wpm_calculator.calculate_wpm();
                let text = format!("{:.0} WPM", wpm);
                
                if let Some(icon) = &mut tray_icon {
                    // On macOS, this sets the text next to the icon (or instead of it if no icon)
                    let _ = icon.set_title(Some(text.clone())); 
                    let _ = icon.set_tooltip(Some(format!("Current Speed: {}", text)));
                }
            }
            _ => (),
        }
    });
}


