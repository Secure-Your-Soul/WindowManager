use input::event::keyboard::KeyboardEventTrait;
use smithay::{
    backend::{
        input::InputEvent,
        libinput::{LibinputInputBackend, LibinputSessionInterface},
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
    let display: Display<State> = Display::new()?;
    let _display_handle = display.handle();
    let mut state = State { running: true };

    let (session, session_notifier) = LibSeatSession::new()?;
    
    event_loop.handle().insert_source(session_notifier, |_, _, _| {
        // Obsługa zawieszenia / wznowienia sesji (np. przełączenie TTY)
    })?;

    let seat_name = session.seat().to_owned();
    let udev_backend = UdevBackend::new(&seat_name)?;

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

    // Inicjalizacja kontekstu libinput powiązanego z sesją libseat
    let mut libinput_context = input::Libinput::new_with_udev(LibinputSessionInterface::from(session.clone()));
    libinput_context
        .udev_assign_seat(&seat_name)
        .map_err(|_| "Nie udało się przypisać seat do libinput")?;
    
    let libinput_backend = LibinputInputBackend::new(libinput_context);

    // Obsługa zdarzeń wejściowych z paczki `input`
    event_loop.handle().insert_source(libinput_backend, move |event: InputEvent<LibinputInputBackend>, _, _state| {
        match event {
            InputEvent::Keyboard { event: keyboard_event, .. } => {
                // Używamy KeyState z biblioteki `input`
                if keyboard_event.key_state() == input::event::keyboard::KeyState::Pressed {
                    // Metoda .key() zwraca surowy kod skanu (u32 / u16)
                    let keycode = keyboard_event.key();
                    println!("Naciśnięto surowy keycode: {:?}", keycode);

                    // Klawisz 'R' w standardowym układzie evdev (keycode 33)
                    if keycode == 33 {
                        println!("--> Skrót aktywowany! Wciśnięto klawisz R!");
                    }
                }
            }
            _ => {}
        }
    })?;

    println!("Kompozytor DRM/Udev + Libinput zainicjalizowany pomyślnie!");

    while state.running {
        event_loop.dispatch(std::time::Duration::from_millis(16), &mut state)?;
    }

    Ok(())
}