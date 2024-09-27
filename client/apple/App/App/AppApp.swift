//
//  AppApp.swift
//  App
//
//  Created by Anthony Miceli on 25/9/2024.
//

import SwiftUI
import Cocoa

fileprivate var dataPath: URL {
    guard let appSupportPath = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first else {
        fatalError("Couldn't find application support directory!")
    }
    let bundleID = Bundle.main.bundleIdentifier ?? "com.lingolessons.client"
    return appSupportPath.appendingPathComponent(bundleID)
}

class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        print("launching...")
        let task = Task {
            let domain: Domain
            do {
                let path = dataPath.path
                try FileManager.default.createDirectory(at: dataPath, withIntermediateDirectories: true, attributes: nil)
                
                domain = try DomainBuilder()
                    .baseUrl(url: "http://10.0.2.2:8000/api/v1/")
                    .dataPath(path: dataPath.appendingPathComponent("app.db").path)
                    .build()
                
                let session = try await domain.getSession()
                if (session == Session.none) {
                    print("No session")
                    let sess = try await domain.login(username: "admin", password: "admin")
                    print("logged in")
                } else {
                    print("A session")
                }
            } catch {
                print(error)
            }
            return
        }
    }
}

@main
struct AppApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
