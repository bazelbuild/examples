package com.example.android.bazel

import com.google.common.truth.Truth
import org.junit.Test
import org.robolectric.RobolectricTestRunner
import org.junit.runner.RunWith
import org.robolectric.Robolectric
import org.robolectric.Shadows.shadowOf
import android.content.Intent
import org.robolectric.RuntimeEnvironment
import android.app.Activity
import android.view.View
import org.junit.Assert.assertEquals

@RunWith(RobolectricTestRunner::class)
class WelcomeActivityTest {

    @Test
    fun clickingLogin_shouldStartLoginActivity() {
        Robolectric.buildActivity(WelcomeActivity::class.java).use { controller ->
            controller.setup() // Moves Activity to RESUMED state
            val activity: Activity = controller.get()
            activity.findViewById<View>(R.id.login).performClick()
            val expectedIntent = Intent(activity, LoginActivity::class.java)
            val actual: Intent = shadowOf(RuntimeEnvironment.application).getNextStartedActivity()
            assertEquals(expectedIntent.getComponent(), actual.getComponent())
        }
    }
}
