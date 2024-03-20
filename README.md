# Nums

# Rules

You decide on a target-number which normally is a 3-digit number.
In addition you roll 4 dice, each with the numbers 1-6.

The goal of this game is to find a way to calculate the target-number using the 4 dice.

The numbers on the dice can be used as is or you can multiply them by 10.
So the number 3 could be used as a 3, 30, 300, etc.

To combine the numbers you can use the following operations:
- Addition: 3 + 4 = 7
- Subtraction: 3 - 2 = 1
- Multiplication: 3 * 4 = 12
- Division: 6 / 2 = 3

You can not use the numbers in other operations like exponents or roots.

To complete the game you have to find a way to calculate the target-number using the 4 dice.
You cannot use a dice more than once and you have to use all 4 dice.

It is likely that there are multiple solutions. There are also cases where no solution is possible.


# Examples

## Trivial Example

target: 123, dice: 1 2 3 1

One possible solution is: (100 + 20 + 3) * 1 = 123

## More Complex Example

target: 130, dice: 6 6 4 3

One possible solution is: ((60 / 6) + (40 * 3)) = 130

# Tool

This program is a tool to calculate solutions for the game.
It will incrementally ask to show the next step in the solutions.
This way you can use the tool to "create" a game, try solving it yourself and then use the tool to see other solutions.

# Usage

```console 
nums <target_num> <dice1> <dice2> <dice3> <dice4>
```

All Arguments are optional.

You can run the program without any arguments to generate the target and the dice values.

Alternatively, you can run the program with the target specified.
In Addition it will generate the dice values.

```console
nums 123
```

Lastly you can specifiy the target and the dice values.

```console
nums 123 1 2 3 4
```

This will calculate the solutions to the target with the given dice values.
