//
//  AppDelegate.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 29/9/2024.
//

import OSLog

#if canImport(AppKit)
import AppKit
#elseif canImport(UIKit)
import UIKit
#endif

let bundleID = Bundle.main.bundleIdentifier ?? "com.lingolessons.client"

fileprivate var dataPath: URL {
    guard let appSupportPath = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first else {
        fatalError("Couldn't find application support directory!")
    }
    return appSupportPath.appendingPathComponent(bundleID)
}

func domainState() -> DomainState {
    return state
}

private let state = DomainState()

class AppDelegate: NSObject {
    private static let logger = Logger(
        subsystem: bundleID,
        category: String(describing: AppDelegate.self)
    )
    
    private func initDomain() {
        print("Init domain")
        do {
            print("App Delegate init")
            try FileManager.default.createDirectory(at: dataPath, withIntermediateDirectories: true, attributes: nil)
            
            print("Creating domain")
            let domain = try DomainBuilder()
                .baseUrl(url: "http://10.0.2.2:8000/api/v1/")
                .dataPath(path: dataPath.appendingPathComponent("app.db").path)
                .build()
            
            print("Setting domain state")
            state.attach(domain: domain)
        } catch {
            // No errors here are currently recoverable
            switch (error) {
            case DomainError.Api(let message):
                Self.logger.error("Domain API error: \(message)")
            case DomainError.Database(let message):
                Self.logger.error("Domain DB error: \(message)")
            case DomainError.Unexpected(let message):
                Self.logger.error("Domain unclassified error: \(message)")
            default:
                Self.logger.error("Non domain failed init")
            }
            fatalError("Failed to initialise the app")
        }
    }
}

#if canImport(AppKit)
extension AppDelegate: NSApplicationDelegate {
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        initDomain()
    }
}
#elseif canImport(UIKit)
extension AppDelegate: UIApplicationDelegate {
    func application(_: UIApplication, didFinishLaunchingWithOptions _: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        initDomain()
        return true
    }
}
#endif
