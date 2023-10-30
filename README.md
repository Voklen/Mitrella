# Mitrella

This is a graphing calculator, inspired by desmos (great tool, [check it out](https://www.desmos.com/calculator)) but using my own stack-based mathematical notation.

Read from left to right, each element is seperated by a space, an number or variable on it's own will add it to the stack. Operators pop items from the stack and then push their results to the top of the stack.

In this example, 2 is added to the stack, then 3 is added to the stack. The addition operator(+) pops the top 2 items of the stack, sums them, and then pushes the result.
```
2 3 +
-----
5
```

Note that subtraction (and division) subtracts the item on the top of the stack from the second-to-top item in the stack.
```
2 3 -
-----
-1
```

There is no order of operations, by reading from left to right and using the stack you can unambiguously determine what a expression evaluates to.
```
4 5 * 10 / 6 3 - +
-----
5
```

Optionally, you can add brackets to improve readability but these do not change the result in any way.
```
(4 5 * 10 /) (6 3 -) +
-----
5
```
