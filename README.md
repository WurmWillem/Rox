<H1> Rox: A Refreshingly Simple And Dutch Programming Language </H1>

<H2>Introducing Rox 🦀</H2>r
This is Rox, my implementation of Lox in Rust with elegant Dutch keywords. The goal of Rox is to spread the beautiful Dutch language to as many people as possible.

<H2>Why should you use Rox?</H2>

- 🇳🇱 Je bent een Nederlander in hart en nieren
-  🚀 Blazingly fast and written in Rust  
- 😃 Because you are a masochist
- 🥰 You want to support me


<H3>Sneak Peek</H3>

```c
// print with 'zeg' (print without newline) en 'roep' (print with newline)
zeg "Hallo ";   // prints "Hallo "
roep "Wereld!"; // prints "Wereld!" with newline

// declare variables with 'laat'.
laat getal = 1 + 2 * 3; // getal holds 7

// declare if statements with `als`, `anders als`, and `anders`
// the indentation is purely for readability purposes
laat variabele = -3.23;
// prints "negatief nummer"
als variabele < 0
  roep "negatief nummer";
anders
  roep "positief nummer";

// declare while loops with 'terwijl condition'
laat i = 0;
// prints 0-9 (note that 10 is not included)
terwijl i < 10 {
  roep i;
  i = i + 1;
}

// declare for loops with 'voor x van lower bounds tot upper bounds'
// prints 0-9 (note that 10 is not included)
voor i van 0 tot 10
  roep i;

// declare functions with 'proces'
proces keerDrie(n) {
  geef n * 3;     // return a value with 'geef'
}
roep keerDrie(5); // prints 15

```

<H2>Features</H2>

- 🇳🇱 Beautiful Dutch keywords  (wellus, nietus, terwijl, ...)
- 🦀 Memory safe, as it is written in 100% safe Rust 
- 💵 Poor type system
  - Null type: `niks`
  - Booleans: `wellus` and `nietus`
    supported operators: &&, ||, !, <, >, <=, >=, ==, !=
    ```c
    roep wellus || nietus;    // prints wellus
    roep wellus == !!!nietus; // prints wellus
    roep 3 < 5;               // prints wellus
    ```
  - Numbers
    - 64-bit floating point
    - Supported operators are +, -, *, /, ^ (raises a number to a given power, e.g. 2^3 = 8)

  - Strings
    - The only supported operator is '+'.
      ```c
      roep "Hallo " + "Wereld!"'               // prints "Hallo Wereld!"
      roep "Ik heb al " + 17 + " kokosnoten."; // prints "Ik heb al 17 kokosnoten!!"
      ```
- Control flow with `als` statements
  ```c
  als leeftijd < 18
    roep "Je zit op school";
  anders als leeftijd < 65
    roep "Je bent werkende.";
   anders
    roep "Je bent echt megaoud lol🤣."; // yes, emojis are supported.
  ```
- Functions
  ```c
  // function that returns the fibonacci number of its argument in an extremely inefficient manner
  proces fib(n) {
    als n <= 1 {
      geef n;
    }
    geef fib(n - 1) + fib(n - 2);
  }
  roep fib(6);  // prints 8
  ```
- Lists
  ```c
    // prints every element in the list
    laat lijst = [12, -2.3, 3 * 7, "hey"];

    voor i van 0 tot lengte(lijst)
      roep lijst[i];
  ```

<H2>How do I use Rox?</H2>
You can either download the source code or ask me for a binary and I'll send you an executable for your platform.


- Create a file called "file.rox" in the same directory as the project and execute the command cargo run --release without additional arguments. This will give you full access to Rox, statements included.
  ```rust
  cargo run --release
  ```
  
- Or execute it with the command cargo run <"expression"> as such. This will only work for expressions, not statements.
  ```rust
  cargo run "1 + 1"
  // prints 2
  ```


<H2>Roadmap</H2>

- Expand standard library
- Add Arrays
- Add enums
- Add more string related operations on strings
- Add some syntactic sugar for mutating variables (+=, -=, *=, /=, ++, --, ..)
- Add break and continue statements
- ✅ Add functions
- ✅ Add loops
- ✅ Add if statements
- ✅ Add variable scope
- ✅ Add (nested) multi-line comments with /* */
- ✅ Add variables
- ✅ Make '+' work on combinations ofstrings and integers, e.g. "oppervlakte =" + 5 == "oppervlakte = 5" 
- ✅ Add '^' operator, e.g. 2^3 == 8
