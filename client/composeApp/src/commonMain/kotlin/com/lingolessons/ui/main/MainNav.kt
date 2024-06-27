package com.lingolessons.ui.main

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import com.lingolessons.ui.common.AuthenticatedScreen
import com.lingolessons.ui.lessons.LessonsScreen
import com.lingolessons.ui.profile.ProfileScreen
import com.lingolessons.ui.study.StudyScreen

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
        modifier = Modifier.fillMaxSize().background(MaterialTheme.colors.background),
    ) {
        composable(route = AppScreen.Profile.name) {
            AuthenticatedScreen {
                ProfileScreen()
            }
        }
        composable(route = AppScreen.Study.name) {
            AuthenticatedScreen {
                StudyScreen()
            }
        }
        composable(route = AppScreen.Lessons.name) {
            AuthenticatedScreen {
                LessonsScreen()
            }
        }
    }
}
