# CrossoutLD12
## What is it?
The name is pretty self-explanatory, but here's a full list of the tweaks offered in this small CLI Program:
- DirectX 12 is default, no more DirectX 11
   - Patched by forcing Easy AntiCheat to use the DirectX 12 configuration
- Easy AntiCheat Runtime is forced
   - Simply pass the directory to the runtime directory inside of the configuration JSON and it should work
- Removal of Gaijin and Targem's broken tray and agent applications
   - This is because at times, they may hinder a proper, clean startup from happening.

## But, why DirectX 12? Why's 11 an issue?
11 **isn't** an issue for most people. But if you're experiencing random crashes, then this might just help solve those issues.

It might also improve performance, although I haven't done any performance tests to validate/invalidate those claims.

## Requirements
There are no special requirements, there are also zero dependencies as it's just a simple CLI-Patcher.

The only requirements are:

- A GPU that supports the descriptors used by Vulkan for translating DirectX 12. See more at [vkd3d-proton's GitHub](https://github.com/HansKristian-Work/vkd3d-proton).
   - In general, most (if not all) GPUs should support it, unless you're on some ancient T-Rex GPU.
- A Crossout install already setup
- You to follow the on-screen messages.

## Usage
1. Place the executable inside the Crossout game directory, in my case its at `/home/unmapped/.var/app/com.usebottles.bottles/data/bottles/bottles/XO/drive_c/users/unmapped/AppData/Local/Crossout/`
   - Ensure the directory has the files `CrossoutLauncher.exe` and `launcher.exe` in it, alongside with a `EasyAntiCheat` folder.
2. Run `chmod +x crossoutld12`
3. Open the Crossout Launcher and wait for the "Play" button to become interactive.
4. Run `./crossoutld12` and follow the instructions on-screen.
5. Wait for it to process and you're done, you may now hit "Play" in the launcher!
