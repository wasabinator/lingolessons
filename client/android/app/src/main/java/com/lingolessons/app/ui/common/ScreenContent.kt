package com.lingolessons.app.ui.common

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Alignment.Companion.Center
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import com.lingolessons.app.R
import com.lingolessons.app.ui.common.ScreenState.Status

@Composable
fun <T> ScreenContent(
    state: ScreenState<T>,
    busyIndicator: @Composable (() -> Unit) -> Unit = { c -> BusyContent(c) },
    errorIndicator: @Composable (String, () -> Unit) -> Unit = { s, c -> ErrorContent(s, c) },
    innerPadding: PaddingValues = PaddingValues(),
    errorMessage: @Composable (ErrorSource) -> String,
    clearStatus: () -> Unit,
    content: @Composable () -> Unit = {}
) {
    Column(
        modifier = Modifier.fillMaxWidth().padding(innerPadding),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        content()

        val status = state.status
        when (status) {
            Status.Busy -> busyIndicator(clearStatus)
            is Status.Error -> errorIndicator(errorMessage(status.source), clearStatus)
            else -> {}
        }
    }
}

@Composable
fun BusyContent(clearStatus: () -> Unit) {
    Dialog(
        onDismissRequest = clearStatus,
        DialogProperties(dismissOnBackPress = false, dismissOnClickOutside = false)) {
            Box(
                contentAlignment = Center,
                modifier =
                    Modifier.size(100.dp)
                        .background(
                            MaterialTheme.colorScheme.background,
                            shape = RoundedCornerShape(8.dp),
                        ),
            ) {
                CircularProgressIndicator()
            }
        }
}

@Composable
fun ErrorContent(message: String, clearStatus: () -> Unit) {
    AlertDialog(
        onDismissRequest = clearStatus,
        title = { Text(stringResource(R.string.title_error)) },
        text = { Text(message) },
        confirmButton = { Button(onClick = clearStatus) { Text(stringResource(R.string.btn_ok)) } },
    )
}
