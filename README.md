# Conic Launcher

[![status](https://img.shields.io/badge/status-stable-blue.svg)](https://github.com/tauri-apps/tauri/tree/dev)
[![Conic Launcher Auto Build](https://img.shields.io/github/actions/workflow/status/conic-apps/launcher/build.yml?label=build%20app&logo=github)](https://github.com/conic-apps/launcher/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/conic-apps/launcher.svg)](https://www.gnu.org/licenses/quick-guide-GPL-3.0.html)
![Contributors](https://img.shields.io/github/contributors/conic-apps/launcher?color=2green)
[![website](https://img.shields.io/badge/website-launcher.btlcraft.top-purple.svg)](https://launcher.btlcraft.top)
[![https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg](https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg)](https://good-labs.github.io/greater-good-affirmation)

[简体中文](./README.zh.md)

Please go to the [official website](https://amethyst.btlcraft.top) to download the launcher!

## Features

-   You can install `Minecraft`, `Forge`, `Fabric`, `Optifine`, `Quilt`
-   **Cross-platform**, Launcher written in rust that runs on Windows, MacOS, and GNU/Linux
-   **Cosmic unbeatable download speed**, It even takes only 15 seconds to install the game on a system with the Linux kernel
-   **Instance management**, Supports grouping and other functions to easily manage multiple instances
-   **Mod download**, You can download mods from Curseforge, Modrinth, FTB directly from the launcher

## Advantages

-   **Save storage space**, the same material packs enabled in multiple instances are only stored once
-   **Automatically test for modules that will not start**, you can automatically test which modules will cause crashes when the game starts
-   **Customize the appearance of the launcher**, every inch of the launcher can be customized
-   **Modify the game rules of the archive in the launcher**, no need to enter commands in the game
-   **Custom download source**, you can use self-built mirror source without compiling it yourself

## License

The software is distributed under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html) with additional terms.

### Additional terms under GPLv3 Section 7

1. When you distribute a modified version of the software, you must change the software name or the version number in a reasonable way in order to distinguish it from the original version. (Under [GPL-3.0, 7(c)](./LICENSE#L372-L374))

You need to find all the words related to the name of this program in the source code and replace them with the name of your own program

2. You must not remove the copyright declaration displayed in the software. (Under [GPL-3.0, 7(b)](./LICENSE#L368-L370))

3. If someone signs something of a contractual nature with the recipient and provides a commitment of liability, the licensor and author are not subject to this liability jointly and severally. (Under [GPL-3.0, 7(b)](./LICENSE#L382-L386))

## Manual build

Make sure you have completed [prep](https://v2.tauri.app/start/prerequisites) first.

Execute the following command to pull the code:

```bash
git clone https://github.com/conic-apps/launcher.git
cd launcher
```

Execute the following command to install the dependencies (in the project folder):

```bash
yarn install
```

Next, simply run the following command to complete the build:

```bash
yarn tauri build
```

If you want to contribute to this project, run `yarn tauri dev` to debug the application and see the details at [here](https://tauri.app/zh-cn/v1/guides/).
