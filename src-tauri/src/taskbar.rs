//! Icona a la safata amb menú, i botons anterior / reprodueix / següent a la miniatura de la
//! barra de tasques de Windows. Tots dos envien el mateix event `media` que les tecles multimèdia.

use std::sync::atomic::{AtomicBool, Ordering};

use serde_json::json;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Wry,
};

const TRAY: &str = "ona";
static PLAYING: AtomicBool = AtomicBool::new(false);

/// L'entrada «Reprodueix / Pausa» del menú, per canviar-ne el text.
pub struct Toggle(MenuItem<Wry>);

pub fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn media(app: &AppHandle, action: &str) {
    let _ = app.emit("media", json!({ "action": action }));
}

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    let prev = MenuItem::with_id(app, "prev", "Anterior", true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "Reprodueix", true, None::<&str>)?;
    let next = MenuItem::with_id(app, "next", "Següent", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "Mostra Ona", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Surt", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&prev, &toggle, &next, &sep, &show, &quit])?;
    let mut tray = TrayIconBuilder::with_id(TRAY)
        .tooltip("Ona")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, e| match e.id().as_ref() {
            "show" => show_main(app),
            "quit" => app.exit(0),
            a => media(app, a),
        })
        .on_tray_icon_event(|tray, e| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                show_main(tray.app_handle());
            }
        });
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    app.manage(Toggle(toggle));
    #[cfg(windows)]
    thumb::init(app);
    Ok(())
}

/// Cançó actual al rètol de la safata (Windows el talla a 127 caràcters).
pub fn set_track(app: &AppHandle, title: Option<&str>, artist: Option<&str>) {
    let tip = match (title, artist) {
        (Some(t), Some(a)) => format!("{t} · {a}"),
        (Some(t), None) => t.to_string(),
        _ => "Ona".into(),
    };
    let tip: String = tip.chars().take(120).collect();
    if let Some(tray) = app.tray_by_id(TRAY) {
        let _ = tray.set_tooltip(Some(tip));
    }
}

/// Canvia «Reprodueix»/«Pausa» al menú i a la miniatura, només quan l'estat canvia.
pub fn set_playing(app: &AppHandle, playing: bool) {
    if PLAYING.swap(playing, Ordering::Relaxed) == playing {
        return;
    }
    if let Some(t) = app.try_state::<Toggle>() {
        let _ = t.0.set_text(if playing { "Pausa" } else { "Reprodueix" });
    }
    #[cfg(windows)]
    {
        let _ = app.run_on_main_thread(move || thumb::update(playing));
    }
}

#[cfg(windows)]
mod thumb {
    use std::{cell::RefCell, sync::OnceLock};

    use tauri::{AppHandle, Manager};
    use windows::{
        core::w,
        Win32::{
            Foundation::{HWND, LPARAM, LRESULT, WPARAM},
            System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
            UI::{
                Shell::{
                    DefSubclassProc, ITaskbarList3, SetWindowSubclass, TaskbarList, THBF_ENABLED, THB_FLAGS,
                    THB_ICON, THB_TOOLTIP, THUMBBUTTON,
                },
                WindowsAndMessaging::{CreateIcon, GetSystemMetrics, RegisterWindowMessageW, HICON, SM_CXSMICON, WM_COMMAND},
            },
        },
    };

    const THBN_CLICKED: usize = 0x1800;
    const IDS: [&str; 3] = ["prev", "toggle", "next"];
    static APP: OnceLock<AppHandle> = OnceLock::new();

    struct Bar {
        list: ITaskbarList3,
        hwnd: HWND,
        icons: [HICON; 4], // anterior, reprodueix, següent, pausa
        added: bool,
    }

    thread_local! {
        // Tot es fa al fil de la finestra (COM d'apartament).
        static BAR: RefCell<Option<Bar>> = const { RefCell::new(None) };
    }

    /// Pinta un glif blanc antialiasat de `s`×`s` píxels (sense fitxers d'icona).
    fn glyph(s: i32, inside: impl Fn(f32, f32) -> bool) -> HICON {
        const SS: i32 = 4;
        let mut px = vec![0u8; (s * s * 4) as usize];
        for y in 0..s {
            for x in 0..s {
                let mut hits = 0;
                for sy in 0..SS {
                    for sx in 0..SS {
                        let u = (x as f32 + (sx as f32 + 0.5) / SS as f32) / s as f32;
                        let v = (y as f32 + (sy as f32 + 0.5) / SS as f32) / s as f32;
                        hits += inside(u, v) as i32;
                    }
                }
                let i = ((y * s + x) * 4) as usize;
                px[i..i + 4].copy_from_slice(&[255, 255, 255, (hits * 255 / (SS * SS)) as u8]);
            }
        }
        let mask = vec![0u8; (((s + 15) / 16 * 2) * s) as usize]; // 1 bit per píxel, files de 16 bits
        unsafe { CreateIcon(None, s, s, 1, 32, mask.as_ptr(), px.as_ptr()).unwrap_or_default() }
    }

    /// Triangle que apunta a la dreta entre x0 i x1, de 0,2 a 0,8 d'alçada.
    fn tri(u: f32, v: f32, x0: f32, x1: f32) -> bool {
        u >= x0 && u <= x1 && (v - 0.5).abs() <= 0.3 * (x1 - u) / (x1 - x0)
    }
    fn bar(u: f32, v: f32, x0: f32, x1: f32) -> bool {
        u >= x0 && u <= x1 && (0.2..=0.8).contains(&v)
    }

    fn icons() -> [HICON; 4] {
        let s = unsafe { GetSystemMetrics(SM_CXSMICON) }.max(16);
        let next = |u: f32, v: f32| tri(u, v, 0.22, 0.66) || bar(u, v, 0.68, 0.8);
        [
            glyph(s, move |u, v| next(1.0 - u, v)),
            glyph(s, |u, v| tri(u, v, 0.3, 0.82)),
            glyph(s, next),
            glyph(s, |u, v| bar(u, v, 0.26, 0.42) || bar(u, v, 0.58, 0.74)),
        ]
    }

    fn button(id: u32, icon: HICON, tip: &str) -> THUMBBUTTON {
        let mut b = THUMBBUTTON {
            dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
            iId: id,
            hIcon: icon,
            dwFlags: THBF_ENABLED,
            ..Default::default()
        };
        for (d, c) in b.szTip.iter_mut().zip(tip.encode_utf16()) {
            *d = c;
        }
        b
    }

    fn play_button(bar: &Bar, playing: bool) -> THUMBBUTTON {
        if playing { button(1, bar.icons[3], "Pausa") } else { button(1, bar.icons[1], "Reprodueix") }
    }

    fn add() {
        BAR.with(|b| {
            let mut b = b.borrow_mut();
            let Some(bar) = b.as_mut() else { return };
            let playing = super::PLAYING.load(std::sync::atomic::Ordering::Relaxed);
            let buttons = [button(0, bar.icons[0], "Anterior"), play_button(bar, playing), button(2, bar.icons[2], "Següent")];
            // Si el botó de la barra encara no existeix, falla i es torna a provar amb TaskbarButtonCreated.
            // Es torna a provar a cada TaskbarButtonCreated (p. ex. si l'Explorador es reinicia);
            // si ja hi eren, falla i es queden com estaven.
            let ok = unsafe { bar.list.ThumbBarAddButtons(bar.hwnd, &buttons) }.is_ok();
            if ok {
                log::info!("botons de la miniatura afegits");
            }
            bar.added |= ok;
        });
    }

    pub fn update(playing: bool) {
        BAR.with(|b| {
            let b = b.borrow();
            let Some(bar) = b.as_ref().filter(|b| b.added) else { return };
            let _ = unsafe { bar.list.ThumbBarUpdateButtons(bar.hwnd, &[play_button(bar, playing)]) };
        });
    }

    fn created() -> u32 {
        static MSG: OnceLock<u32> = OnceLock::new();
        *MSG.get_or_init(|| unsafe { RegisterWindowMessageW(w!("TaskbarButtonCreated")) })
    }

    unsafe extern "system" fn proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM, _: usize, _: usize) -> LRESULT {
        if msg == created() {
            add();
        } else if msg == WM_COMMAND && wp.0 >> 16 == THBN_CLICKED {
            if let (Some(app), Some(a)) = (APP.get(), IDS.get(wp.0 & 0xffff)) {
                super::media(app, a);
                return LRESULT(0);
            }
        }
        DefSubclassProc(hwnd, msg, wp, lp)
    }

    pub fn init(app: &AppHandle) {
        let Some(hwnd) = app.get_webview_window("main").and_then(|w| w.hwnd().ok()) else { return };
        let hwnd = HWND(hwnd.0);
        let list: ITaskbarList3 = match unsafe { CoCreateInstance(&TaskbarList, None, CLSCTX_INPROC_SERVER) } {
            Ok(l) => l,
            Err(e) => return log::warn!("barra de tasques: {e}"),
        };
        if unsafe { list.HrInit() }.is_err() {
            return;
        }
        let _ = APP.set(app.clone());
        created();
        BAR.with(|b| *b.borrow_mut() = Some(Bar { list, hwnd, icons: icons(), added: false }));
        unsafe {
            let _ = SetWindowSubclass(hwnd, Some(proc), 1, 0);
        }
        add(); // per si el botó de la barra ja existia abans de subclassar
    }
}
