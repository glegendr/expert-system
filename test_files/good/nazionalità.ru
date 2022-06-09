# definire le variabili con una stringa
def A "Mangia 🌭 e 🍺" "Non mangia 🌭 e 🍺"
def B "Mangia 🍝 e 🍕" "Non mangia 🍝 e 🍕"
def T "È un tedesco" "Non è tedesco"
def I "È un italiano" "Non è un italiano"
def F "È un francese" "Non è un francese"
def O "È Olaf Scholz" "Non è Olaf Scholz"
def S "È Sergio Mattarella" "Non è Sergio Mattarella"

# regole
if "Mangia 🌭 e 🍺" and "Non mangia 🍝 e 🍕" then "È un tedesco"   # A + !B => F
if "Mangia 🍝 e 🍕" and "Non mangia 🌭 e 🍺" then "È un italiano"  # B + !A => C
if "Mangia 🌭 e 🍺" and "Mangia 🍝 e 🍕" then "È un francese"      # A + B  => M
if "È un tedesco" or "È un francese" then "È Olaf Scholz"                         # F | M  => G
if "È un italiano" then "È Sergio Mattarella"                                     # C      => Y
# valori impostati su true
="Mangia 🍝 e 🍕"
# valori da richiedere
? "È Olaf Scholz""È Sergio Mattarella"