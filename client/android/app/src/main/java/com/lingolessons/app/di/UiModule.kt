package com.lingolessons.app.di

import com.lingolessons.app.ui.lessons.LessonViewModel
import com.lingolessons.app.ui.lessons.LessonsViewModel
import com.lingolessons.app.ui.profile.ProfileViewModel
import com.lingolessons.app.ui.login.LoginViewModel
import org.koin.core.module.dsl.viewModel
import org.koin.dsl.module

val uiModule = module {
    viewModel { LoginViewModel(get()) }
    viewModel { ProfileViewModel(get()) }
    viewModel { LessonsViewModel(get()) }
    viewModel { params ->
        LessonViewModel(
            get(),
            params.get()
        )
    }
}
