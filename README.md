[![Django CI](https://github.com/wasabinator/lingolessons/actions/workflows/django.yml/badge.svg)](https://github.com/wasabinator/lingolessons/actions/workflows/django.yml)
[![Rust CI](https://github.com/wasabinator/lingolessons/actions/workflows/rust.yml/badge.svg)](https://github.com/wasabinator/lingolessons/actions/workflows/rust.yml)
[![Android CI](https://github.com/wasabinator/lingolessons/actions/workflows/android.yml/badge.svg)](https://github.com/wasabinator/lingolessons/actions/workflows/android.yml)

# LingoLessons
Language education system

This is a multiplatform system, which is designed for you to self host via docker. It is still in active development and not yet in a state ready for use.

It is comprised of the following platform components. The goal is for each target to be natively compiled into their respective executable format without external dependencies, utilising the native ui controls of each platform so that they have a native look & feel. Kotlin Multiplatform was first considered, but the results did not look native outside of Android, and also I was not comfortable with the fact that the desktop targets all required a JVM:

### Server (Python Django)

### All Clients: Shared Domain + Data Library (cross platform Rust)
 
### Android (Kotlin + Jetpack Compose)

### macOS & iOS (SwiftUI)

### Linux (Rust + GTK)

### Windows (Rust + Win32 API)
