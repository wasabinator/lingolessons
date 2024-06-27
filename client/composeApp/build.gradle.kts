import de.jensklingenberg.ktorfit.gradle.ErrorCheckingMode
import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidApplication)
    alias(libs.plugins.jetbrainsCompose)
    alias(libs.plugins.compose.compiler)
    alias(libs.plugins.ksp)
    alias(libs.plugins.ktorfit)
    alias(libs.plugins.serialization)
    alias(libs.plugins.osdetector)
    alias(libs.plugins.sqldelight)
    alias(libs.plugins.native.cocoapods)
}

ktorfit {
    errorCheckingMode = ErrorCheckingMode.ERROR
    generateQualifiedTypeName = false
}

kotlin {
    sourceSets.all {
        languageSettings.optIn("kotlinx.cinterop.ExperimentalForeignApi")
    }

    androidTarget {
        @OptIn(ExperimentalKotlinGradlePluginApi::class)
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_11)
        }
    }

    jvm("desktop")

    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach { iosTarget ->
        iosTarget.binaries.framework {
            baseName = "ComposeApp"
            isStatic = true
        }
    }

    cocoapods {
        version = "1.0.0"
        summary = "Common core for LingoLessons"
        homepage = "http://www.lingolessons.com"
        ios.deploymentTarget = "15.3"
        podfile = project.file("../iosApp/Podfile")
        framework {
            baseName = "composeApp"
            isStatic = true
        }
    }

    sourceSets {
        val desktopMain by getting

        androidMain.dependencies {
            implementation(compose.preview)
            implementation(libs.androidx.activity.compose)
            implementation(libs.koin.android)
            implementation(libs.koin.androidx.compose)
            implementation(libs.kotlinx.coroutines.android)
            implementation(libs.sqldelight.android)
        }

        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material)
            implementation(compose.material3)
            implementation(compose.ui)
            implementation(compose.components.resources)
            implementation(compose.components.uiToolingPreview)
            implementation(libs.androidx.lifecycle.viewmodel)
            implementation(project.dependencies.platform(libs.koin.bom))
            implementation(libs.koin.core)
            implementation(libs.koin.compose)
            implementation(libs.koin.compose.viewmodel)
            implementation(libs.androidx.navigation)
            implementation(libs.kotlinx.coroutines.core)
            //implementation(libs.kotlinx.coroutines.debug)

            // Data Layer
            implementation(libs.ktor.core)
            implementation(libs.ktor.cio)
            implementation(libs.ktor.auth)
            implementation(libs.ktor.logging)
            implementation(libs.ktor.logging.jvm)
            implementation(libs.ktor.serialization)
            implementation(libs.ktorfit.lib)
            implementation(libs.ktorfit.content.negotiation)
            implementation(libs.serialization.json)
        }

        desktopMain.dependencies {
            implementation(compose.desktop.currentOs)
            implementation(libs.kotlinx.coroutines.swing)
            implementation(libs.sqldelight.jvm)
            implementation(libs.paths)
        }

        iosMain.dependencies {
            implementation(libs.ktor.darwin)
            implementation(libs.sqldelight.native)
        }
    }
    task("testClasses")
}

android {
    namespace = "com.lingolessons"
    compileSdk = libs.versions.android.compileSdk.get().toInt()

    sourceSets["main"].manifest.srcFile("src/androidMain/AndroidManifest.xml")
    sourceSets["main"].res.srcDirs("src/androidMain/res")
    sourceSets["main"].resources.srcDirs("src/commonMain/resources")

    defaultConfig {
        applicationId = "com.lingolessons"
        minSdk = libs.versions.android.minSdk.get().toInt()
        targetSdk = libs.versions.android.targetSdk.get().toInt()
        versionCode = 1
        versionName = "1.0"
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
            excludes += "/META-INF/INDEX.LIST"
            excludes += "DebugProbesKt.bin" // Coroutines
        }
    }
    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
    buildFeatures {
        compose = true
    }
    dependencies {
        debugImplementation(compose.uiTooling)
    }
}

compose.desktop {
    application {
        mainClass = "MainKt"

        nativeDistributions {
            targetFormats(
                TargetFormat.Dmg,
                TargetFormat.Exe,
                TargetFormat.Deb,
                TargetFormat.Rpm,
            )
            if (osdetector.os.startsWith("linux", ignoreCase = true)) {
                targetFormats += TargetFormat.AppImage
            }
            packageName = "com.lingolessons"
            packageVersion = "1.0.0"
            appResourcesRootDir.set(project.layout.projectDirectory.dir("resources"))
            modules("jdk.unsupported")
            modules("java.sql")
            macOS {
                bundleID = "com.lingolessons"
                infoPlist { // ref: https://stackoverflow.com/questions/77350286/how-to-hide-the-dock-icon-in-jetpack-compose-desktop
                    extraKeysRawXml = """
                        <key>LSUIElement</key>
                        <string>true</string>
                    """.trimIndent()
                }
                iconFile.set(file("src/desktopMain/resources/macOS.icns"))
            }
            windows {
                iconFile.set(file("src/desktopMain/resources/windows.ico"))
            }
            linux {
                iconFile.set(file("src/desktopMain/resources/linux.png"))
            }
        }
        if (osdetector.os.startsWith("mac", ignoreCase = true)) {
            jvmArgs("-Xdock:icon=src/desktopMain/resources/macOS.icns")
        }
    }
}

sqldelight {
    databases {
        create("AppDatabase") {
            packageName.set("com.lingolessons.data.db")
            dialect(libs.sqldelight.dialect)
        }
    }
}
