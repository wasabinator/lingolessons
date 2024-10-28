package com.lingolessons.app.ui.lessons

import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults.topAppBarColors
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color.Companion.Gray
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.lingolessons.app.R
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType

@Composable
fun LessonsScreen(
    viewModel: LessonsViewModel,
) {
    val state = viewModel.state.collectAsState()
    LessonsScreen(
        state = state.value,
        onLessonSelected = {},
        onRefresh = viewModel::refresh
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LessonsScreen(
    state: LessonsViewModel.State,
    onLessonSelected: () -> Unit,
    onRefresh: () -> Unit,
) {
    Scaffold(
        topBar = {
            TopAppBar(
                colors = topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer,
                    titleContentColor = MaterialTheme.colorScheme.primary,
                ),
                title = {
                    Text(stringResource(R.string.feature_lessons))
                }
            )
        }
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(innerPadding),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            when {
                state.isBusy -> {
                    Text(stringResource(R.string.text_loading))
                }

                state.isError -> {
                    Text(stringResource(R.string.text_error))
                }

                else -> {
                    if (state.lessons.isNotEmpty()) {
                        LessonList(state.lessons)
                    } else {
                        Text(stringResource(R.string.feature_lessons_none))
                    }
                }
            }
        }

        LaunchedEffect(Unit) {
            onRefresh()
        }
    }
}

@Composable
private fun LessonList(lessons: List<Lesson>) {
    LazyColumn(
        modifier = Modifier.testTag("lesson_list"),
        state = rememberLazyListState()
    ) {
        item {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 16.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(pluralStringResource(R.plurals.feature_lessons_total, lessons.count()))
            }
        }
        items(lessons) {
            Row(
                Modifier
                    .height(IntrinsicSize.Max)
                    .fillMaxWidth()
                    .border(1.dp, Gray)
                    .padding(vertical = 12.dp)
            ) {
                Text(
                    modifier = Modifier
                        .align(Alignment.CenterVertically)
                        .padding(horizontal = 12.dp),
                    text = it.title,
                )
            }
        }
    }
}

@Composable
@Preview
fun LessonsScreen_Empty_Preview() {
    LessonsScreen(
        state = LessonsViewModel.State(),
        onLessonSelected = {},
        onRefresh = {},
    )
}

@Composable
@Preview
fun LessonsScreen_List_Preview() {
    LessonsScreen(
        state = LessonsViewModel.State(
            lessons = listOf(
                Lesson(
                    id = "",
                    title = "",
                    type = LessonType.GRAMMAR,
                    language1 = "",
                    language2 = "",
                    owner = "owner",
                    updatedAt = DateTime.now(),
                )
            )
        ),
        onLessonSelected = {},
        onRefresh = {},
    )
}
