interface Platform {
    val name: String

    val isLinux: Boolean
        get() {
            return name.contains("linux", true)
        }
}

expect fun getPlatform(): Platform