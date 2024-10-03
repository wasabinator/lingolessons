//
//  LingoLessonsApp.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 25/9/2024.
//

import SwiftUI

@main
struct LingoLessonsApp: App {
    @ObservedObject var router = Router()
    
    var body: some Scene {
        WindowGroup {
            NavigationStack(path: $router.navPath) {
                ContentView()
                    .navigationDestination(for: Router.Destination.self) { destination in
                        router.view(for: destination)
                    }
            }
        }
    }
}
