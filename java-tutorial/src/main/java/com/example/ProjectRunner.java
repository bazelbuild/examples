package com.example;

import com.example.cmdline.LombokExample;

public class ProjectRunner {
    public static void main(String args[]) {
        Greeting.sayHi();
        LombokExample l = new LombokExample("Test");
        System.out.println(l.getName());
    }
}
