import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import data.di.dataModule
import di.appModule
import domain.di.domainModule
import org.koin.compose.KoinApplication
import ui.main.MainScreen
import ui.theme.AppTheme

@Composable
fun App() {
    AppTheme {
        KoinApplication(application = {
            modules(appModule, domainModule, dataModule)
        }) {
        }
        MainScreen()
    }
}
