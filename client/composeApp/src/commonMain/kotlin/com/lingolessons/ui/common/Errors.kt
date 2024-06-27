package com.lingolessons.ui.common

import com.lingolessons.domain.common.DomainError
import io.ktor.utils.io.errors.IOException
import lingolessons.composeapp.generated.resources.Res
import lingolessons.composeapp.generated.resources.error_connection
import lingolessons.composeapp.generated.resources.error_other
import org.jetbrains.compose.resources.getString

suspend fun Result<Any>.getUiErrorMessage(): String =
    when (val error = exceptionOrNull()) {
        is DomainError -> error.userFacingError
        is IOException -> getString(Res.string.error_connection)
        else -> null
    } ?: getString(Res.string.error_other)
