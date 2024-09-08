plugins {
    alias(libs.plugins.kotlin.multiplatform)
}

repositories {
    mavenCentral()
}

kotlin {
    val os = System.getProperty("os.name")
    val isArm64 = System.getProperty("os.arch") == "aarch64"
    when {
        os == "Mac OS X" -> if (isArm64) macosArm64() else macosX64()
        os == "Linux" ->
            (if (isArm64) linuxArm64() else linuxX64()).apply {
                compilations["main"].cinterops {
                    val gtk4 by creating
                }
            }

        os.startsWith("Windows") -> mingwX64()
        else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
    }.binaries {
        executable()
    }

    sourceSets {
        //val linuxApp by creating
        val linuxMain by creating {
            dependencies {
                implementation(projects.shared)
            }
        }
    }
}
