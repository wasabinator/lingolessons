import androidx.compose.runtime.Composable
import org.koin.core.module.Module

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

expect val platformModule: Module
