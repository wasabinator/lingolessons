package com.lingolessons.di

import org.koin.compose.viewmodel.dsl.viewModel
import org.koin.dsl.module
import com.lingolessons.ui.lessons.LessonsViewModel
import com.lingolessons.ui.login.LoginViewModel
import com.lingolessons.ui.profile.ProfileViewModel

val appModule = module {
    viewModel { LoginViewModel(get()) }
    viewModel { ProfileViewModel(get()) }
    viewModel { LessonsViewModel(get()) }
}
