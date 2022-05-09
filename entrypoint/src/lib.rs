pub fn attach_console() {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::Console::AttachConsole;
        unsafe {
            AttachConsole(u32::MAX).as_bool();
        }
    }
}
