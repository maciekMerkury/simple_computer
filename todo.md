# overall todos for this project

## CPU:
### instructions:
    - fuck that, i forgot about the existance of r1, substract 3 from all jiz-values
        - Jiz should use a 14-bit value for the PC value, as this is the maximum that can be expressed in a 16-bit instruction, given that 2 bits are needed for the instruction type
            - it could even have a 15-bit value, if all other instructions were shifted left by 1. That would  mean that if the right-most bit is 1, this instruction is jiz and should be treated as such
            - i still have to think about that tho, as this might be simply annoying

