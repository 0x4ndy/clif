# clif
`clif` is a CLI fuzzer. 

# Usage
```bash
clif -e <executable> [-w <wordlist>, -n <number_range> -s <string_range>] -a "args_with_marks"

-e - executable
-w - wordlist
-a - arguments as string
-n - number range
-s - list of strings of 'A' a defined length range
```

# Example
```bash
# throw wordlist.txt as input
clif -e my_program -w wordlist.txt 

# throw wordlist.txt as -p argument
clif -e my_program -w wordlist.txt -a "-p FUZZ" 

# throw numbers from range 100..100000000 as the first argument
clif -e my_program -n 100..100000000 -a "-n FUZZ" 

# throw a string with length from range 10..100 as the first argument
clif -e my_program -s 10..100 
```
