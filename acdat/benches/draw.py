import re
import matplotlib.pyplot as plt
import matplotlib.font_manager as fmg
from matplotlib.ticker import MultipleLocator, FormatStrFormatter
import numpy as np

result = '''
test bench05_acdat ... bench:   3,128,258 ns/iter (+/- 275,459)
test bench05_crate ... bench:   8,664,802 ns/iter (+/- 758,563)
test bench06_acdat ... bench:   3,686,061 ns/iter (+/- 596,717)
test bench06_crate ... bench:  10,223,664 ns/iter (+/- 1,586,353)
test bench07_acdat ... bench:   4,194,413 ns/iter (+/- 487,957)
test bench07_crate ... bench:  12,570,452 ns/iter (+/- 3,549,142)
test bench08_acdat ... bench:   4,482,788 ns/iter (+/- 537,039)
test bench08_crate ... bench:  13,753,303 ns/iter (+/- 1,857,045)
test bench09_acdat ... bench:   4,807,153 ns/iter (+/- 575,382)
test bench09_crate ... bench:  15,847,870 ns/iter (+/- 1,745,553)
test bench10_acdat ... bench:   5,420,359 ns/iter (+/- 580,452)
test bench10_crate ... bench:  17,630,200 ns/iter (+/- 2,588,742)
test bench11_acdat ... bench:   5,988,254 ns/iter (+/- 548,343)
test bench11_crate ... bench:  19,622,801 ns/iter (+/- 3,127,411)
test bench12_acdat ... bench:   6,009,782 ns/iter (+/- 641,030)
test bench12_crate ... bench:  21,118,994 ns/iter (+/- 3,146,038)
test bench13_acdat ... bench:   6,596,669 ns/iter (+/- 760,102)
test bench13_crate ... bench:  22,879,564 ns/iter (+/- 3,014,345)
test bench14_acdat ... bench:   7,006,636 ns/iter (+/- 859,421)
test bench14_crate ... bench:  24,073,799 ns/iter (+/- 3,959,134)
test bench15_acdat ... bench:   7,363,400 ns/iter (+/- 1,385,673)
test bench15_crate ... bench:  25,732,916 ns/iter (+/- 4,704,062)
test bench16_acdat ... bench:   7,486,487 ns/iter (+/- 1,028,483)
test bench16_crate ... bench:  26,134,218 ns/iter (+/- 1,940,928)
test bench17_acdat ... bench:   7,929,154 ns/iter (+/- 726,016)
test bench17_crate ... bench:  28,293,507 ns/iter (+/- 4,207,779)
test bench18_acdat ... bench:   8,344,587 ns/iter (+/- 1,276,831)
test bench18_crate ... bench:  29,172,750 ns/iter (+/- 3,197,207)
'''

data = re.findall(r'bench(\d+)_(\w+) .* ([\d,]*) .* ([\d,]*)', result)
data = [(int(x), y, int(a.replace(',', '')), int(b.replace(',', '')))for x, y, a, b in data]

x1 = np.array([x for x, y, a, b in data if y == 'acdat'])
y1 = np.array([a/10000.0 for x, y, a, b in data if y == 'acdat'])
e1 = np.array([b/10000.0/10000.0 for x, y, a, b in data if y == 'acdat'])
z1 = np.poly1d(np.polyfit(x1, y1, 1))
print(np.polyfit(x1, y1, 1))

x2 = np.array([x for x, y, a, b in data if y == 'crate'])
y2 = np.array([a/10000.0 for x, y, a, b in data if y == 'crate'])
e2 = np.array([b/10000.0/10000.0 for x, y, a, b in data if y == 'crate'])
z2 = np.poly1d(np.polyfit(x2, y2, 1))
print(np.polyfit(x2, y2, 1))

xp = np.linspace(0, 20, 100)

prop = fmg.FontProperties(fname='/System/Library/Fonts/HelveticaNeue.ttc')
prop.set_size(18)
fig, ax = plt.subplots(figsize=(10, 6), dpi=160)
ax.errorbar(x1, y1, yerr=e1, marker='o', label='Aho–Corasick Double-Array Trie')
ax.plot(xp, z1(xp))
ax.errorbar(x2, y2, yerr=e2, marker='o', label='FnvHashMap Trie (hyphenation 0.6.1)')
ax.plot(xp, z2(xp))
ax.set_title('hyphenation speed benchmark', fontproperties=prop)

ax.xaxis.set_major_locator(MultipleLocator(1))
ax.xaxis.grid(True, which='major')

ax.yaxis.set_major_locator(MultipleLocator(200))
ax.yaxis.set_minor_locator(MultipleLocator(50))
ax.yaxis.grid(True, which='minor')
ax.yaxis.set_major_formatter(FormatStrFormatter('%dns/word'))

ax.set_xlim(0, 20)
ax.set_ylim(0, 3500)
ax.set_xlabel('word length')
ax.set_ylabel('time cost')
ax.legend(loc='best')

plt.savefig('result.png', pad_inches=0)
