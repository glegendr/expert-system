# define variables with string
def A "It live in 🌊 and eats 🪰" "It don't live in 🌊 nor eats 🪰"
def B "It 🎶 and has 🪶" "It don't 🎶 nor has 🪶"
def F "It's a 🐸" "It's definitely not a 🐸"
def C "It's a 🦆" "It's definitely not a 🦆"
def M "It's a 👿" "It's definitely not a 👿"
def G "It's 🟢" "It's definitely not 🟢"
def Y "It's 🟡" "It's definitely not 🟡"
# # define rules
# A + !B => F
# B + !A => C
# A + B => M
# F | M => G
# C => Y
# TODO RULES
if "It live in 🌊 and eats 🪰" and "It don't 🎶 nor has 🪶" then "It's a 🐸"   # A + !B => F
if "It 🎶 and has 🪶" and "It don't live in 🌊 nor eats 🪰" then "It's a 🦆" # B + !A => C
if "It live in 🌊 and eats 🪰" and "It 🎶 and has 🪶" then "It's a 👿"      # A + B  => M
if "It's a 🐸" or "It's a 👿" then "It's 🟢"                     # F | M  => G
if "It's a 🦆" then "It's 🟡"                                      # C      => Y
# values setted to true
="It live in 🌊 and eats 🪰"
# values to request
? "It's 🟢""It's 🟡"