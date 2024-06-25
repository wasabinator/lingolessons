package ui.common

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import domain.auth.GetUserSession
import domain.auth.SessionState
import org.koin.compose.koinInject
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI
import ui.login.LoginScreen
import ui.login.LoginViewModel

@OptIn(KoinExperimentalAPI::class)
@Composable
fun AuthenticatedScreen(
    getUserSession: GetUserSession = koinInject(),
    content: @Composable (String) -> Unit
) {
    val state = getUserSession.perform(Unit).collectAsState(initial = null)
    when (val currentState = state.value) {
        is SessionState.None -> {
            LoginScreen(viewModel = koinViewModel<LoginViewModel>())
        }

        is SessionState.Authenticated -> {
            content(currentState.username)
        }

        null -> Unit
    }
}
