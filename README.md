[![Rust](https://github.com/gustawdaniel/asci-art-rust-node-js/actions/workflows/rust.yml/badge.svg)](https://github.com/gustawdaniel/asci-art-rust-node-js/actions/workflows/rust.yml)
[![Node.js CI](https://github.com/gustawdaniel/asci-art-rust-node-js/actions/workflows/node.js.yml/badge.svg)](https://github.com/gustawdaniel/asci-art-rust-node-js/actions/workflows/node.js.yml)

## The Goal

In stations and airports you often see this type of screen:

![](https://www.codingame.com/fileservlet?id=21824381272)

Have you ever asked yourself how it might be possible to simulate this display on a good old terminal? We have: with ASCII art!

## Rules

ASCII art allows you to represent forms by using characters. To be precise, in our case, these forms are words. For example, the word "MANHATTAN" could be displayed as follows in ASCII art:

```text
# #  #  ### # #  #  ### ###  #  ###
### # # # # # # # #  #   #  # # # #
### ### # # ### ###  #   #  ### # #
# # # # # # # # # #  #   #  # # # #
# # # # # # # # # #  #   #  # # # #
```

Your mission is to write a program that can display a line of text in ASCII art in a style you are given as input.

## Game Input

*Input*

**Line 1**: the width `L` of a letter represented in ASCII art. All letters are the same width.

**Line 2**: the height `H` of a letter represented in ASCII art. All letters are the same height.

**Line 3**: The line of text `T`, composed of `N` ASCII characters.

**Following lines**: the string of characters ABCDEFGHIJKLMNOPQRSTUVWXYZ? Represented in ASCII art.

*Output*

The text `T` in ASCII art.

The characters a to z are shown in ASCII art by their equivalent in upper case.

The characters that are not in the intervals [a-z] or [A-Z] will be shown as a question mark in ASCII art.

*Constraints*

```text
0 < L < 30
0 < H < 30
0 < N < 200
```

### *Example 1*

*Input*

```text
4
5
E
#  ##   ## ##  ### ###  ## # # ###  ## # # #   # # ###  #  ##   #  ##   ## ### # # # # # # # # # # ### ### 
# # # # #   # # #   #   #   # #  #    # # # #   ### # # # # # # # # # # #    #  # # # # # # # # # #   #   # 
### ##  #   # # ##  ##  # # ###  #    # ##  #   ### # # # # ##  # # ##   #   #  # # # # ###  #   #   #   ## 
# # # # #   # # #   #   # # # #  #  # # # # #   # # # # # # #    ## # #   #  #  # # # # ### # #  #  #       
# # ##   ## ##  ### #    ## # # ###  #  # # ### # # # #  #  #     # # # ##   #  ###  #  # # # #  #  ###  #  
```

*Output*

```text
### 
#   
##  
#   
### 
```

### *Example 2*

*Input*

```text
4
5
MANHATTAN
#  ##   ## ##  ### ###  ## # # ###  ## # # #   # # ###  #  ##   #  ##   ## ### # # # # # # # # # # ### ### 
# # # # #   # # #   #   #   # #  #    # # # #   ### # # # # # # # # # # #    #  # # # # # # # # # #   #   # 
### ##  #   # # ##  ##  # # ###  #    # ##  #   ### # # # # ##  # # ##   #   #  # # # # ###  #   #   #   ## 
# # # # #   # # #   #   # # # #  #  # # # # #   # # # # # # #    ## # #   #  #  # # # # ### # #  #  #       
# # ##   ## ##  ### #    ## # # ###  #  # # ### # # # #  #  #     # # # ##   #  ###  #  # # # #  #  ###  #  
```

*Output*

```text
# #  #  ### # #  #  ### ###  #  ###  
### # # # # # # # #  #   #  # # # #  
### ### # # ### ###  #   #  ### # #  
# # # # # # # # # #  #   #  # # # #  
# # # # # # # # # #  #   #  # # # # 
```

## Source

This exercise was originally published here:

https://www.codingame.com/ide/puzzle/ascii-art
