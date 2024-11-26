package com.lingolessons.app.ui.lessons

import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults.topAppBarColors
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import com.lingolessons.app.ui.common.ScreenContent
import com.lingolessons.app.ui.common.ScreenState
import kotlin.reflect.KFunction1

@Composable
fun LessonScreen(
    viewModel: LessonViewModel,
) {
    val state = viewModel.state.collectAsState()
    LessonScreen(
        state = state.value,
        updateStatus = viewModel::updateStatus
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LessonScreen(
    state: LessonViewModel.State,
    updateStatus: (ScreenState.Status) -> Unit,
) {
    Scaffold(
        topBar = {
            TopAppBar(
                colors = topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer,
                    titleContentColor = MaterialTheme.colorScheme.primary,
                ),
                title = {
                    Text(
                        modifier = Modifier.testTag("screen_title"),
                        text = state.lesson?.title ?: ""
                    )
                }
            )
        }
    ) { innerPadding ->
        ScreenContent(
            state = state,
            innerPadding = innerPadding,
            updateStatus = updateStatus
        ) {

        }
    }
}
