package com.example;

import com.example.Calculator;
import org.junit.Test;

import static org.junit.Assert.assertEquals;

public class RunnerTest {
    @Test
    public void evaluatesExpression() {
        Calculator calculator = new Calculator();
        int sum = calculator.add(4,2);
        assertEquals(6, sum);
    }
}
