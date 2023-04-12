### Solution overview:
The solution involves iterating through the input Roman numeral string and converting each character to its corresponding integer value using the `roman_char_to_int` function. During the iteration, the `eval_roman_chars` function is used to compare the current Roman character with the next one. This helps to identify and handle the cases where subtraction is required, such as 'IV' and 'IX'. If subtraction is needed, the function returns a tuple containing a boolean value `true` (indicating to skip the next character) and the integer value resulting from the subtraction. Otherwise, the function returns `false` and the integer value of the current Roman character.

The `roman_to_int` function iterates through the input string, applying the `eval_roman_chars` function to determine whether to add or subtract the corresponding integer values. It keeps track of the total value in the `total` variable. Once the iteration is complete, the `total` value is returned as the integer representation of the input Roman numeral.

### Implementation details:
- A `roman_char_to_int` function that converts a single Roman character to its corresponding integer value.
- An `eval_roman_chars` function that evaluates two Roman characters and returns a tuple containing a boolean value to indicate whether to skip the next character and an integer value to be added to the total.
- A `roman_to_int` function that converts a given Roman numeral string to its integer value.

Additionally, tests have been added to verify the correctness of the solution.

