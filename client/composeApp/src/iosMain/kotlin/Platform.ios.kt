import androidx.compose.runtime.Composable
import com.lingolessons.data.db.DriverFactory
import org.koin.dsl.module
import platform.UIKit.UIDevice

class IOSPlatform : Platform {
    override val name: String =
        UIDevice.currentDevice.systemName() + " " + UIDevice.currentDevice.systemVersion

    @Composable
    override fun isLargeScreen() = false
}

actual fun getPlatform(): Platform = IOSPlatform()

actual val platformModule = module {
    single { DriverFactory() }
}
