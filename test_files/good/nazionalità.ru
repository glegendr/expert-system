# definire le variabili con una stringa
def A "Mangia ๐ญ e ๐บ" "Non mangia ๐ญ e ๐บ"
def B "Mangia ๐ e ๐" "Non mangia ๐ e ๐"
def T "ร un tedesco" "Non รจ tedesco"
def I "ร un italiano" "Non รจ un italiano"
def F "ร un francese" "Non รจ un francese"
def O "ร Olaf Scholz" "Non รจ Olaf Scholz"
def S "ร Sergio Mattarella" "Non รจ Sergio Mattarella"

# regole
if "Mangia ๐ญ e ๐บ" and "Non mangia ๐ e ๐" then "ร un tedesco"   # A + !B => F
if "Mangia ๐ e ๐" and "Non mangia ๐ญ e ๐บ" then "ร un italiano"  # B + !A => C
if "Mangia ๐ญ e ๐บ" and "Mangia ๐ e ๐" then "ร un francese"      # A + B  => M
if "ร un tedesco" or "ร un francese" then "ร Olaf Scholz"                         # F | M  => G
if "ร un italiano" then "ร Sergio Mattarella"                                     # C      => Y
# valori impostati su true
="Mangia ๐ e ๐"
# valori da richiedere
? "ร Olaf Scholz""ร Sergio Mattarella"