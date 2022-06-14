use std::sync::{Mutex, Arc};

use miette::IntoDiagnostic;
use miette::Diagnostic;
use thiserror::Error;
use winsafe::co::PROCESS;
use winsafe::prelude::{KernelHprocess, *};
use winsafe::{EnumWindows, HPROCESS};


#[derive(Debug, Error, Diagnostic)]
pub enum SpotifyError {
    #[error("Spotify is closed")]
    SpotifyClosed,

    #[error("Spotify seems to be paused right now")]
    SpotifyPaused,
}

pub fn get_currently_playing_song_info() -> Result<(String, String), miette::Error> {
    let windows = find_spotify_and_get_windows_name()?;
    if windows.is_empty() {
        return Err(SpotifyError::SpotifyClosed.into());
    }
    if windows[0].starts_with("Spotify") {
        return Err(SpotifyError::SpotifyPaused.into());
    }

    #[rustfmt::skip]
    let (artist, track) =
     if let Some((artist, track)) = windows[0].split_once(" - ") {
        (artist.to_owned(), track.to_owned())
    } else {
        let (artist, track) = ("".to_owned(), windows[0].clone());
        (artist, track)
    };
    Ok((artist, track))
}

fn find_spotify_and_get_windows_name() -> miette::Result<Vec<String>> {
    let windows = Arc::new(Mutex::new(Vec::new()));
    EnumWindows(|hwnd| {
        let window_text = hwnd.GetWindowText();
        let classname = hwnd.GetClassName();

        if let (Ok(w_text), Ok(cls_name)) = (window_text, classname) {
            if cls_name == "Chrome_WidgetWin_0" && w_text.chars().count() > 0 {
                let (_, proc_id) = hwnd.GetWindowThreadProcessId();

                #[rustfmt::skip]
                let proc_hwnd =
                    HPROCESS::OpenProcess(
                         PROCESS::QUERY_LIMITED_INFORMATION,
                         false, 
                         proc_id)
                        .expect("couldn't get process handle");

                let path = proc_hwnd
                    .QueryFullProcessImageName(0.into())
                    .expect("couldn't get process name");

                proc_hwnd.CloseHandle().expect("couldn't close process");

                if path.split('\\').last().unwrap() == "Spotify.exe" {
                    windows.lock().unwrap().push(w_text);
                }
            }
        }
        true
    })
    .into_diagnostic()?;

    let windows = Ok(windows.lock().unwrap().to_vec());
    windows
}
