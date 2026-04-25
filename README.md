# itf_patterns_rs
For all the ITF Taekwon-Do enthusiasts, this is a desktop app for randomly generating patterns for a given degree.

Built using the [Dioxus](https://github.com/DioxusLabs/dioxus) framework.

# Installation
Download the desktop app from [releases](https://github.com/OscarAspelin95/itf_patterns_rs/releases)

# Development
## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust 2021 edition
- Dioxus cli == 0.7.2

## Usage
`dx serve`


##
```bash
export ANDROID_HOME="/home/$USER/Android/Sdk" \
&& export ANDROID_NDK_HOME="/home/$USER/Android/Sdk/ndk/30.0.14904198" \
&& export JAVA_HOME="/home/$USER/android_studio/android-studio/jbr" \
&& dx build --platform android --release --target aarch64-linux-android \
&& cp ./target/dx/itf_patterns_rs/release/android/app/app/build/outputs/apk/debug/app-debug.apk itf_patterns.apk
```

![play_gif](https://github.com/OscarAspelin95/itf_patterns_rs/blob/main/assets/itf_patterns.gif)
