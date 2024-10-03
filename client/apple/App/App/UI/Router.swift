//
//  Router.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 2/10/2024.
//

import SwiftUI
import Combine

final class Router: ObservableObject {
    public enum Destination: Codable, Hashable {
        case home
        case login
        case study(lessonId: String)
    }
    
    @Published var navPath = NavigationPath()
    
    init() {
        print("Router init")
    }
    
    private lazy var domainState: DomainState = {
        print("getting domain state")
        let state = LingoLessons.domainState()
        self.subscription = state.session.sink {
            print("Session completion \($0)")
        } receiveValue: { value in
            print("Session state: \(value ?? false)")
            if (value == false) {
                print("Session state false!")
                self.navigate(to: .login)
            }
        }
        return state
    }()
    
    private var subscription: AnyCancellable? = nil
    
    @ViewBuilder func view(for destination: Destination) -> some View {
        let _ = domainState
        switch destination {
        case .home:
            ContentView()
        case .login:
            LoginView().environmentObject(LoginViewModel(domainState: self.domainState))
        case .study(let lessonId):
            LoginView().environmentObject(LoginViewModel(domainState: self.domainState))
        }
    }
    
    func navigate(to destination: Destination) {
        navPath.append(destination)
    }
    
    func navigateBack() {
        navPath.removeLast()
    }
    
    func navigateToRoot() {
        navPath.removeLast(navPath.count)
    }
}
