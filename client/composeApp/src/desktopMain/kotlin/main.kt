import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Tray
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.application
import androidx.compose.ui.window.rememberNotification
import androidx.compose.ui.window.rememberTrayState
import androidx.compose.ui.window.rememberWindowState

fun main() = application {
    var isOpen by remember { mutableStateOf(true) }
    var isVisible by remember { mutableStateOf(false) }

    //val systemTray = remember { SystemTray.get() }
    val windowState = rememberWindowState(
        position = WindowPosition(Alignment.Center),
        size = DpSize(450.dp, 600.dp),
        isMinimized = false
    )


    if (isOpen) {
        val trayState = rememberTrayState()
        val notification = rememberNotification("Notification", "Message from MyApp!")

        Tray(
            state = trayState,
            icon = TrayIcon,
            onAction = {
                windowState.isMinimized = false
            },
            menu = {
                // TODO: Change Tray source so that it doesn't set popupMenu if menu is null
                if (!getPlatform().isLinux) {
                    Item(
                        "Increment value",
                        onClick = {
                        }
                    )
                    Item(
                        "Send notification",
                        onClick = {
                            trayState.sendNotification(notification)
                        }
                    )
                    Item(
                        "Exit",
                        onClick = {
                            isOpen = false
                        }
                    )
                }
            }
        )

        Window(
            state = windowState,
            onCloseRequest = {
                //::exitApplication,
                isOpen = false
            },
            title = "LingoLessons",
            resizable = false,
            //visible = isVisible,
        ) {
            App()
        }
    }
}

object TrayIcon : Painter() {
    override val intrinsicSize = Size(256f, 256f)
    override fun DrawScope.onDraw() {
        drawOval(Color(0xFFFFA500))
    }
}
