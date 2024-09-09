import com.lingolessons.domain.lesson.SaveLesson
import kotlinx.cinterop.ExperimentalForeignApi

import gtk4.*
import kotlinx.cinterop.COpaquePointer
import kotlinx.cinterop.CPointer
import kotlinx.cinterop.StableRef
import kotlinx.cinterop.asStableRef
import kotlinx.cinterop.reinterpret
import kotlinx.cinterop.staticCFunction

private const val APP_ID = "com.lingolessons.app"

@OptIn(ExperimentalForeignApi::class)
fun main() {
    val lesson = SaveLesson()

    val app: CPointer<GtkApplication>? =
        gtk_application_new(
            application_id = "com.lingolessons.app",
            flags = G_APPLICATION_DEFAULT_FLAGS
        )

    val callback = StableRef.create(staticCFunction { source: COpaquePointer,
                                                      data: COpaquePointer ->
        println("*** CALLED BACK ***")
        data.asStableRef<() -> Unit>().get().invoke()
    })

    gtk_signal_connect(
        instance = app,
        detailed_signal = "activate",
        c_handler = callback.asCPointer().reinterpret(),
        data = null
    )

    gtk_application_run(
        app = app,
        argc = 0,
        argv = null
    )

    println("Hello world 123!")
}
