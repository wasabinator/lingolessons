import androidx.compose.runtime.Composable

interface Platform {
    val name: String

    val isLinux: Boolean
        get() {
            return name.contains("linux", true)
        }

    @Composable
    fun isLargeScreen(): Boolean
}

expect fun getPlatform(): Platform