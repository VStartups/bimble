
# Welcome to **Bimble** - The Friendly C-Like Language for Beginners

**Bimble** is a beginner-friendly programming language designed to provide the power of C without its complexity. If you find traditional C a bit overwhelming, Bimble offers a simpler, cleaner approach to programming that helps you learn core concepts quickly.

---

## 🚀 Why Bimble?

- **Powerful Yet Simple**: C’s capabilities with an easy-to-read, beginner-friendly syntax.
- **Human-Readable Code**: Clean, straightforward syntax that’s easy to understand and maintain.
- **Dynamic Variables**: Easily incorporate variables within strings for readable and interactive outputs.
- **Tailored for Learners**: Especially for those new to programming or transitioning from other languages.

---

## 🌟 Key Features

### 1. **Variable Declaration with `may`**
Bimble simplifies variable declaration with the `may` keyword.

```bimble
may name = ""
```

- Variables are declared dynamically using `may`, initializing an empty string or other types (int, float, etc.).

### 2. **User-Friendly Output with `echoln`**
No more complex `printf` syntax! Bimble uses `echoln` to print to the console, automatically adding a new line.

```bimble
echoln("Hello, World!")
```

Output:

```
Hello, World!
```

#### 💡 Embedding Variables:
Bimble allows seamless embedding of variables into your strings using the `$` symbol:

```bimble
may name = "Alice"
echoln("Hello, $name!")
```

Output:

```
Hello, Alice!
```

### 3. **Input Handling with `takein`**
Gathering user input is incredibly simple with the `takein` command:

```bimble
may name = ""
echoln("What is your name?")
takein(name)
echoln("Nice to meet you, $name!")
```

- Asks for input, stores it in `name`, and prints a friendly greeting.

### 4. **Combining Variables & Strings**
You can combine text and variables naturally for dynamic output:

```bimble
may age = 25
echoln("You are $age years old.")
```

---

## 🎯 Quickstart Guide

### 1. **Clone the Repository**
```bash
git clone https://github.com/yourusername/bimble.git
```

### 2. **Build the Project**
Ensure you have the necessary dependencies installed and run:
```bash
cd bimble
./build.sh
```
- then find the binary for linux named ``bimble-linux`` and for windows ``bimble-windows.exe`` inside bin folder
` make a new file in name it anything and it shall end with ``.bb`` (standing for bimble) and put a code derived from following in it :
```
may name = ""
echoln("Please tell me your name")
takein(name)
echoln("HI $name")
```
## **try altering it for example ask for age**
### 3. **Run Your First Bimble Program**
```bash
./bimble file_name.bb
```
- then you can run your compiled program by simply typing ``./t`` on linux terminal in the same dir or ``./t.exe`` on windows in the same dir  

Write your Bimble code and start programming!

---

## 🛠 Example Program

Here’s a sample Bimble program that interacts with the user:

```bimble
may name = ""
echoln("What is your name?")
takein(name)
echoln("Hello, nice to meet you $name")
```

- This simple code asks for the user's name and responds with a personalized greeting.

---

## 🤝 Contributing to Bimble

We welcome contributions! Whether it’s improving documentation, fixing bugs, or suggesting new features, your input is valuable. Here’s how you can contribute:

1. **Fork the Project**: Make a copy to start working.
2. **Create a Branch**: Work on your changes in a new branch.
3. **Submit a Pull Request**: Let us review your contributions!

---

## 🙌 Improving This `README.md`

Any suggestions for improving this `README.md` are greatly appreciated! Whether it’s grammar fixes, formatting tweaks, or larger structural improvements, feel free to contribute or share feedback.

---

## 📄 License

This project is licensed under the [GNU General Public License V3](LICENSE).

---

### 📢 Stay Updated

Be sure to check back regularly for updates, new features, and improvements. Bimble is growing, and your feedback helps make it even better!

Happy coding with **Bimble**! 😊