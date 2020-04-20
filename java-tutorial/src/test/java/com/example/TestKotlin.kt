package com.example

import org.junit.Assert
import org.junit.Test

class TestKotlin {
    @Test
    fun simpleTest() {
        // pass
        Assert.assertTrue(Greeting().returnTrue())
    }

    @Test
    fun secondTest() {
        // fail
        Assert.assertFalse(Greeting().returnTrue())
    }

    @Test
    fun `thirdTest name with space`() {
        // pass
        Assert.assertTrue(Greeting().returnTrue())
    }
}

