---
layout: post
title: CS 6422 - Database System Implementation
nav_exclude: true
---

{% assign course-notes = site.data.course-notes.notes-tree | where: "course-code", title | first %}

# {{ course-notes.course-code }} - {{ course-notes.course-name }}

## Notes by week

{% for note in course-notes.tree %}
[{{ note[0] }}]({{ note[1] | split: ".md" | first }})
{% endfor %}
