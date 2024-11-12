plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.kapt)
    alias(libs.plugins.kotlinx.kover)
}

val abiTargets = setOf("arm64-v8a", "x86_64")

android {
    namespace = "com.lingolessons.app"
    compileSdk = 34
    ndkVersion = "27.1.12297006"

    defaultConfig {
        applicationId = "com.lingolessons.app"
        minSdk = 26
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
        freeCompilerArgs += "-Xcontext-receivers"
    }
    @Suppress("UnstableApiUsage")
    testOptions {
        unitTests.isIncludeAndroidResources = true
        unitTests.isReturnDefaultValues = true
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
            merges += "META-INF/LICENSE.md"
            merges += "META-INF/LICENSE-notice.md"
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
    implementation(project.dependencies.platform(libs.koin.bom))
    implementation(libs.koin.core)
    implementation(libs.koin.android)
    implementation(libs.koin.compose.viewmodel.navigation)
    implementation(libs.androidx.paging.compose)

    testImplementation(libs.junit)
    testImplementation(libs.mockk.android)
    testImplementation(libs.mockk.agent)
    testImplementation(libs.kotlinx.coroutines.test)
    testImplementation(libs.robolectric.robolectric)
    testImplementation(libs.androidx.paging.common)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.ui.test.junit4)
    androidTestImplementation(libs.mockk.android)
    androidTestImplementation(libs.mockk.agent)
    debugImplementation(libs.androidx.ui.tooling)
    debugImplementation(libs.androidx.ui.test.manifest)
    debugImplementation(libs.androidx.ui.test.junit4.android)
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

// Generation tasks for building the shared Rust lib plus bindings to interact with it
tasks.register("generateUniFFIBindings") {
    doLast {
        exec {
            this.workingDir("../../rust")
            this.executable("./make_android.sh")
            this.args("if_not_exists")
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

kover {
    reports {
        filters {
            excludes {
                packages(
                    "*.di",
                    "*.shared",
                    "*.ui.theme",
                )

                classes(
                    "*ComposableSingletons$*",
                    "*.MainActivity*",
                )

                annotatedBy(
                    "androidx.compose.ui.tooling.preview.Preview",
                )
            }
        }
    }
}