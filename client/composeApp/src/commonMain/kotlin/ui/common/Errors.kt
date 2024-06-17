package ui.common

import domain.common.RepositoryError
import io.ktor.utils.io.errors.IOException
import lingolessons.composeapp.generated.resources.Res
import lingolessons.composeapp.generated.resources.error_connection
import lingolessons.composeapp.generated.resources.error_other
import org.jetbrains.compose.resources.getString

suspend fun Result<Any>.getUiErrorMessage(): String =
    when (val error = exceptionOrNull()) {
        is RepositoryError -> error.userFacingError
        is IOException -> getString(Res.string.error_connection)
        else -> null
    } ?: getString(Res.string.error_other)
