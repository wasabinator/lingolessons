//
//  LoginView.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 2/10/2024.
//

import SwiftUI

struct LoginView: View {
    @EnvironmentObject var viewmodel: LoginViewModel
    //@State private var viewModel: LoginViewModel
    
    //init(viewModel: LoginViewModel) {
    //    self.viewModel = viewModel
    //}
    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Login View!")
        }
        .padding()
    }
}
