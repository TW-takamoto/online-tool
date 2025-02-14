use std::{thread, time::Duration};
use chrono::Local;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    KEYEVENTF_KEYUP, INPUT, KEYBDINPUT, VK_CONTROL,
    INPUT_KEYBOARD, SendInput, INPUT_0
};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    style::{Color, SetForegroundColor},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prefix = "Running... :";
    
    // ターミナルをクリアして色を設定
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        SetForegroundColor(Color::Yellow)
    )?;
    
    print!("{}", prefix);
    
    loop {
        // Ctrl キーを送信
        send_ctrl_key()?;
        
        // 現在時刻を表示
        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::White)
        )?;
        
        let datetime = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();
        print!("{}", datetime);
        
        // カーソルを移動
        execute!(
            std::io::stdout(),
            MoveTo(prefix.len() as u16, 0)
        )?;
        
        // 60秒待機
        thread::sleep(Duration::from_secs(60));
    }
}

fn send_ctrl_key() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: unsafe {
            let mut anonymous: INPUT_0 = std::mem::zeroed();
            anonymous.ki = KEYBDINPUT {
                wVk: VK_CONTROL,
                wScan: 0,
                dwFlags: 0,
                time: 0,
                dwExtraInfo: 0,
            };
            anonymous
        }
    };
    
    unsafe {
        SendInput(1, &input as *const INPUT, std::mem::size_of::<INPUT>() as i32);
        
        (*(&mut input.Anonymous as *mut INPUT_0)).ki.dwFlags = KEYEVENTF_KEYUP;
        SendInput(1, &input as *const INPUT, std::mem::size_of::<INPUT>() as i32);
    }
    
    Ok(())
}
