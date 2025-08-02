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

@Composable
fun ScreenContent(
    state: ScreenState,
    busyIndicator: @Composable ((ScreenState.Status) -> Unit) -> Unit = { u -> BusyContent(u) },
    errorIndicator: @Composable (String, (ScreenState.Status) -> Unit) -> Unit = { s, u ->
        ErrorContent(s, u)
    },
    innerPadding: PaddingValues = PaddingValues(),
    updateStatus: (ScreenState.Status) -> Unit,
    content: @Composable () -> Unit = {}
) {
    Column(
        modifier = Modifier.fillMaxWidth().padding(innerPadding),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        content()

        when {
            state.isBusy -> busyIndicator(updateStatus)
            state.isError -> errorIndicator(state.errorMessage!!, updateStatus)
        }
    }
}

@Composable
fun BusyContent(updateStatus: (ScreenState.Status) -> Unit) {
    Dialog(
        onDismissRequest = { updateStatus(ScreenState.Status.None) },
        DialogProperties(dismissOnBackPress = false, dismissOnClickOutside = false)
    ) {
        Box(
            contentAlignment = Center,
            modifier =
            Modifier
                .size(100.dp)
                .background(
                    MaterialTheme.colorScheme.background,
                    shape = RoundedCornerShape(8.dp)
                )
        ) {
            CircularProgressIndicator()
        }
    }
}

@Composable
fun ErrorContent(message: String, updateStatus: (ScreenState.Status) -> Unit) {
    AlertDialog(
        onDismissRequest = { updateStatus(ScreenState.Status.None) },
        title = { Text(stringResource(R.string.title_error)) },
        text = { Text(message) },
        confirmButton = {
            Button(onClick = { updateStatus(ScreenState.Status.None) }) {
                Text(stringResource(R.string.btn_ok))
            }
        }
    )
}
