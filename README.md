<H1> Rox: A Refreshingly Simple And Dutch Programming Language </H1>

<H2>Introducing Rox 🦀</H2>
This is my implementation of Lox, a programming language so simple and Dutch that even your little sister can program in it. As of right now the language (if you can even call it that) is extremely bare-bones, but it is improving rapidly. The goal of Lox is not to create a fully-functioning language for commercial use; it is simply the hobby project of a mediocre Rust programmer.

<H2>Why should you use Rox?</H2>

- 🇳🇱 Je bent een Nederlander in hart en nieren
-  🚀 Blazingly fast and written in Rust  
- 😃 Because you are a masochist
- 🥰 You want to support me

<H2>Features</H2>

- Beautiful Dutch keywords 🇳🇱 (wellus, nietus, terwijl, ...)
- Memory safe, as it is written in 100% safe Rust
- Expressive expressions
  - Arithmetic with proper precedence rules (similiar to C-like languages)
    - Supported operators are +, -, *, /, ^ (raises a number to a given power, e.g. 2^3 = 8) 
      
    ```
    roep 1 + 2 * 3;
    output: 7

    roep "Hallo " + "Wereld!";
    output: Hallo Wereld!
        
    roep "average = " + (1 + 3) / 2;
    output: average = 15
    ```
  - Logic operators that can be chained
    - Supported logical operators are ==, >, <, >=, <= !=, !, //
    ```
    roep 5 == 2 + 3;
    output: wellus
  
    roep (3 + 2 * 3 == 3 * (2 + 1)) == (8 >= 4);
    output: wellus
    ```
- Dynamically typed mutable variables
  ```
  laat breedte = 3;
  laat lengte = 5;

  roep "oppervlakte = " + breedte * lengte;
  output: oppervlakte = 15
  ```
- Control flow with if statements
  ```
  als variabele < 0
    roep "negatief nummer";
  anders als variabele > 0
    roep "positief nummer";
  anders
    roep "het nummer is 0";
  // The indentation is purely for readability reasons, Rox does not care about indentation.
  ```
  ```
  als leeftijd >= 18 en leeftijd < 65
    roep "Je bent een werkende volwassene.";
  anders als leeftijd < 18 of leeftijd >= 65
    roep "Je bent niet in de volwassen leeftijdsgroep.";
  ```
- While and for loops (both examples print the digits 0 up to and including 9)
  ```
  laat i = 0;
  terwijl x < 10 {
    roep x;
    i = i + 1;
  }
  ```
  ```
  voor i = 0 tot 10
    roep i;
  ```


<H2>How do I use Rox?</H2>
You can either download the source code or ask me for a binary and I'll send you an executable for your platform.


- Create a file called "file.lox" in the same directory as the project and execute the command cargo run without additional arguments. this will give you full access to Rox, statements included.
  ```
  cargo run
  ```
  
- Or execute it with the command cargo run "expression" as such. This will only work for expressions, not statements.
  ```
  cargo run "1 + 1"
  output: 2
  ```


<H2>Roadmap</H2>

- Add functions
- Add enums
- Remove null and replace it with Option<T> enum
- Add more string related operations on strings
- Add some syntactic sugar for mutating variables (+=, -=, *=, /=, ++, --)
- Add break and continue statements
- ✅ Add loops
- ✅ Add if statements
- ✅ Add variable scope
- ✅ Add (nested) multi-line comments with /* */
- ✅ Add variables
- ✅ Make '+' work on combinations ofstrings and integers, e.g. "oppervlakte =" + 5 == "oppervlakte = 5" 
- ✅ Add '^' operator, e.g. 2^3 == 8
