package com.lingolessons.ui.lesson

import androidx.compose.runtime.Composable
import com.lingolessons.ui.lessons.LessonsViewModel
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI

@OptIn(KoinExperimentalAPI::class)
@Composable
fun LessonScreen(
    viewModel: LessonsViewModel = koinViewModel()
) {

}
