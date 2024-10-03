//
//  LoginView.swift
//  LingoLessons
//
//  Created by Anthony Miceli on 2/10/2024.
//

import SwiftUI

struct LoginView: View {
    @EnvironmentObject var viewModel: LoginViewModel
    
    var body: some View {
        VStack {
            VStack {
                TextField(
                    "Username",
                    text: $viewModel.username
                )
                .disableAutocorrection(true)
                .padding(.top, 20)
                
                SecureField(
                    "Password",
                    text: $viewModel.password
                )
                .padding(.top, 5)
            }
            
            Button(
                action: viewModel.login,
                label: {
                    Text("Login")
                        .font(.system(size: 24, weight: .bold, design: .default))
                        .frame(maxWidth: .infinity, maxHeight: 60)
                        .foregroundColor(Color.white)
                        .background(Color.blue)
                        .cornerRadius(10)
                }
            )
            .padding(.top, 20)
            
            Spacer()
        }
        .frame(maxWidth: 200)
        .padding(30)
    }
}
