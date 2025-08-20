package com.lingolessons.app.ui.lessons

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults.topAppBarColors
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import com.lingolessons.app.R
import com.lingolessons.app.common.KoverIgnore
import com.lingolessons.app.ui.common.ErrorSource
import com.lingolessons.app.ui.common.ScreenContent
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.lessons.LessonViewModel.ScreenData
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType

@Composable
@KoverIgnore
fun LessonScreen(viewModel: LessonViewModel, onNavigateBack: () -> Unit) {
    val state = viewModel.state.collectAsState()
    LessonScreen(
        state = state.value,
        errorMessage = { stringResource(R.string.error_other) },
        onNavigateBack = onNavigateBack,
        clearStatus = viewModel::clearStatus,
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LessonScreen(
    state: ScreenState<ScreenData>,
    errorMessage: @Composable (ErrorSource) -> String,
    onNavigateBack: () -> Unit,
    clearStatus: () -> Unit,
) {
    Scaffold(
        topBar = {
            TopAppBar(
                colors =
                    topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.primaryContainer,
                        titleContentColor = MaterialTheme.colorScheme.primary,
                    ),
                title = {
                    Text(
                        modifier = Modifier.testTag("screen_title"),
                        text = state.data.lesson?.title ?: "",
                    )
                },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(
                            imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = "Localized description",
                        )
                    }
                })
        }) { innerPadding ->
            ScreenContent(
                state = state,
                innerPadding = innerPadding,
                errorMessage = errorMessage,
                clearStatus = clearStatus,
            ) {}
        }
}

@Composable
@Preview
fun LessonScreen_Preview() {
    LessonScreen(
        state =
            ScreenState(
                data =
                    ScreenData(
                        lessonId = "123",
                        lesson =
                            Lesson(
                                id = "123",
                                title = "Lesson 1",
                                type = LessonType.GRAMMAR,
                                language1 = "en",
                                language2 = "jp",
                                owner = "owner",
                                updatedAt = DateTime.now(),
                            ),
                    ),
            ),
        errorMessage = { "" },
        onNavigateBack = {},
        clearStatus = {},
    )
}
