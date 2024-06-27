package com.lingolessons.di

import org.koin.compose.viewmodel.dsl.viewModel
import org.koin.dsl.module
import com.lingolessons.ui.lessons.LessonsViewModel
import com.lingolessons.ui.login.LoginViewModel

val appModule = module {
    viewModel { LoginViewModel(get()) }
    viewModel { LessonsViewModel(get()) }
}
