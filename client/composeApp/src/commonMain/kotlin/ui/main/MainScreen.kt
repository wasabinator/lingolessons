package ui.main

import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material.Scaffold
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationDrawerItem
import androidx.compose.material3.PermanentDrawerSheet
import androidx.compose.material3.PermanentNavigationDrawer
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavHostController
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import getPlatform


@Composable
fun MainScreen(
    navController: NavHostController = rememberNavController()
) {
    val backStackEntry by navController.currentBackStackEntryAsState()
    val largeScreen = getPlatform().isLargeScreen()

    val currentRoute =
        derivedStateOf { backStackEntry?.destination?.route?.let { AppScreen.valueOf(it) } }

    if (largeScreen) {
        Scaffold {
            PermanentNavigationDrawer(
                drawerContent = {
                    DrawerNavigation(
                        currentScreen = currentRoute.value ?: AppScreen.Profile,
                        onClick = { navController.navigate(it.name) }
                    )
                }
            ) {
                MainNav(navController = navController)
            }
        }
    } else {
        Scaffold(
            bottomBar = {
                BottomNavigationBar(
                    currentScreen = currentRoute.value
                        ?: AppScreen.Profile,
                    onClick = { navController.navigate(it.name) }
                )
            }
        ) {
            MainNav(navController = navController)
        }
    }
}

@Composable
fun DrawerNavigation(
    currentScreen: AppScreen,
    onClick: (AppScreen) -> Unit
) {
    PermanentDrawerSheet(modifier = Modifier.width(240.dp)) {
        NavigationDrawerItem(
            icon = { },
            label = { Text("Profile") },
            selected = currentScreen == AppScreen.Profile,
            onClick = { onClick(AppScreen.Profile) },
            modifier = Modifier.padding(horizontal = 12.dp)
        )
        NavigationDrawerItem(
            icon = { },
            label = { Text("Study") },
            selected = currentScreen == AppScreen.Study,
            onClick = { onClick(AppScreen.Study) },
            modifier = Modifier.padding(horizontal = 12.dp)
        )
        NavigationDrawerItem(
            icon = { },
            label = { Text("Lessons") },
            selected = currentScreen == AppScreen.Lessons,
            onClick = { onClick(AppScreen.Lessons) },
            modifier = Modifier.padding(horizontal = 12.dp)
        )
    }
}

@Composable
fun BottomNavigationBar(
    currentScreen: AppScreen,
    onClick: (AppScreen) -> Unit
) {
    NavigationBar {
        NavigationBarItem(
            selected = currentScreen == AppScreen.Profile,
            onClick = { onClick(AppScreen.Profile) },
            icon = { },//Icon(imageVector = Icons.Default.Face, contentDescription = null) },
            label = { Text("Profile") }
        )
        NavigationBarItem(
            selected = currentScreen == AppScreen.Study,
            onClick = { onClick(AppScreen.Study) },
            icon = { },//Icon(imageVector = Icons.Default.Face, contentDescription = null) },
            label = { Text("Study") }
        )
        NavigationBarItem(
            selected = currentScreen == AppScreen.Lessons,
            onClick = { onClick(AppScreen.Lessons) },
            icon = { },//Icon(imageVector = Icons.Default.Face, contentDescription = null) },
            label = { Text("Lessons") }
        )
    }
}
