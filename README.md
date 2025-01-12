# WinSafe

[![Crates.io](https://img.shields.io/crates/v/winsafe.svg)](https://crates.io/crates/winsafe)
[![Docs.rs](https://docs.rs/winsafe/badge.svg)](https://docs.rs/winsafe)
[![Lines of code](https://tokei.rs/b1/github/rodrigocfd/winsafe)](https://github.com/rodrigocfd/winsafe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Windows API and GUI in safe, idiomatic Rust.

WinSafe has:

* low-level Win32 API constants, functions and structs;
* high-level structs to build native Win32 GUI applications.

If you're looking for a comprehensive Win32 coverage, take a look at [winapi](https://crates.io/crates/winapi) or [windows](https://crates.io/crates/windows) crates, which are *unsafe*, but have everything.

WinSafe documentation:

| Branch | Docs |
| - | - |
| Stable | [docs.rs/winsafe](https://docs.rs/winsafe) |
| Nightly (master) | [rodrigocfd.github.io/winsafe/winsafe](https://rodrigocfd.github.io/winsafe/winsafe/) |

## Current status

These are the estimated progresses of the GUI features:

| GUI feature | Estimated progress |
| - | - |
| User window/dialogs (main, modal, modeless and control) | 100% |
| Native controls | 85% |

Plus, below are the numbers of native FFI items implemented:

| Native FFI item | Count |
| - | - |
| Functions | 668 |
| Structs | 179 |
| Constants | 12,554 |
| Window messages | 648 |
| Handles | 41 |
| COM interfaces | 44 |
| COM methods | 222 |

## Usage

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
winsafe = { version = "0.0.15", features = [] }
```

You can, alternatively, use the Nightly (master) branch [directly](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories), to get the latest features right away:

```toml
[dependencies]
winsafe = { git = "https://github.com/rodrigocfd/winsafe", features = [] }
```

Then you must enable the [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) you want to be included – these modules are named after native Windows DLL and library names, mostly.

The following Cargo features are available so far:

| Feature | Description |
| - | - |
| `comctl` | ComCtl32.dll, for [Common Controls](https://learn.microsoft.com/en-us/windows/win32/api/_controls/) |
| `comdlg` | ComDlg32.dll, for the old [Common Dialogs](https://learn.microsoft.com/en-us/windows/win32/uxguide/win-common-dlg) |
| `dshow` | [DirectShow](https://learn.microsoft.com/en-us/windows/win32/directshow/directshow) |
| `dxgi` | [DirectX Graphics Infrastructure](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dx-graphics-dxgi) |
| `gdi` | Gdi32.dll, the [Windows GDI](https://learn.microsoft.com/en-us/windows/win32/gdi/windows-gdi) |
| **`gui`** | **The WinSafe high-level GUI abstractions** |
| `kernel` | Kernel32.dll, Advapi32.dll and Ktmw32.dll – all others will include it |
| `msimg` | Msimg32.dll |
| `ole` | OLE and basic COM support |
| `oleaut` | [OLE Automation](https://learn.microsoft.com/en-us/windows/win32/api/_automat/) |
| `shell` | Shell32.dll and Shlwapi.dll, the COM-based [Windows Shell](https://learn.microsoft.com/en-us/windows/win32/shell/shell-entry) |
| `user` | User32.dll, the basic Windows GUI support |
| `uxtheme` | UxTheme.dll, extended window theming |
| `version` | Version.dll, to manipulate *.exe version info |

Note that a Cargo feature may depend on other features, which will be enabled automatically.


## Example

**Note:** You can find several examples in the dedicated repo: [github.com/rodrigocfd/winsafe-examples](https://github.com/rodrigocfd/winsafe-examples)

WinSafe allows you to create windows in two ways:

* programmatically defining parameters; or
* [loading dialogs](https://github.com/rodrigocfd/winsafe-examples/tree/master/03_dialog_resources) from a `.res` file created with a WYSIWYG resource editor.

The [example below](https://github.com/rodrigocfd/winsafe-examples/tree/master/01_button_click/) creates a window  with a button programmatically. Note how the click event is handled with a closure:

![Example 01](https://raw.githubusercontent.com/rodrigocfd/winsafe-examples/master/01_button_click/screen.gif)

```toml
[dependencies]
winsafe = { version = "0.0.15", features = ["gui"] }
```

```rust
#![windows_subsystem = "windows"]

use winsafe::prelude::*;
use winsafe::{gui, POINT, SIZE};

fn main() {
    let my = MyWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}


#[derive(Clone)]
pub struct MyWindow {
    wnd:       gui::WindowMain, // responsible for managing the window
    btn_hello: gui::Button,     // a button
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new( // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                size: (300, 150),
                ..Default::default() // leave all other options as default
            },
        );

        let btn_hello = gui::Button::new(
            &wnd, // the window manager is the parent of our button
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: (20, 20),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_hello };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        let wnd = self.wnd.clone(); // clone so it can be passed into the closure
        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!")?;
            Ok(())
        });
    }
}
```

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.md](LICENSE.md) for details.
