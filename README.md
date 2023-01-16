Compiler Construction
=

Issues
-

1. Exception java.lang.UnsatisfiedLinkError when trying to run Nand2Tetris tools on Ubuntu 22.04

  ```sh
  ~/Desktop/nand2tetris/tools/HardwareSimulator.sh

  // Error
  Exception in thread "main" java.lang.UnsatisfiedLinkError: Can't load library: /usr/lib/jvm/java-11-openjdk-amd64/lib/libawt_xawt.so
  ```
  - [Fix: install `openjdk-11-jdk`](https://stackoverflow.com/questions/67391845/exception-java-lang-unsatisfiedlinkerror-when-trying-to-open-allure-reports-in-w)

  ```
  sudo apt install openjdk-11-jdk
  ```

  # compiler-construction
