//
//  AppApp.swift
//  App
//
//  Created by Anthony Miceli on 25/9/2024.
//

import SwiftUI

#if canImport(AppKit)
import AppKit
#elseif canImport(UIKit)
import UIKit
#endif

@main
struct LingoLessonsApp: App {
#if canImport(AppKit)
    @NSApplicationDelegateAdaptor(AppDelegate.self)
#elseif canImport(UIKit)
    @UIApplicationDelegateAdaptor(AppDelegate.self)
#endif
    private var appDelegate
    
    @ObservedObject var router = Router()
    
    var body: some Scene {
        WindowGroup {
            NavigationStack(path: $router.navPath) {
                router.view(for: .home)
                    .navigationDestination(for: Router.Destination.self) { destination in
                        router.view(for: destination)
                    }
            }
        }
    }
}
