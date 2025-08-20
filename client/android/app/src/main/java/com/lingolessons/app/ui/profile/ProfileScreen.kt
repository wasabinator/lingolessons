package com.lingolessons.app.ui.profile

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults.topAppBarColors
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.lingolessons.app.R
import com.lingolessons.app.common.KoverIgnore
import com.lingolessons.app.ui.common.ErrorSource
import com.lingolessons.app.ui.common.ScreenContent
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.profile.ProfileViewModel.ScreenData

@Composable
@KoverIgnore
fun ProfileScreen(viewModel: ProfileViewModel) {
    val state by viewModel.state.collectAsState()
    ProfileScreen(
        state = state,
        errorMessage = { stringResource(R.string.error_other) },
        logout = viewModel::logout,
        clearStatus = viewModel::clearStatus,
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProfileScreen(
    state: ScreenState<ScreenData>,
    errorMessage: @Composable (ErrorSource) -> String,
    logout: () -> Unit,
    clearStatus: () -> Unit,
) {
    Scaffold(
        topBar = {
            TopAppBar(
                colors =
                    topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.primaryContainer,
                        titleContentColor = MaterialTheme.colorScheme.primary),
                title = { Text(stringResource(R.string.feature_profile)) },
            )
        },
    ) { innerPadding ->
        ScreenContent(
            state = state,
            innerPadding = innerPadding,
            errorMessage = errorMessage,
            clearStatus = clearStatus,
        ) {
            Spacer(modifier = Modifier.height(16.dp))
            Button(modifier = Modifier.testTag("logout"), onClick = logout) {
                Box(modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) {
                    Text(stringResource(R.string.btn_logout))
                }
            }
        }
    }
}

@Composable
@Preview
fun ProfileScreenPreview() {
    ProfileScreen(
        state = ScreenState(ScreenData()),
        logout = {},
        errorMessage = { "" },
        clearStatus = {},
    )
}
