import com.lingolessons.data.di.dataModule
import com.lingolessons.di.appModule
import com.lingolessons.domain.di.domainModule
import org.koin.core.context.startKoin

class Helper {
    fun initKoin() {
        startKoin {
            modules(appModule, domainModule, platformModule, dataModule)
        }
    }
}
