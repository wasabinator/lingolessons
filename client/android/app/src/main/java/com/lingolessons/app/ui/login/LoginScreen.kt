package com.lingolessons.app.ui.login

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.lingolessons.app.R
import com.lingolessons.app.common.KoverIgnore
import com.lingolessons.app.ui.common.ErrorSource
import com.lingolessons.app.ui.common.ScreenContent
import com.lingolessons.app.ui.common.ScreenState
import com.lingolessons.app.ui.login.LoginViewModel.Errors
import com.lingolessons.app.ui.login.LoginViewModel.ScreenData

@Composable
@KoverIgnore
fun LoginScreen(viewModel: LoginViewModel) {
    val state by viewModel.state.collectAsState()
    LoginScreen(
        state = state,
        updateUsername = viewModel::updateUsername,
        updatePassword = viewModel::updatePassword,
        login = viewModel::login,
        errorMessage = { error ->
            stringResource(
                when (error) {
                    Errors.UnauthorisedError -> R.string.auth_invalid_credentials
                    else -> R.string.error_other
                },
            )
        },
        clearStatus = viewModel::clearStatus,
    )
}

@Composable
fun LoginScreen(
    state: ScreenState<ScreenData>,
    updateUsername: (String) -> Unit,
    updatePassword: (String) -> Unit,
    login: () -> Unit,
    errorMessage: @Composable (ErrorSource) -> String,
    clearStatus: () -> Unit,
) {
    ScreenContent(
        state = state,
        errorMessage = errorMessage,
        clearStatus = clearStatus,
    ) {
        Row(
            modifier =
                Modifier.fillMaxWidth().background(color = MaterialTheme.colorScheme.background),
        ) {
            Box(modifier = Modifier.weight(0.2f))

            Column(
                modifier =
                    Modifier.weight(0.6f)
                        .windowInsetsPadding(WindowInsets.safeDrawing)
                        .imePadding()
                        .verticalScroll(
                            rememberScrollState(),
                        ),
                horizontalAlignment = Alignment.CenterHorizontally) {
                    Image(
                        modifier = Modifier.padding(vertical = 48.dp),
                        painter = painterResource(id = R.drawable.logo),
                        contentDescription = null, // decorative element
                    )

                    OutlinedTextField(
                        modifier = Modifier.padding(bottom = 16.dp).testTag("username"),
                        value = state.data.username,
                        onValueChange = updateUsername,
                        placeholder = { Text(stringResource(id = R.string.username)) },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Text),
                        maxLines = 1,
                    )

                    OutlinedTextField(
                        modifier = Modifier.padding(bottom = 24.dp).testTag("password"),
                        value = state.data.password,
                        onValueChange = updatePassword,
                        placeholder = { Text(stringResource(id = R.string.password)) },
                        visualTransformation = PasswordVisualTransformation(),
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password),
                        maxLines = 1,
                    )

                    Button(
                        modifier = Modifier.testTag("login"),
                        enabled = state.data.enabled,
                        onClick = login,
                    ) {
                        Box(modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) {
                            Text(stringResource(R.string.btn_login))
                        }
                    }
                }

            Box(modifier = Modifier.weight(0.2f))
        }
    }
}

@Composable
@Preview
fun LoginScreenPreview() {
    LoginScreen(
        state = ScreenState(ScreenData()),
        updateUsername = {},
        updatePassword = {},
        login = {},
        errorMessage = { "" },
        clearStatus = {},
    )
}
