package com.lingolessons.app.di

import com.lingolessons.app.ui.lessons.LessonsViewModel
import com.lingolessons.app.ui.profile.ProfileViewModel
import com.lingolessons.ui.login.LoginViewModel
import org.koin.core.module.dsl.viewModel
import org.koin.dsl.module

val uiModule = module {
    viewModel { LoginViewModel(get()) }
    viewModel { ProfileViewModel(get()) }
    viewModel { LessonsViewModel(get()) }
}
