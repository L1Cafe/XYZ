# C

General-purpose, weakly-typed, imperative, low-level programming language. It is one of the most used languages according to [GitHut](https://githut.info/), with the eighth position in the chart when sorted by "active repositories".

It was designed with a strong focus on Unix, but is considered completely cross-platform nowadays, with compilers for macOS, Windows and even embedded devices or system drivers.

It features a **very small number of keywords**, low-level memory management through pointers and allocation, and a strong and healthy developer community.

C is also infamous for being a language with a lot of [*undefined behaviour*](https://en.wikipedia.org/wiki/Undefined_behavior). These situations arise when a programming language executes a certain computer code that is not well-defined by the language specification. A good example could be, for example, allocating portions of the system's memory and immediately trying to read them. Because it is not specified whether or not the compiler or the OS should erase the reserved memory, and the case is often that memory will not be cleared for performance reasons, this operation could end up reading memory leftovers from other applications. Writing code relying on undefined behaviour can introduce a plethora of issues ranging from readability and portability problems to even security vulnerabilities or application crashes and it is the sole responsibility of the programmer to never rely on undefined behaviour when writing code, because most simple compilers are unable to check for these issues.

## Pros

- Low level access to system internals
- Suited for realtime applications
- Lots of documentation and libraries readily available
- General purpose

## Cons

- Memory management is complex
- Too much undefined behaviour
- Syntax and coding style can vary wildly between different programmers

# C++

Created as an extension of the C programming language. It adds object-oriented programming to the already familiar functional features of C as well as retaining low-level memory access. It is designed with a strong focus on embedded devices, where efficiency is a requirement, but it has also been successfully applied in other contexts such as desktop applications and network-connected servers.

It also adds anonymous functions (lambda functions) to the C standard. The C++ standards don't dictate implementation of name mangling or exception handling, but the disadvantage is that the object code produced by different compilers is typically incompatible.

Most of the advantages and drawbacks of C still hold valid for C++. One of the most notable differences is that C++ is significantly more difficult to anaylse and parse due to the added complexity of objects and new reserved words.

## Pros

- Most C code can be made to compile with C++ compilers with little to no modification
- It's as popular as C (or even more popular)
- The Microsoft ecosystem has extensive documentation in regards to C++ applications for Windows
- C++ is very popular for desktop apps in all major operating systems

## Cons

- It has received [harsh criticism](https://wiki.theory.org/index.php/YourLanguageSucks#C.2B.2B_sucks_because) in regards to its design and complexity

# BASIC

Its design precedes C. Its goal was [letting non-STEM students write their own programs](https://en.wikipedia.org/wiki/BASIC) for computers, which at the time required custom programming, such as the Atari or Commodore computers, but it became obsolete when other programming languages gained popularity, and modern computers with greater capabilities became available.

Despite being fundamentally focused on rudimentary 8 bit computers, it is high level enough to be much more understandable than other languages, even without prior experience in programming, because it uses simple English keywords instead of complex assembly code or syntax symbols, which were the norm at its time of introduction.

## Pros

- Designed to be easy to understand for non-expert programmers, high-level
- Relatively widespread and still in use today in softwares like office suites or calculators

## Cons

- Large list of reserved words
- Generally, not powerful enough to write low-level software easily
- Non-structured, can lead to confusion if the programmer is not an expert and overuses GOTO clauses

# Haskell

A strongly-typed, purely functional programming language. It sports lazy evaluation and type polymorphism, among other features.

It places a strong emphasis on [referential transparency](https://en.wikipedia.org/wiki/Referential_transparency), but is also lenient enough to allow the programmer to make concessions in aspects that break RT, like I/O.

It is used by many tools and corporations such as Linspire, Pandoc, Xmonad, Facebook, GitHub, or seL4, a formally verified microkernel.

The most well-known Haskell implementation is the Glasgow Haskell Compiler, which compiles to native code on many processor architectures and even ANSI C.

## Pros

- Great for handling functional problems
- It is the declarative programming language par excellence

## Cons

- Its fundamental design foundation makes it difficult for non-expert programmers to use Haskell
- Lazy evaluation makes it much more difficult to debug memory leaks

# Java

General-purpose, object-oriented programming language. First appeared in 1995. Its design philosophy focuses on a Write Once, Run Anywhere approach with which programmers can write reasonably low-level code but still be sure that their code would run anywhere the Java Virtual Machine (JVM) is ported to. Java has similarities with C in terms of syntax, but the biggest difference is with memory management, because Java implements a garbage collector, and therefore does away with manual memory management, as the runtime decides when and how memory should be assigned and reclaimed automatically.

Nowadays, Java is a very widespread foundation with many different architecture targets ranging from regular x86-64 desktop computers, to ARM mobile phones (Android uses Java heavily through the Dalvik VM), or even embedded devices such as card readers and even smart-cards.

Java is often criticised for its lack of speed when compared to C, but Java advocates claim Java can be faster than C if an expert Java programmer, when writing the software, is aware of the Java environment, and how it handles Just-In-Time compilation or Garbage Collection, as without Garbage Collection, a regular C program would have to optimally handle memory in such a way that it uses only CPU cycles when idle, which is often not the case, and requires an even deeper understanding of the underlying hardware intricacies when writing C applications.

Modern Java versions include advanced features such as Lambda Functions, and the rich Java community has brought many Java-first development tools such as complete IDEs built around Java, software testing suites, or MVP web development frameworks and ORM interfaces for several databases.

## Pros

- Reasonably easy to learn, because memory management is managed automatically
- Natural transition for C programmers due to syntax similarity
- Most applications require little to zero modification to be ported to a different platform architecture
- Extremely widespread usage
- Under active development, and each new version brings many (newer) programming concepts

## Cons

- Despite being implemented in resource-limited environments such as smart-cards, the generic JVM is relatively heavy
- Its advanced features are a double-edged sword because, despite making it easier to handle modern programming concepts, it also introduces a steeper learning curve for newcomers
- Its fast development and short release cycles implies there's a high fragmentation between the different Java versions, with the latest version being Java SE 13, but the oldest currently supported version at the time of writing dating back to Java SE 8, often being the lowest common denominator for Java software and even back to Java SE 6 for extended support.
- It is overly verbose, which, althought might look like an advantage for readability, also means that writing code is much slower than in other languages with shorter built-in functions
- Oracle is the owner of Java, and it imposes a restrictive license on Java code which has lead to [legal disputes](https://en.wikipedia.org/wiki/Google_v._Oracle_America) with Google in regards to Android, in stark contrast to its previous owner: Sun Microsystems which were usually seen as friendly towards open source projects

# JavaScript

Despite having "Java" in its name, it is not related to the Java Programming language in any other meaningful way beyond loosely sharing some common programming concepts like Object-Oriented programming or Garbage Collection-assisted memory management.

It is a high-level, interpreted scripting language, which is one of the backbones of web technologies, alongside CSS and HTML, and has influenced modern programming languages, such as JSON serialisation, which has replaced XML for the most part in modern applications.

Nowadays, JavaScript is not a web-exclusive language, as, thanks to the work of projects like Node.js, it can now be used to write server-side applications, thus facilitating traditionally front-end web developers to assist in also writing back-end services.

## Pros

- Same language for many different environments
- High-level enough for the programmer to not need to be concerned with system internals
- Modern web browsers sandbox JavaScript and thus make platform exploitation difficult

## Cons

- Until very recently, JS grew organically, and is therefore not well designed from the beginning, but rather, became refined over time
- Although it is cross-platform, the most used JS-based frameworks rely on the Chrome engine which is well-known for being taxing on system resources like memory and processor

# Lisp

Lisp isn't a language in itself, but a family of programming languages characterised by the usage of parenthesised prefix notation. It is the second oldest high-level programming language family (Fortran being older by one year). The most well-known dialects are: [Clojure](https://en.wikipedia.org/wiki/Clojure), [Common Lisp](https://en.wikipedia.org/wiki/Common_Lisp) and [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)). It was originally created as a practical mathematical notation for computer programs. Lisp algorithms can usually be expressed in both imperative and functional styles.

The fundamental data structure of Lisp is the list, and Lisp offers macros that allow you to dynamically extend the langage beyond the designer's boundaries. Lisp is often considered a "programmable programming language", meaning the programmer is free to add features they deem desirable without much hassle.

In particular, Clojure is the Lisp dialect which places the most emphasis on immutability and draws considerable influences from Haskell, making it the closest to a purely functional programming language.

## Pros

- Extremely flexible, allows the programmer to have full control on the software they write
- It is very simple in design: Lisp code is simply a list of symbols

## Cons

- Lisp makes it difficult to easily identify language constructs
- It's not very widespread, so documentation and help for obscure errors might be difficult to find

# Pascal

Imperative programming language introduced in 1970 with the focus of getting programmers to acquire good programming practices such as [structured programming](https://en.wikipedia.org/wiki/Structured_programming) and [data structuring](https://en.wikipedia.org/wiki/Data_structure). It was based on the ideas of ALGOL 60. It became very successful upon its introduction, most notably on the minicomputer market, and began to be used in 1980s as a teaching language in many universities. C would eventually replace it.

Pascal, being a small and relatively simple programming language doesn't have [many reserved words](https://wiki.freepascal.org/Reserved_words), making it easier to analyse syntactically.

## Pros

- Encourages good programming practices
- It is small in size and efficient

## Cons

- Although Object Pascal exists, it is a separate project, as the original language does not include many newer programming paradigms
- There are barely any companies or projects still using Pascal or any of its derivatives nowadays
- Early criticism had the release of Extended Pascal as a consequence, but it didn't gain much traction

# PHP

It's mainly a server-side scripting programming language for the web.

## Pros

- Very big and active developer community make it easier to get started quickly, even without prior programming experience
- Very straightforward to develop and integrate web technologies with PHP

## Cons

- Like JavaScript, it suffers from organic growth which has negatively impacted its design, in regards, for example, but not limited to: function naming conventions, inconsistent function argument positions, etc.
- Also because it grew organically, most developers are using old and vulnerable versions of PHP
- Since it is relatively low level, it does not enforce conventions and mechanisms that make a web app safer, like ORM or MVP paradigms, that are part of frameworks like Ruby on Rails, which then therefore leads to bad coding practices by junior programmers

# Prolog

Logic, declarative, specific-purpose programming language. It has traditionally been an untyped language, but there have been attempts to extend Prolog with types.

## Pros

- Building knowledge databases is natural to Prolog thanks to the existence of what's called `facts`
- Collecting all the solutions that match a certain criteria is also very straightforward thanks to higher-order programming
- Aids greatly in the resolution of logic problems

## Cons

- Using a logic paradigm, novice programmers with no experience in formal logic might struggle to grasp core Prolog concepts more so than, for example, if a C programmer wants to learn Java
- Due to its nature, it is harder to interact with I/O or system internals

# Python

General-purpose, high-level, interpreted, object-oriented programming language.

Its syntax is whitespace based, so that code blocks must be indented adequately in order to be syntactically valid. It features a strong and active developer community, especially among data scientists and the cybersecurity industry, due to its ease of use and plethora of libraries and related projects.

## Pros

- The Garbage Collector frees the developer of having to manage the system memory
- Python sports a fairly complete Object-Oriented paradigm including inheritance, polymorphism and other OOP features

## Cons

- There's no concept of `switch` statement, which means that, to achieve the same effect, it is necessary to use nested `if` and `elif` clauses
- The fast paced Python release cycle has caused that Python 2.x, that has been retired, is still in use by some companies

# Lessons learned

- Garbage collectors free the programmer from the task of manually allocating and freeing memory, but come with performance drawbacks as applications will need a runtime.
- Garbage collection solves most issues where I/O or data storage is involved but they often add computational overhead to the runtime in terms of space and time.
- A clear, verbose syntax requires more lines of code for the same functionality in other languages, but simplifies the compiler and parser.
- There's no correlation between community size and how well designed a language is.
- A rapid development cycle typically leads to developers sticking to outdated versions, which may impact the security of applications running on a certain version once security patches are no longer delivered.
- High-level scripting languages that run in virtual machines are easier to port to other architectures, as you only need to reimplement the VM, Python is a good example of this.
- Declarative programming is closer to how humans reason, and imperative programming is closer to how machines work.
