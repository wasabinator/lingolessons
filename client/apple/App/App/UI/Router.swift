//
//  Router.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 2/10/2024.
//

import SwiftUI

final class Router: ObservableObject {
    public enum Destination: Codable, Hashable {
        case login
        case study(lessonId: String)
    }
    
    @Published var navPath = NavigationPath()
    
    private lazy var domainState: DomainState = {
        LingoLessons.domainState()
    }()
    
    @ViewBuilder func view(for destination: Destination) -> some View {
        switch destination {
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
