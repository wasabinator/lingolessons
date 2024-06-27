import androidx.compose.runtime.Composable
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.platform.LocalWindowInfo
import com.lingolessons.data.db.DriverFactory
import org.koin.dsl.module

class JVMPlatform : Platform {
    override val name: String = System.getProperty("os.name")

    @OptIn(ExperimentalComposeUiApi::class)
    @Composable
    override fun isLargeScreen(): Boolean = LocalWindowInfo.current.containerSize.width > 600
}

actual fun getPlatform(): Platform = JVMPlatform()

actual val platformModule = module {
    single { DriverFactory() }
}
