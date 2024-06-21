package ui.main

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import org.koin.core.annotation.KoinExperimentalAPI
import ui.lessons.LessonsScreen
import ui.profile.ProfileScreen
import ui.study.StudyScreen

enum class AppScreen {
    Profile,
    Study,
    Lessons,
}

@OptIn(KoinExperimentalAPI::class)
@Composable
fun MainNav(
    navController: NavHostController
) {
    NavHost(
        navController = navController,
        startDestination = AppScreen.Profile.name,
        modifier = Modifier.fillMaxSize().background(MaterialTheme.colors.background),
    ) {
        composable(route = AppScreen.Profile.name) {
            ProfileScreen()
            //LoginScreen(viewModel = koinViewModel<LoginViewModel>())
        }
        composable(route = AppScreen.Study.name) {
            StudyScreen()
            //LoginScreen(viewModel = koinViewModel<LoginViewModel>())
        }
        composable(route = AppScreen.Lessons.name) {
            LessonsScreen()
            //LoginScreen(viewModel = koinViewModel<LoginViewModel>())
        }
    }
}
