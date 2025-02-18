package com.lingolessons.app.common

import android.util.Log

inline fun <reified T: Any> T.logWarning(msg: String, t: Throwable? = null) {
    Log.w(T::class.qualifiedName, msg, t)
}

inline fun <reified T: Any> T.logError(msg: String, t: Throwable? = null) {
    Log.e(T::class.qualifiedName, msg, t)
}
