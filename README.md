# Arma3 Mod Manager CLI

<p align="center">
  <img src="https://github.com/user-attachments/assets/f5f58180-e5f4-4442-a448-c60f81df907d" alt="animated" />
</p>

## Overview

Arma3 Mod Manager CLI is a command-line tool designed to simplify the process of enabling and disabling mods for Arma 3. Since there is no official Arma 3 Launcher port available for Linux / macOS.

This have been tested on Apple Silicon M1.

## Installation

### Prerequisites

- Ensure you have Arma 3 installed through Steam.
- Ensure having Rust and Cargo installed if you are building from source.

### Steps

* Downloading the latest Pre-Built UNIX executable
  
  [Releases](https://github.com/viktorholk/arma3-mod-manager-cli/releases)

  <details><summary>MacOS</summary>

  On MacOS, you may be greeted with a security warning.
  Go to Settings > Privary & Security > Security
  and press Open Anyway

  ![image](https://github.com/user-attachments/assets/966592ac-b40a-439e-b793-70fc42070ccd)


  ![image](https://github.com/user-attachments/assets/6d58efce-6dff-41f9-b790-7839c2a15a36)


  </details>

* Or you can build from Source
  
  ````
  git clone git@github.com:viktorholk/arma3-mod-manager-cli.git
  cd arma3-mod-manager-cli
  cargo run
  ````


### CLI Troubleshooting Guide

**Issue**: Running the CLI gives an error: 

`Error: InvalidPath("/Users/user/Library/Application Support/Steam/steamapps/workshop/content/107410")`

**Steps to Resolve**:
1. **Check Config File**: Verify `~/arma3-mod-manager-cli-config.json` has the correct Steam path.
2. **Ensure Workshop Mods**: Confirm Arma 3 workshop mods are installed via Steam.
3. **Locate Steam Path**:
   - Right-click *Arma 3* in Steam > *Manage* > *Browse local files*.
   - Copy a file path (`COMMAND + C`) and paste (`COMMAND + V`) into the terminal to confirm it matches your config.

**Adjust and test** the paths, then rerun the CLI.




## Issues
Please report issues using [GitHub's issues tab](https://github.com/viktorholk/script-interactor/issues).

## License
Arma 3 Mod Manager CLI is under the [MIT LICENSE](LICENSE).
