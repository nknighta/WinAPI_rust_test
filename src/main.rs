use windows::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {

    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(None, s!("Ansi"), s!("v"), MB_OK);
    }
    Ok(())
}