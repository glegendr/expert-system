# define variables with string
def A "It croaks and eats flies" "It don't croaks nor eats flies"
def B "It chirps and sings" "It don't chirps nor sings"
def F "It's a Frog" "It's definitely not a Frog"
def C "It's a Canary" "It's definitely not a Canary"
def M "It's a Monster" "It's definitely not a Monster"
def G "It's colored green" "It's definitely not green"
def Y "It's colored yellow" "It's definitely not yellow"
# # define rules
# A + !B => F
# B + !A => C
# A + B => M
# F | M => G
# C => Y
# TODO RULES
if "It croaks and eats flies" and "It don't chirps nor sings" then "It's a Frog"   # A + !B => F
if "It chirps and sings" and "It don't croaks nor eats flies" then "It's a Canary" # B + !A => C
if "It croaks and eats flies" and "It chirps and sings" then "It's a Monster"      # A + B  => M
if "It's a Frog" or "It's a Monster" then "It's colored green"                     # F | M  => G
if "It's a Canary" then "It's colored yellow"                                      # C      => Y
# values setted to true
="It croaks and eats flies"
# values to request
? "It's colored green""It's colored yellow"
