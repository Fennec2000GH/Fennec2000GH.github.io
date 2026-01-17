---
layout: minimal
title: String Search Algorithms
nav_exclude: true
---

<script>
window.MathJax = {
  tex: {
    inlineMath: [['$', '$'], ['\\(', '\\)']],
    displayMath: [['$$', '$$'], ['\\[', '\\]']]
  }
};
</script>

<script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>


# String Search Algorithms

## Problem

More specifically, the problem is simply finding the occurrence of a substring s2 within a longer string s1. There are different variations of the problem, including finding first match or all matches. By default, we assume finding the first match unless otherwise noted. Denote the lengths of s1 and s2 by M and N, respectively. For any indexes that iterate over the strings, denote the indexes for s1 and s2 by i and j, respectively.

## Algorithms

### 1. Naive search

We use a sliding window representing character-by-character checks on s2, which scans through s1 from each index i until we find some i such that the next N characters match s2. To make the code more easy to implement, we instead reject the current window upon detecting an unmatched character during the window being scanned.

```c++
#include <string>

using namespace std;
string s1 = "thecowjumpedoverthemoon";
string s2 = "jump";
size_t M = s1.size();
size_t N = s2.size();

for(size_t i = 0; i < M - N; i++) {
  bool found = true;
  for(size_t j = 0; j < N; j++)
    if(s1[i+j] != s2[j]) {
      found = false;
      break;
  }
  if(found)
    return 0;
}

return -1;
```


Time Complexity: O(MN)

Space Complexity: O(M)

### 2. Rabin-Karp
**2.1 Recompute hash per substring window**


```c++
#include <string>
#include <functional>

using namespace std;
string s1 = "thecowjumpedoverthemoon";
string s2 = "jump";
size_t M = s1.size();
size_t N = s2.size();

hash<std::string> hasher;
size_t hash_expected = hasher(s2);

for(size_t i = 0; i < M - N; i++) {
  string window = s1.substr(i, N);
  size_t hash = hasher(window);
  
  if(hash == hash_expected && window == s2)
      return 0;
}

return -1;
```

Time Complexity: O(MN)

Space Complexity: O(M)

**2.2 Use hash function that supports window of constant time and space complexity**

Suppose we use a simple hash function that is fully predictable, preferably one that means constant space and time complexity each time the substring window rolls forward by one. An easy example is simply summing up the integer values of the characters held by the window. Because each adjacent window differs by one character, we can simply subtract the leftmost character from the current sum and add the new character just to the right of the window to compute the "hash" for the new window.

```c++
#include <string>
#include <numeric>

using namespace std;
string s1 = "thecowjumpedoverthemoon";
string s2 = "jump";
size_t M = s1.size();
size_t N = s2.size();

int sum_expected = accumulate(s2.begin(), s2.end(), 0);
int sum = sum_expected;

s1.substr(0, N).sum()

for(size_t i = N; i < M - N; i++) {
  sum -= (int)s1[i - N];
  sum += (int)s1[i];
  
  if(sum == sum_expected && s1.substr(0, N) == s2)
      return 0;
}

return -1;
```

Time Complexity: O(MN)

Space Complexity: O(M)

Note that the worst case time and space complexities do not change. However, the total number of steps, as in character reads, is on average much less than the standard Rabin-Karp algorithm due to constant-time computation of the hash due to being a window sum.
