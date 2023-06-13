use crate::cli_tools::CLITools;
use json::JsonValue;

/// Runner responsible for patching and starting the game.
pub struct Runner {
    /// Cached config.
    config: JsonValue,

    /// Bad executables to remove, if present.
    bad_executables: [&'static str; 3],
}

impl Runner {
    /// Constructs a new instance to `Runner`.
    pub fn new() -> Self {
        Self {
            config: json::parse(
                &std::fs::read_to_string("./config.json").expect("[ERROR] Missing config.json!"),
            )
            .expect("[ERROR] Failed parsing JSON!"),
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

    /// Installs VKD3D using vkd3d-proton.
    fn install_vkd3d(&self) {
        std::process::Command::new("setup_vkd3d_proton")
            .arg("install")
            .output() // Need blocking here.
            .expect("[ERROR] Failed launching setup_vkd3d_proton with 'install' argument!");
    }

    /// Gets the user-defined Easy AntiCheat runtime path.
    fn get_eac_runtime_path(&self) -> &str {
        self.config["easy_ac_runtime"]
            .as_str()
            .expect("[ERROR] Unspecified easy_ac_runtime, specify the path in your config!")
    }

    /// Gets the specified game window resolution.
    fn get_window_resolution(&self) -> &str {
        self.config["game_wnd_resolution"]
            .as_str()
            .unwrap_or("1280x800")
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
    pub fn run_game_launcher(&self) {
        // Launch the game through wine in a forced custom window resolution.
        std::process::Command::new("wine")
            .env("PROTON_EAC_RUNTIME", self.get_eac_runtime_path())
            .args([
                "explorer",
                &format!("/desktop=Crossout,{}", self.get_window_resolution()),
                "launcher.exe",
            ])
            .spawn()
            .expect("[ERROR] Failed launching launcher.exe!");

        CLITools::print_pause("[i] Opening game launcher, please wait until you're able to hit Play before pressing any key!");
        self.remove_bad_executables();
        self.install_vkd3d();
        self.patch_eac_config();
        CLITools::print_pause("[i] Game should now be patched, press Play and hit any key to exit.")
    }
}
