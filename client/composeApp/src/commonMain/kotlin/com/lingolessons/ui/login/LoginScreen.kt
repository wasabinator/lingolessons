package com.lingolessons.ui.login

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
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.AlertDialog
import androidx.compose.material.Button
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.MaterialTheme
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Alignment.Companion.Center
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import lingolessons.composeapp.generated.resources.Res
import lingolessons.composeapp.generated.resources.logo
import lingolessons.composeapp.generated.resources.password
import lingolessons.composeapp.generated.resources.username
import org.jetbrains.compose.resources.imageResource
import org.jetbrains.compose.resources.stringResource
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI


@OptIn(KoinExperimentalAPI::class)
@Composable
fun LoginScreen(viewModel: LoginViewModel = koinViewModel()) {
    val state by viewModel.state.collectAsState()
    LoginScreen(
        state = state,
        updateUsername = viewModel::updateUsername,
        updatePassword = viewModel::updatePassword,
        login = viewModel::login,
        dismissDialog = viewModel::dismissDialog
    )
}

@Composable
fun LoginScreen(
    state: LoginViewModel.LoginState,
    updateUsername: (String) -> Unit,
    updatePassword: (String) -> Unit,
    login: () -> Unit,
    dismissDialog: () -> Unit,
) {
    Row(
        modifier = Modifier.fillMaxWidth().background(color = MaterialTheme.colors.background)
    ) {
        Box(modifier = Modifier.weight(0.2f)) {}
        Column(
            modifier = Modifier
                .weight(0.6f)
                .windowInsetsPadding(WindowInsets.safeDrawing)
                .imePadding()
                .verticalScroll(rememberScrollState()),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Image(
                modifier = Modifier.padding(vertical = 48.dp),
                bitmap = imageResource(Res.drawable.logo),
                contentDescription = null
            )

            OutlinedTextField(
                modifier = Modifier.padding(bottom = 16.dp).testTag("username"),
                value = state.username,
                onValueChange = updateUsername,
                placeholder = { Text(stringResource(Res.string.username)) },
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Text),
                maxLines = 1,
            )

            OutlinedTextField(
                modifier = Modifier.padding(bottom = 16.dp).testTag("password"),
                value = state.password,
                onValueChange = updatePassword,
                placeholder = { Text(stringResource(Res.string.password)) },
                visualTransformation = PasswordVisualTransformation(),
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password),
                maxLines = 1,
            )

            Button(
                modifier = Modifier.testTag("login"),
                enabled = state.enabled,
                onClick = login,
            ) {
                Box(modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) {
                    Text("Login")
                }
            }
        }
        Box(modifier = Modifier.weight(0.2f)) {}

        if (state.isError) {
            AlertDialog(
                onDismissRequest = dismissDialog,
                title = { Text("Error") },
                text = { Text(state.errorMessage!!) },
                confirmButton = {
                    Button(
                        onClick = dismissDialog
                    ) {
                        Text("OK")
                    }
                }
            )
        }

        if (state.isBusy) {
            Dialog(
                onDismissRequest = dismissDialog,
                DialogProperties(dismissOnBackPress = false, dismissOnClickOutside = false)
            ) {
                Box(
                    contentAlignment = Center,
                    modifier = Modifier
                        .size(100.dp)
                        .background(
                            MaterialTheme.colors.background,
                            shape = RoundedCornerShape(8.dp)
                        )
                ) {
                    CircularProgressIndicator()
                }
            }
        }
    }
}
