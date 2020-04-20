package com.example;

import org.junit.Assert;
import org.junit.Test;

public class TestJava {
    @Test
    public void simpleTest() {
        // this test will pass
        Greeting greeting = new Greeting();
        Assert.assertTrue(greeting.returnTrue());
    }

    @Test
    public void secondTest() {
        Greeting greeting = new Greeting();
        // this line will fail
        Assert.assertFalse(greeting.returnTrue());
    }
}
