import android.os.Build
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalConfiguration
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

class AndroidPlatform : Platform {
    override val name: String = "Android ${Build.VERSION.SDK_INT}"

    @Composable
    override fun isLargeScreen(): Boolean = LocalConfiguration.current.screenWidthDp > 600
}

actual fun getPlatform(): Platform = AndroidPlatform()

actual val platformModule = module {
    single { DriverFactory(androidContext()) }
}
