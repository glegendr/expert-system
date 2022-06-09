# define variables with string
def A "Il croasse et mange des mouches" "Il ne croasse pas et ne mange pas de mouches"
def B "Il mange du fromage et a des moustaches" "Il n'aime pas le fromage et n'as pas de moustaches"
def F "C'est une grenouille" "Ce n'est pas une grenouille"
def R "C'est un rat" "Ce n'est pas un rat"
def M "C'est un monstre" "Ce n'est pas un monstre"
def G "C'est Fritz !" "Ce n'est pas Fritz"
def Y "C'est Ratatouille !" "Ce n'est pas Ratatouille"

# rules
if "Il croasse et mange des mouches" and "Il n'aime pas le fromage et n'as pas de moustaches" then "C'est une grenouille"   # A + !B => F
if "Il mange du fromage et a des moustaches" and "Il ne croasse pas et ne mange pas de mouches" then "C'est un rat"         # B + !A => C
if "Il croasse et mange des mouches" and "Il mange du fromage et a des moustaches" then "C'est un monstre"                  # A + B  => M
if "C'est une grenouille" or "C'est un monstre" then "C'est Fritz !"                                                        # F | M  => G
if "C'est un rat" then "C'est Ratatouille !"                                                                                # C      => Y
# values setted to true
="Il mange du fromage et a des moustaches"
# values to request
? "C'est Fritz !""C'est Ratatouille !"