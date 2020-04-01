# Ketamine
Embeddable simplistic dynamic scripting language

## Types
- integer  
  `integer = 1;`
- float  
  `float = 3.14;`
- boolean  
  `boolean = true;`
- string  
  `string = "hello world!";`
- array  
  `array = [1, 2, 3];`
- object  
  `object = { key: "value" };`
- function  
  ```
  fib = function(n) {
      if (n < 3) {
          1
      } else {
          fib(n - 2) + fib(n - 1)
      }
  };
  ```

## Control flow
- If  
  ```
  age = if (person.age < 15) {
      "child"
  } else if (person.age < 18) {
      "adolescent"
  } else {
      "adult
  };
  ```
- for
  ```
  for (person in people) {
      print("Hello", person.first_name);
  };
  ```
- while
  ```
  result = while (true) {
      next = try_again();
      if (next != null) {
          break next;
      };
  }
  ```

## Features
- embeddable & extendable  
  ```rust
  fn abs(this: i64, args: Vec<Value>) -> Result<Value, String> {
      Ok(Value::Integer(this.abs()))
  }
  
  interpreter.prototype_function("abs", abs);
  assert_eq!(interpreter.eval("-10.abs()").unwrap() == Value::Integer(10));
  ```
- first-class functions
- implicit `return`  
  ```
  with_return    = function() { return 1; };
  without_return = function() { 1 };
  ```
- implicit `this`  
  ```
  counter = {
      count: 0,
      increment: function() {
          this.count = this.count + 1;
      }
  };
  counter.increment();
  ```
- extend types using prototypes  
  ```
  $integer.abs = function() {
      if (this < 0) {
          -this
      } else {
          this
      }
  }
  ```
- range expressions  
  ```
  for (x in 0..10) { ... };
  ```