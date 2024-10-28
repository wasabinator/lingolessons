package com.lingolessons.app.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import com.lingolessons.app.ui.common.AuthenticatedScreen
import com.lingolessons.app.ui.lessons.LessonsScreen
import com.lingolessons.app.ui.lessons.LessonsViewModel
import com.lingolessons.app.ui.profile.ProfileScreen
import com.lingolessons.app.ui.profile.ProfileViewModel
import org.koin.compose.viewmodel.koinViewModel

enum class AppScreen {
    Profile,
    Study,
    Lessons,
}

@Composable
fun MainNav(
    navController: NavHostController
) {
    NavHost(
        navController = navController,
        startDestination = AppScreen.Profile.name,
        modifier = Modifier.fillMaxSize().background(MaterialTheme.colorScheme.background),
    ) {
        composable(route = AppScreen.Profile.name) {
            val viewModel: ProfileViewModel = koinViewModel()
            AuthenticatedScreen(viewModel = viewModel) {
                ProfileScreen(
                    viewModel = viewModel,
                )
            }
        }
        composable(route = AppScreen.Study.name) {
            // TODO: Add study screen
        }
        composable(route = AppScreen.Lessons.name) {
            val viewModel: LessonsViewModel = koinViewModel()
            AuthenticatedScreen(viewModel = viewModel) {
                LessonsScreen(
                    viewModel = viewModel
                )
            }
        }
    }
}
