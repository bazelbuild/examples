package com.example.cmdline;

// import com.example.Greeting;

public class Runner {
   public static void main(String args[]) {
     // Greeting.sayHi();
     LombokExample l = new LombokExample("Test");
     System.out.println(l.toString());
   }
}
