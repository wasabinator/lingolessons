## Developing LingoLessons Android client

This application is still in development and not ready for deployment.

### Requirements

- Android Studio and/or Android Build Tools.
- Rust and Cargo, which can be installed via rustup (See: https://rustup.rs/).

### Running the App

Currently only debug builds are configured.

Either launch via Android studio in an emulator; or

- Launch an emulator
- Run the command
```
./gradlew installDebug
```

The build is currently configured to reach the server at 10.0.2.2:8000, which corresponds with localhost:8000.
