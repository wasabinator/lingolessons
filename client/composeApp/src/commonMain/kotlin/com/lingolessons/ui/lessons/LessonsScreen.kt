package com.lingolessons.ui.lessons

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import getPlatform
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI

@OptIn(KoinExperimentalAPI::class)
@Composable
fun LessonsScreen(
    largeScreen: Boolean = getPlatform().isLargeScreen(),
    viewModel: LessonsViewModel = koinViewModel()
) = Box(
    modifier = Modifier.padding(horizontal = 8.dp).fillMaxSize()
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        val state = viewModel.state.collectAsState()
        val listState = rememberLazyListState()

        val shouldLoadNextPage = remember {
            derivedStateOf {
                state.value.data?.let {
                    (it.lessons.size < it.total) &&
                            ((listState.layoutInfo.visibleItemsInfo.lastOrNull()?.index
                                ?: 0) >= listState.layoutInfo.totalItemsCount - 4)
                } == true
            }
        }

        LaunchedEffect(key1 = shouldLoadNextPage.value) {
            if (shouldLoadNextPage.value && viewModel.state.value.isSuccess())
                viewModel.loadNextPage()
        }

        LazyColumn(
            state = listState
        ) {
            val lessonState = state.value
            lessonState.data?.let { data ->
                item {
                    Text(
                        modifier = Modifier.fillMaxWidth().padding(vertical = 16.dp),
                        text = "You currently have ${data.total} lessons. Select any below to edit",
                        textAlign = TextAlign.Center,
                    )
                }
                items(data.lessons.size) { index ->
                    RowItem {
                        Text(
                            modifier = Modifier.fillMaxWidth().padding(4.dp),
                            text = data.lessons[index].title,
                            textAlign = TextAlign.Center,
                            fontWeight = FontWeight.SemiBold,
                        )
                    }
                }
            }
            when {
                lessonState.isLoading() -> {
                    item {
                        Row(modifier = Modifier.fillMaxWidth()) {
                            Text(
                                modifier = Modifier.fillMaxWidth().padding(vertical = 16.dp),
                                text = "Loading...",
                                textAlign = TextAlign.Center,
                            )
                        }
                    }
                }

                lessonState.isError() -> {
                    item {
                        Row(modifier = Modifier.fillMaxWidth()) {
                            Text(
                                modifier = Modifier.fillMaxWidth()
                                    .padding(horizontal = 8.dp, vertical = 16.dp),
                                text = "Error...",
                                textAlign = TextAlign.Center,
                            )
                        }
                    }
                }

                else -> {
                    if (lessonState.data == null) {
                        item {
                            Row(modifier = Modifier.fillMaxWidth()) {
                                Text("You currently have no lessons")
                            }
                        }
                    }
                }
            }
        }
    }

    ExtendedFloatingActionButton(
        onClick = {},
        expanded = largeScreen,
        icon = { Icon(Icons.Filled.Add, "Localized description") },
        text = { Text(text = "New Lesson") },
        modifier = Modifier.align(Alignment.BottomEnd).padding(12.dp)
    )
}

@Composable
fun RowItem(content: @Composable () -> Unit) {
    Card(
        modifier = Modifier.fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 2.dp),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant.copy(
                alpha = 0.5f
            )
        )
    ) {
        Row(modifier = Modifier.padding(8.dp)) {
            content()
        }
    }
}
