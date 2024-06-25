package di

import org.koin.compose.viewmodel.dsl.viewModel
import org.koin.dsl.module
import ui.lessons.LessonsViewModel
import ui.login.LoginViewModel

val appModule = module {
    viewModel { LoginViewModel(get()) }
    viewModel { LessonsViewModel(get()) }
}
