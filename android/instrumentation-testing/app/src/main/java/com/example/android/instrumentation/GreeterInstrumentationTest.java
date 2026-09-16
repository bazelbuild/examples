package com.example.android.instrumentation;

import static org.junit.Assert.assertEquals;

import androidx.test.runner.AndroidJUnit4;
import org.junit.Test;
import org.junit.runner.RunWith;

@RunWith(AndroidJUnit4.class)
public final class GreeterInstrumentationTest {
    @Test
    public void testDefaultGreeting() {
        assertEquals("Hello, Android!", new Greeter().greet(null));
    }

    @Test
    public void testNamedGreeting() {
        assertEquals("Hello, Bazel!", new Greeter().greet(" Bazel "));
    }
}
