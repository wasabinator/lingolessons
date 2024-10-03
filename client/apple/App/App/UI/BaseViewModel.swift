//
//  BaseViewModel.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 2/10/2024.
//

import Foundation

class BaseViewModel: ObservableObject {
    var domainState: DomainState
    
    init(domainState: DomainState) {
        self.domainState = domainState
    }
}
