package di

import org.koin.compose.viewmodel.dsl.viewModel
import org.koin.dsl.module
import ui.login.LoginViewModel

val appModule = module {
    viewModel { LoginViewModel(get()) }
}
