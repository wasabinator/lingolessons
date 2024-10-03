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
#if canImport(AppKit)
    let appDelegate = NSApplication.shared.delegate
#elseif canImport(UIKit)
    let appDelegate = UIApplication.shared.delegate
#endif
    if let domainState = (appDelegate as? AppDelegate)?.domainState {
        return domainState
    } else {
        fatalError("Exception: AppDelegate was nil")
    }
}

class AppDelegate: NSObject {
    private static let logger = Logger(
        subsystem: bundleID,
        category: String(describing: AppDelegate.self)
    )
    
    var domainState: DomainState
    
    override init() {
        do {
            try FileManager.default.createDirectory(at: dataPath, withIntermediateDirectories: true, attributes: nil)
            
            let domain = try DomainBuilder()
                .baseUrl(url: "http://10.0.2.2:8000/api/v1/")
                .dataPath(path: dataPath.appendingPathComponent("app.db").path)
                .build()
            
            domainState = DomainState(domain: domain)
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
        super.init()
    }
}

#if canImport(AppKit)
extension AppDelegate: NSApplicationDelegate {
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        print("Launched")
//        initDomain()
    }
}
#elseif canImport(UIKit)
extension AppDelegate: UIApplicationDelegate {
//    func application(_: UIApplication, didFinishLaunchingWithOptions _: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
//        initDomain()
//        return true
//    }
}
#endif
