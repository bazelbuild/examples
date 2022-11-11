package com.example.restaurant;

import com.example.dishes.MacAndCheese;
import com.example.dishes.Pizza;
import java.util.HashMap;

public class Chef {
    
    public HashMap<String,String> listDishes() {
        HashMap<String,String> dishes = new HashMap<String,String>();

        dishes.put(MacAndCheese.DISH_NAME,MacAndCheese.DESCRIPTION);
        dishes.put(Pizza.DISH_NAME,Pizza.DESCRIPTION);


        return dishes;
    }
}
