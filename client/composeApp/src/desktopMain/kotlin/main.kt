import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.DpSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Tray
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.WindowPosition
import androidx.compose.ui.window.application
import androidx.compose.ui.window.rememberNotification
import androidx.compose.ui.window.rememberTrayState
import androidx.compose.ui.window.rememberWindowState
import com.lingolessons.data.di.dataModule
import com.lingolessons.data.di.platformModule
import com.lingolessons.di.uiModule
import com.lingolessons.ui.di.domainModule
import org.koin.core.context.startKoin
import java.awt.Dimension

fun main() {
    startKoin {
        modules(uiModule, domainModule, platformModule, dataModule)
    }
    application {
        var isOpen by remember { mutableStateOf(true) }
        val isVisible by remember { mutableStateOf(true) }

        //val systemTray = remember { SystemTray.get() }
        val windowState = rememberWindowState(
            position = WindowPosition(Alignment.Center),
            size = DpSize(800.dp, 600.dp),
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
                resizable = true,
                visible = isVisible,
            ) {
                with(LocalDensity.current) {
                    window.minimumSize = Dimension(400.dp.toPx().toInt(), 600.dp.toPx().toInt())
                }
                App()
            }
        }
    }
}

object TrayIcon : Painter() {
    override val intrinsicSize = Size(256f, 256f)
    override fun DrawScope.onDraw() {
        drawOval(Color(0xFFFFA500))
    }
}
