package com.lingolessons.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.lingolessons.app.ui.MainScreen
import com.lingolessons.app.ui.theme.AppTheme
import com.lingolessons.shared.DomainBuilder
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

//        CoroutineScope(Dispatchers.IO).launch {
//            try {
//                val path = getDatabasePath("app.db")
//                val domain = DomainBuilder()
//                    .baseUrl("http://10.0.2.2/api/v1/")
//                    .dataPath(path.absolutePath)
//                    .build()
//                val session = domain.login("admin", "admin")
//            } catch (e: Exception) {
//                println("Error! $e")
//            }
//        }
//
        enableEdgeToEdge()
        setContent {
            AppTheme {
                MainScreen()
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    AppTheme {
        Greeting("Android")
    }
}