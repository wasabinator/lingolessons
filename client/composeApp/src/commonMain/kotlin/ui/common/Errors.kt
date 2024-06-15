package ui.common

import domain.common.RepositoryError

fun Result<Any>.getUiErrorMessage(): String {
    val errorMessage: String? = when (val error = exceptionOrNull()) {
        is RepositoryError -> error.userFacingError
        else -> null
    }
    return errorMessage ?: "Something went wrong. Please try again"
}
