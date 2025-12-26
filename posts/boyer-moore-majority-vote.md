---
layout: minimal
title: Boyer-Moore Majority Vote
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


# Boyer-Moore Majority Vote

## Problem

Consider the problem of identifying the element that appears the majority of times in a sequence. Such an element, i.e. majority element, must occur > 50% of the time in the sequence. 

**Example**

In the sequence [A, B, A, C, A, A, B], the majority element is A since it appears 4 out of 7 times (> 50%).

## Algorithm analysis

**Pseudocode**

From the associated [Wikipedia article](https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm), we find the following pseudocode.

```
Initialize an element m
Initialize a counter c ← 0
for each element x of the input sequence:
    if c = 0, do 
        m ← x
        c ← 1
    else if m = x, do
        c ← c + 1
    else do
        c ← c - 1
return m
```

**Interpretation**

- Let *S* denote the sequence of elements, which has length *N*. Let $I_M$ denote the set of all indices in *S* where *M* appears, i.e. $I_M = \{i : S[i] = M, i = 0, 1, \dots, N - 1 \}$ if we use 0-indexing.

- We have a single placeholder variable *m*, that when a majority element *M* does exist, is guaranteed to equal *M* after iterating through the whole sequence using the algorithm. 

- Since *M* makes up >50% of all elements in the sequence, there exists a 1-to-1 matching between each non-M element to a unique element equal to M, with leftover unmatched elements still equal to M.

- Therefore, the last increment operation performed on counter *c* must (1) either be done by some M element or (2) decremented by some non-M element, but leaves counter positive. This keeps the current value of *m* still set to *M*.
