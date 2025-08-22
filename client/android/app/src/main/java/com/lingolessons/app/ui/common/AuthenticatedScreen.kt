package com.lingolessons.app.ui.common

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import com.lingolessons.app.ui.login.LoginScreen
import com.lingolessons.app.ui.login.LoginViewModel
import com.lingolessons.shared.Session
import org.koin.compose.viewmodel.koinViewModel

@Composable
fun <T> AuthenticatedScreen(
    viewModel: DomainStateViewModel<T>,
    content: @Composable (String) -> Unit
) {
    val state by viewModel.domainState.state.collectAsState()

    when (val currentState = state) {
        is Session.None -> {
            LoginScreen(
                viewModel = koinViewModel<LoginViewModel>(),
            )
        }

        is Session.Authenticated -> {
            content(currentState.v1)
        }
    }
}
