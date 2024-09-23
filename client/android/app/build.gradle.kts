import java.util.Locale

plugins {
    alias(libs.plugins.cargo.android)
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.kapt)
    alias(libs.plugins.hilt.android)
}

val abiTargets = setOf("arm64-v8a", "x86_64")

android {
    namespace = "com.lingolessons.app"
    compileSdk = 34
    ndkVersion = "27.1.12297006"

    defaultConfig {
        applicationId = "com.lingolessons.app"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"
        ndk.abiFilters += abiTargets

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        compose = true
    }
    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.1"
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.navigation)
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.ui)
    implementation(libs.androidx.ui.graphics)
    implementation(libs.androidx.ui.tooling.preview)
    implementation(libs.androidx.material3)
    implementation(libs.hilt.android)
    implementation(libs.hilt.navigation)
    kapt(libs.hilt.compiler)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.ui.test.junit4)
    debugImplementation(libs.androidx.ui.tooling)
    debugImplementation(libs.androidx.ui.test.manifest)
    implementation(libs.jna) {
        artifact {
            //classifier = "debug"
            type = "aar"
        }
    }
}

// Allow references to generated code
kapt {
    correctErrorTypes = true
}

// This will generate the static lib, along with the uniffi generated wrapper in shared.kt/
cargo {
    verbose = true
    module = "../../rust/shared"
    targetDirectory = "../../rust/target"
    libname = "shared"
    targets = listOf("arm64", "x86_64")

    exec = { spec, toolchain ->
        if (toolchain.target == "x86_64-linux-android") {
            val tc = when (System.getProperties()["os.name"].toString().lowercase()) {
                "windows" -> "windows-x86_64"
                else -> "linux-x86_64"
            }
            spec.environment(
                "RUSTFLAGS",
                "-L${android.sdkDirectory}/ndk/${android.ndkVersion}/toolchains/llvm/prebuilt/$tc/lib/clang/18/lib/linux/ -lstatic=clang_rt.builtins-x86_64-android"
            )
        }
    }
}

tasks.register("generateUniFFIBindings") {
    dependsOn("cargoBuild")
    mustRunAfter("cargoBuild")
    abiTargets.forEach { arch ->
        doLast {
            exec {
                this.workingDir("../../rust")
                this.executable("cargo")
                this.args("run", "--bin", "uniffi-bindgen", "generate", "--library", "${layout.buildDirectory.asFile.get()}/rustJniLibs/android/${arch}/libshared.so", "--language", "kotlin", "--out-dir", "${layout.buildDirectory.asFile.get()}/generated-sources/shared")
            }
        }
    }
}

tasks.whenTaskAdded {
    when (name) {
        "compileDebugKotlin", "compileReleaseKotlin", "compileReleaseTestKotlin" -> {
            dependsOn("generateUniFFIBindings")
            mustRunAfter("generateUniFFIBindings")
        }
    }
}
