package com.lingolessons.ui.profile

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import org.koin.compose.viewmodel.koinViewModel
import org.koin.core.annotation.KoinExperimentalAPI

@OptIn(KoinExperimentalAPI::class)
@Composable
fun ProfileScreen(
    viewmodel: ProfileViewModel = koinViewModel(),
) {
    ProfileScreen(
        logout = viewmodel::logout
    )
}

@Composable
fun ProfileScreen(
    logout: () -> Unit
) = Column {
    Text("profile")

    Button(
        onClick = logout,
    ) {
        Box(modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp)) {
            androidx.compose.material.Text("Logout")
        }
    }
}
