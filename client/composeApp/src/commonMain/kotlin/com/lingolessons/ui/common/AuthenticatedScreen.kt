package com.lingolessons.ui.common

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import com.lingolessons.domain.auth.GetUserSession
import com.lingolessons.domain.auth.SessionState
import org.koin.compose.koinInject
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI
import com.lingolessons.ui.login.LoginScreen
import com.lingolessons.ui.login.LoginViewModel

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
