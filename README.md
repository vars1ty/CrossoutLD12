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
11 **isn't** an issue for most people. But if you're experiencing random crashes from **stack smashing detected!**, then this might just help solve those issues.

It might also improve performance, although I haven't done any performance tests to validate/invalidate those claims.

## Requirements
Dependencies listed are from the AUR. If you are on a different distro, you'll have to find/compile them on your own.

- A GPU that supports the descriptors used by Vulkan for translating DirectX 12. See more at [vk3d-proton's GitHub](https://github.com/HansKristian-Work/vkd3d-proton).
   - In general, most (if not all) GPUs should support it, unless you're on some ancient T-Rex GPU.
- A Crossout install already setup
- vkd3d-proton-bin
   - Needed for easy VKD3D
- wine-ge-custom
   - Haven't tested if normal wine works.
   - Alternatively if you don't want to compile it, use ProtonUp and setup so that wine-ge is the default when running the `wine` command, or modify the source.

## Usage
[TODO]
