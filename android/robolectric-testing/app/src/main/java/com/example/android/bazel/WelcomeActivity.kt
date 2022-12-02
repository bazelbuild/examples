package com.example.android.bazel

import android.app.Activity
import android.os.Bundle
import android.view.View
import android.content.Intent

class WelcomeActivity : Activity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContentView(R.layout.welcome_activity)

        val button: View = findViewById(R.id.login)
        button.setOnClickListener({ v ->
            startActivity(Intent(this@WelcomeActivity, LoginActivity::class.java))
        })
    }
}
