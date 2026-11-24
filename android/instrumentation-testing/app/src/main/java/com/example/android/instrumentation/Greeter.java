package com.example.android.instrumentation;

public final class Greeter {
    public String greet(String name) {
        String normalizedName = name == null ? "" : name.trim();
        if (normalizedName.isEmpty()) {
            return "Hello, Android!";
        }
        return "Hello, " + normalizedName + "!";
    }
}
