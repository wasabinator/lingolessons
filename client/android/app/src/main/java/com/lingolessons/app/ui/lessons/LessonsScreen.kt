package com.lingolessons.app.ui.lessons

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Search
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TextFieldColors
import androidx.compose.material3.TextFieldDefaults
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults.topAppBarColors
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.paging.PagingData
import androidx.paging.compose.LazyPagingItems
import androidx.paging.compose.collectAsLazyPagingItems
import com.lingolessons.app.R
import com.lingolessons.app.common.KoverIgnore
import com.lingolessons.app.ui.common.ScreenContent
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.theme.backgroundDark
import com.lingolessons.app.ui.theme.backgroundLight
import com.lingolessons.shared.DateTime
import com.lingolessons.shared.Lesson
import com.lingolessons.shared.LessonType
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.coroutines.flow.flowOf

@Composable
@KoverIgnore
fun LessonsScreen(
    viewModel: LessonsViewModel,
    onLessonSelected: (Lesson) -> Unit,
) {
    val state = viewModel.state.collectAsState()
    LessonsScreen(
        state = state.value,
        updateStatus = viewModel::updateStatus,
        onLessonSelected = onLessonSelected,
        onSearchTextChanged = viewModel::updateFilterText,
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LessonsScreen(
    state: LessonsViewModel.State,
    updateStatus: (ScreenState.Status) -> Unit,
    onLessonSelected: (Lesson) -> Unit,
    onSearchTextChanged: (String) -> Unit,
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
        ScreenContent(
            state = state,
            updateStatus = updateStatus,
            innerPadding = innerPadding,
        ) {
            TextField(
                modifier = Modifier
                    .fillMaxWidth()
                    //.padding(2.dp)
                    .testTag("search_text"),
                leadingIcon = {
                    Icon(
                        imageVector = Icons.Rounded.Search,
                        tint = MaterialTheme.colorScheme.onBackground,
                        contentDescription = "Search icon"
                    )
                },
                colors = TextFieldDefaults.colors(
                    focusedContainerColor = Color.Transparent,
                    unfocusedContainerColor = Color.Transparent,
                ),
                value = state.filterText,
                onValueChange = onSearchTextChanged,
            )
            val items = state.lessons.collectAsLazyPagingItems()
            if (items.itemCount > 0) {
                LessonList(
                    items = items,
                    onLessonSelected = onLessonSelected,
                )
            } else {
                Row(
                    modifier = Modifier.fillMaxHeight(),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Text(
                        stringResource(R.string.feature_lessons_none)
                    )
                }
            }
        }
    }
}

@Composable
private fun LessonList(
    items: LazyPagingItems<Lesson>,
    onLessonSelected: (Lesson) -> Unit,
) {
    LazyColumn(
        modifier = Modifier.testTag("lesson_list"),
        state = rememberLazyListState()
    ) {
        items(items.itemCount) { index ->
            items[index]?.let {
                ListItem(
                    modifier = Modifier.clickable {
                        onLessonSelected(it)
                    },
                    headlineContent = {
                        Text(text = it.title)
                    }
                )
                HorizontalDivider()
            }
        }
    }
}

@Composable
@Preview
fun LessonsScreen_Empty_Preview() {
    LessonsScreen(
        state = LessonsViewModel.State(
            lessons = emptyFlow()
        ),
        updateStatus = {},
        onLessonSelected = {},
        onSearchTextChanged = {},
    )
}

@Composable
@Preview
fun LessonsScreen_List_Preview() {
    LessonsScreen(
        state = LessonsViewModel.State(
            lessons = flowOf(
                PagingData.from(
                    listOf(
                        Lesson(
                            id = "",
                            title = "Lesson 1",
                            type = LessonType.GRAMMAR,
                            language1 = "en",
                            language2 = "jp",
                            owner = "owner",
                            updatedAt = DateTime.now(),
                        ),
                        Lesson(
                            id = "",
                            title = "Lesson 2",
                            type = LessonType.GRAMMAR,
                            language1 = "en",
                            language2 = "jp",
                            owner = "owner",
                            updatedAt = DateTime.now(),
                        )
                    )
                )
            )
        ),
        updateStatus = {},
        onSearchTextChanged = {},
        onLessonSelected = {},
    )
}
