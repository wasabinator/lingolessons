import com.lingolessons.data.di.dataModule
import com.lingolessons.data.di.platformModule
import com.lingolessons.di.uiModule
import com.lingolessons.ui.di.domainModule
import org.koin.core.context.startKoin

fun initKoin() {
    startKoin {
        modules(uiModule, domainModule, platformModule, dataModule)
    }
}
