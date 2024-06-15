package ui.main

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color.Companion.Black
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI
import ui.login.LoginScreen
import ui.login.LoginViewModel


enum class AppScreen {
    Login,
}

@OptIn(KoinExperimentalAPI::class)
@Composable
fun MainScreen(
    navController: NavHostController = rememberNavController()
) {
    val backStackEntry by navController.currentBackStackEntryAsState()

    Scaffold {
        NavHost(
            navController = navController,
            startDestination = AppScreen.Login.name,
            modifier = Modifier.fillMaxSize().background(MaterialTheme.colors.background),
        ) {
            composable(route = AppScreen.Login.name) {
                LoginScreen(viewModel = koinViewModel<LoginViewModel>())
            }
        }
    }
}
