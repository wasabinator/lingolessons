package ui.lessons

import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI

@OptIn(KoinExperimentalAPI::class)
@Composable
fun LessonsScreen(viewModel: LessonsViewModel = koinViewModel()) = Column {
    Text("lessons")

    val state = viewModel.state.collectAsState()

    if (state.value != null) {
        Text("Items: ${state.value?.total}")
    }
}
