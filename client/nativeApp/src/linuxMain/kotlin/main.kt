import com.lingolessons.domain.lesson.SaveLesson
import kotlinx.cinterop.ExperimentalForeignApi

import gtk4.*

private const val APP_ID = "com.lingolessons.app"

@OptIn(ExperimentalForeignApi::class)
fun main() {
    val lesson = SaveLesson()

    //val app = Application(APP_ID, ApplicationFlags.FLAGS_NONE)
    val x = gSignalConnect(

    )

    println("Hello world 123!")
}
