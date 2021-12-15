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

    @Test
    public void evaluatesExpressionRight() {
        Calculator calculator = new Calculator();
        int sum = calculator.add(7,2);
        assertEquals(9, sum);
    }
}
