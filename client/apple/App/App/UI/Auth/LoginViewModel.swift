//
//  LoginViewModel.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 2/10/2024.
//

import Foundation

class LoginViewModel: BaseViewModel {
    @Published var username: String = ""
    @Published var password: String = ""
    
    func login() {
        Task {
            print("Logging in")
            do {
                try await domainState.domain?.login(username: username, password: password)
                print("Logged in")
            }
            catch {
                print("Error logging in \(error)")
            }
        }
    }
}
