import androidx.compose.runtime.Composable
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.platform.LocalWindowInfo

class JVMPlatform : Platform {
    override val name: String = System.getProperty("os.name")

    @OptIn(ExperimentalComposeUiApi::class)
    @Composable
    override fun isLargeScreen(): Boolean = LocalWindowInfo.current.containerSize.width > 600
}

actual fun getPlatform(): Platform = JVMPlatform()
