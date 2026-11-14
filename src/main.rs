use smithay::{
    backend::{
        session::{Session, libseat::LibSeatSession},
        udev::{UdevBackend, UdevEvent},
    },
    reexports::{
        calloop::EventLoop,
        wayland_server::Display,
    },
};

struct State {
    running: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop = EventLoop::try_new()?;
    // Usunięto niepotrzebne 'mut'
    let display: Display<State> = Display::new()?;
    let _display_handle = display.handle();
    let mut state = State { running: true };

    // Usunięto niepotrzebne 'mut' przy session
    let (session, session_notifier) = LibSeatSession::new()?;
    
    event_loop.handle().insert_source(session_notifier, |_, _, _| {
        // Obsługa zawieszenia / wznowienia sesji
    })?;

    let seat_name = session.seat().to_owned();
    // Usunięto niepotrzebne 'mut' przy udev_backend
    let udev_backend = UdevBackend::new(&seat_name)?;

    // Zmieniono device_id na _device_id, ponieważ nie jest używane w pętli
    for (_device_id, path) in udev_backend.device_list() {
        println!("Znaleziono urządzenie udev pod ścieżką: {:?}", path);
    }

    event_loop.handle().insert_source(udev_backend, move |event, _, _state| {
        match event {
            UdevEvent::Added { device_id: _, path } => {
                println!("Podłączono nowe urządzenie: {:?}", path);
            }
            UdevEvent::Removed { device_id: _ } => {
                println!("Odłączono urządzenie.");
            }
            _ => {}
        }
    })?;

    println!("Kompozytor DRM/Udev zainicjalizowany!");

    while state.running {
        event_loop.dispatch(std::time::Duration::from_millis(16), &mut state)?;
    }

    Ok(())
}