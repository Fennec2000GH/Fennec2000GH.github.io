---
layout: minimal
title: Misra-Gries Heavy Hitters
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

# Misra-Gries Heavy Hitters

Consider the problem of identifying all elements that appear more than n/k times in a sequence of n elements. Such elements are called heavy hitters. The Misra-Gries algorithm efficiently finds all heavy hitters using limited memory.

**Example**

In the sequence [A, B, A, C, A, A, B] with n=7 and k=3, the heavy hitters are A (appears 4 times) and B (appears 2 times), since both appear more than n/k = 7/3 ≈ 2.33 times.

## Algorithm analysis

**Pseudocode**

From the associated [Wikipedia article](https://en.wikipedia.org/wiki/Misra%E2%80%93Gries_heavy_hitters_algorithm), we find the following pseudocode.

```
algorithm Misra–Gries is
    t, d := { }, 0
    for i from 0 to n-1 do
        if b[i] 
∉
{\displaystyle \notin } t then
            t, d:= t ∪ {b[i]}, d+1
        else 
            t, d:= t ∪ {b[i]}, d
        endif
        if d = k then
            Delete k distinct values from t; update d
        endif
    endfor
```

**Interpretation**

To-do...

