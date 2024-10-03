//
//  DomainState.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 29/9/2024.
//

import Combine

class DomainState {
    public let session = CurrentValueSubject<Bool?, Never>(nil)
    public var domain: Domain? = nil

    func attach(domain: Domain) {
        self.domain = domain
        refresh()
    }
    
    func refresh() {
        print("refreshing session state")
        assert(self.domain != nil, "Attempt to call refresh() on an uninitialised domain")
        Task {
            print("Begin refresh session")
            do {
                let session = try await self.domain!.getSession()
                print("Refreshed session: \(session)")
                switch session {
                case .authenticated(let username):
                    self.session.send(true)
                case .none:
                    self.session.send(false)
                }
            } catch {
                // No errors here are currently recoverable
                switch (error) {
                case DomainError.Api(let message):
                    print("Domain API error: \(message)")
                case DomainError.Database(let message):
                    print("Domain DB error: \(message)")
                case DomainError.Unexpected(let message):
                    print("Domain unclassified error: \(message)")
                default:
                    print("Non domain failed init")
                }
            }
        }
    }
}
