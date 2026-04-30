use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WtProfile {
    pub status: &'static str,
    pub nick: Option<String>,
    pub level: Option<String>,
    pub registration_date: Option<String>,
    pub avatar_url: Option<String>,
    pub msg: Option<String>,
}

#[tauri::command]
pub fn fetch_wt_profile(_nick: String) -> WtProfile {
    WtProfile {
        status: "error",
        nick: None,
        level: None,
        registration_date: None,
        avatar_url: None,
        msg: Some("Use scrape_wt_profile".to_string()),
    }
}

#[tauri::command]
pub async fn scrape_wt_profile(app: tauri::AppHandle, nick: String) -> Result<(), String> {
    use tauri::{Emitter, Manager, WebviewWindowBuilder};

    if let Some(old_win) = app.get_webview_window("wt-scraper") {
        let _ = old_win.close();
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    let safe_nick = nick.replace(" ", "%20");
    let url = format!(
        "https://warthunder.com/en/community/userinfo/?nick={}",
        safe_nick
    );

    let win_builder = WebviewWindowBuilder::new(
        &app,
        "wt-scraper",
        tauri::WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?),
    )
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
    .title("ThunderStrike - Profiler")
    .visible(false)
    .inner_size(800.0, 600.0);

    let app_clone = app.clone();
    let win = win_builder
        .on_navigation(move |url| {
            let url_str = url.as_str();
            if url_str.starts_with("https://tauri.scrape.result/") {
                for (key, val) in url.query_pairs() {
                    let json_str = if key.is_empty() {
                        val.to_string()
                    } else {
                        key.to_string()
                    };
                    if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        let _ = app_clone.emit("wt-profile-result", json_data);
                        if let Some(w) = app_clone.get_webview_window("wt-scraper") {
                            let _ = w.close();
                        }
                    }
                    break;
                }
                return false;
            } else if url_str.starts_with("https://tauri.scrape.action/show_window") {
                if let Some(w) = app_clone.get_webview_window("wt-scraper") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
                return false;
            } else if url_str.starts_with("https://tauri.scrape.log/") {
                return false;
            }
            true
        })
        .build()
        .map_err(|e| format!("Failed to create webview: {}", e))?;

    let win_clone = win.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(3000));
        let scrape_loop = r#"
            (() => {
                const getTableData = (selector) => {
                    const list = document.querySelector(selector);
                    if (!list) return null;
                    const items = Array.from(list.querySelectorAll('.user-stat__list-item')).map(el => el.textContent.trim());
                    return {
                        wins: items[1] || '0',
                        missions: items[2] || '0',
                        ratio: items[3] || '0%',
                        deaths: items[4] || '0',
                        lions: items[5] || '0',
                        time: items[6] || '0',
                        air: items[7] || '0',
                        ground: items[8] || '0',
                        naval: items[9] || '0'
                    };
                };

                const checkAuth = setInterval(() => {
                    const html = document.body.innerHTML.toLowerCase();
                    if (window.location.href.includes("login.gaijin.net") || document.querySelector("iframe[src*='recaptcha']") || html.includes("captcha") || html.includes("please complete the security check")) {
                        window.location.href = "https://tauri.scrape.action/show_window";
                        clearInterval(checkAuth);
                    }
                }, 1000);

                const check = setInterval(() => {
                    const nickEl = document.querySelector('.user-profile__data-nick, .user-profile__nick');
                    if (nickEl) {
                        const nick = nickEl.textContent?.trim() ?? '';
                        let lvl = "1";
                        document.querySelectorAll('.user-profile__data-item').forEach(el => {
                            const t = el.textContent?.trim() ?? '';
                            if (t.includes("Level")) lvl = t.replace("Level", "").trim();
                        });
                        const avatar = document.querySelector('.user-profile__ava img')?.src ?? '';
                        let date = '';
                        const dateEl = document.querySelector('.user-profile__data-regdate');
                        if (dateEl) date = dateEl.textContent?.trim().replace("Registration date", "").trim() ?? '';
                        
                        const stats = {
                            arcade: getTableData('.user-stat__list.arcadeFightTab'),
                            realistic: getTableData('.user-stat__list.historyFightTab'),
                            simulator: getTableData('.user-stat__list.simulationFightTab')
                        };

                        const data = { nick, lvl, avatar, date, stats };
                        clearInterval(check);
                        window.location.href = "https://tauri.scrape.result/?" + encodeURIComponent(JSON.stringify(data));
                    }
                }, 1500);
            })()
        "#;
        let _ = win_clone.eval(scrape_loop);
        std::thread::sleep(std::time::Duration::from_millis(60000));
        let _ = win_clone.close();
    });

    Ok(())
}

#[tauri::command]
pub async fn launch_wt(path: String, steam_username: String) -> Result<(), String> {
    let mut game_root = std::path::PathBuf::from(&path);
    if game_root.ends_with("lang") {
        if let Some(parent) = game_root.parent() {
            game_root = parent.to_path_buf();
        }
    }

    let is_steam = !steam_username.trim().is_empty();

    if is_steam {
        std::process::Command::new("cmd")
            .args(["/C", "start", "steam://rungameid/236390"])
            .spawn()
            .map_err(|e| format!("Failed to launch Steam: {}", e))?;
            
        std::thread::sleep(std::time::Duration::from_secs(15));
    } else {
        let launcher_path = game_root.join("launcher.exe");
        
        if !launcher_path.exists() {
            return Err("launcher.exe not found".to_string());
        }

        let mut child = std::process::Command::new(&launcher_path)
            .current_dir(&game_root)
            .spawn()
            .map_err(|e| format!("Failed to launch launcher.exe: {}", e))?;

        let _ = child.wait();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let tasklist = std::process::Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq aces.exe")
            .output();

        if let Ok(out) = tasklist {
            let out_str = String::from_utf8_lossy(&out.stdout).to_lowercase();
            if !out_str.contains("aces.exe") {
                break;
            }
        } else {
            break;
        }
    }

    Ok(())
}
