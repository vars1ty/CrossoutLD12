use crate::cli_tools::CLITools;

/// Runner responsible for patching and starting the game.
pub struct Runner {
    /// Bad executables to remove, if present.
    bad_executables: [&'static str; 3],
}

impl Runner {
    /// Constructs a new instance to `Runner`.
    pub fn new() -> Self {
        Self {
            bad_executables: ["gaijin_downloader.exe", "bpreport.exe", "gjagent.exe"],
        }
    }

    /// Crashes if the path is invalid.
    fn validate_path(&self) {
        if !std::path::Path::new("./CrossoutLauncher.exe").exists() {
            panic!("[ERROR] Invalid path, place the executable in the same directory as the \"CrossoutLauncher.exe\" file!");
        }
    }

    /// Removes bad executable files, if any, from the current directory.
    fn remove_bad_executables(&self) {
        self.validate_path();
        let files = std::fs::read_dir(".").expect("[ERROR] Failed reading the current directory!");
        for file in files {
            let file = file.expect("[ERROR] File couldn't be processed!");
            let name = file.file_name();
            let name = name.to_string_lossy();
            for bad_executable in self.bad_executables {
                if name == bad_executable {
                    // Bad, remove it.
                    std::fs::remove_file(file.path()).expect("[ERROR] Failed removing file!");
                    println!("[i] Removed bad executable: {name}");
                }
            }
        }
    }

    /// Patches the Easy AntiCheat configuration files to start with DirectX 12.
    fn patch_eac_config(&self) {
        // Remove the initial x64 config, as its DX11.
        std::fs::remove_file("./EasyAntiCheat/Launcher/Settings64.json")
            .expect("[ERROR] Failed removing Settings64.json!");
        // Rename the D3D12 one to 64, so its used at startup, forcing the use of DX12.
        std::fs::rename(
            "./EasyAntiCheat/Launcher/SettingsD3D12.json",
            "./EasyAntiCheat/Launcher/Settings64.json",
        )
        .expect("[ERROR] Failed renaming SettingsD3D12.json!");
    }

    /// Runs the game launcher and finalizes everything.
    pub fn patch_game(&self) {
        CLITools::print_pause("[i] Open the Crossout Launcher and wait for the \"Play\" button to become interactive, then hit any key in this window to begin!");
        self.remove_bad_executables();
        self.patch_eac_config();
        CLITools::print_pause("[i] Game should now be patched, press Play and hit any key to exit.")
    }
}
