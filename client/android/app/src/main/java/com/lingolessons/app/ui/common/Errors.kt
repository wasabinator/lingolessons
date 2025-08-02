package com.lingolessons.app.ui.common

import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import com.lingolessons.app.R
import com.lingolessons.shared.DomainException
import java.io.IOException

@Composable
fun Result<Any>.uiMessage(fallbackMessage: String = stringResource(R.string.error_other)): String =
    when (val error = exceptionOrNull()) {
        is DomainException -> error.message
        is IOException -> stringResource(R.string.error_connection)
        else -> null
    } ?: fallbackMessage
