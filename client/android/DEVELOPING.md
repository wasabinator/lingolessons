## Developing LingoLessons Android client

This application is still in development and not ready for deployment.

### Requirements

- Android Studio and/or Android Build Tools.
- Rust and Cargo, which can be installed via rustup (See: https://rustup.rs/).

### Via NIX/Direnv

- An environment is already setup which will create a direnv shell if run via Nix (eg. NixOS/NixWSL/NixDarwin).
- Simply cd into lingolessons/client and the environment will be setup, ready to go.
- To create an initial AVD for testing, use the `create-avd` command.
- To launch the avd, use the `avd` command.
- On first import of project to Android Studio, cancel the setup wizard, then import the project. When prompted for the Android SDK location, use the ANDROID_HOME env variable as defined by the direnv shell (which will be a `/nix/store/...` path).
- All SDK components are thus managed via changes to `android/default.nix`.

### Running the App

Currently only debug builds are configured.

Either launch via Android studio in an emulator; or

- Launch an emulator
- Run the command
```
./gradlew installDebug
```

The build is currently configured to reach the server at 10.0.2.2:8000, which corresponds with localhost:8000.
