---
layout: about
title: About Me
permalink: /about/
---

<style>
	.attention { color: beige; }
</style>

# Caijun Qin
**Master's student @GeorgiaTech | Former SDE @AWS**


---

## Contact | Links
<!-- 📍 Location: Seattle, WA -->

| - | - |
| 💼 LinkedIn: | [https://www.linkedin.com/in/cq-profile](https://www.linkedin.com/in/cq-profile) |
| 🧑‍💻 GitHub: | [https://www.github.com/Fennec2000GH](https://www.github.com/Fennec2000GH) |
| ✉️ Email:  | [qcaijun2013@gmail.com](mailto:qcaijun2013@gmail.com) |
| 📄 Resume/CV dump: | [https://fennec2000gh.github.io/Resume/](https://fennec2000gh.github.io/Resume/) |

---

<div class="attention">
	<h2><strong> Attention </strong></h2>
	<br/>
	<span> Welcome to my online hub. This website was originally intended as a knowledge dump, but I later decided to double as my personal landing page. If you are a recruiter, you will find that this page greatly overlaps with my resume. Feel free to visit the last link under the above section. Otherwise, please visit whatever you find interesting on the sidebar. Cheers! 🥂  </span>
</div>

---

## About

For my undergraduate studies, I earned my dual degree in Computer Science and Statistics from the University of Florida. While there, I became invested in several projects, research programs, and research assistantships in the areas of applied natural language processing (NLP). The applications ranged from indexing ancient Latin and Greek that have undergone digitization to judging the performance between different machine learning models on classifying legal contracts and documents.

After graduating, I landed my first serious employment as a software engineer at Amazon Web Services, Inc. in Seattle. In my almost 2.5 years there, I chiefly worked on support systems used by various teams in the Elastic Load Balancing (ELB) division. My longest term accomplishment there was designing, constructing, and iteratively refining an automated patching system for EC2 instance fleets while under the constraint of minimizing downtime.

Currently, I am pursuing a M.S. in Computer Science from Georgia Institute of Technology as a means to re-upskill. This is one major step in my future goal in pivoting towards the language and compiler space within software engineering.

Outside of school and work, I split my free time amongst running, reading science fiction novels, and playing logic intensive games such as chess and go.

---

## Education

| - | - |
| 01/2025 – 12/2026 | **Georgia Institute of Technology** <br/> M.S. in Computer Science |
| 08/2019 – 05/2022 | **University of Florida** <br/> B.S. in Computer Science \| B.A. in Statistics |

---

## Professional Experience

| - | - |
| 10/2022 – 01/2025 | **Software Development Engineer** <br/> Amazon Web Services, Inc. |
| 06/2022 – 10/2022 | **Software Engineer Internship** <br/> UKG, Inc. |

---

## Selected Projects
<table>
	{% for project in site.data.about.selected-projects %}
	<tr>
		<td>
			<article class="card">
				<div class="card-body">
					<h2 class="card-title"><strong> {{ project.title }} </strong></h2>
					<h4> {{ project.subtitle }} </h4>
					<ul>
					{% for description in project.description %}
						<li><p class="card-text"> {{ description }} </p></li>
					{% endfor %}
					</ul>
				</div>
			</article>
		</td>
	</tr>
	{% endfor %}
</table>

## Publications & Writing

| - |
| Qin, C., Yang, Y., Chen, H., & Ding, J. (2021). A Comparison Study of Machine Learning and Deep Learning for Legal Contract Understanding [Manuscript submitted for publication], Department of Computer & Information Science & Engineering (CISE), University of Florida. |
| Qin, C. (2021). Predictive Sampling Method for Spread Models in Networks. UF Journal of Undergraduate Research, 23(Fall 2021). [https://doi.org/10.32473/ufjur.v23i.128429](https://doi.org/10.32473/ufjur.v23i.128429) |

---

## Awards & Recognition

| - | - |
| 06/2021 | **NSF REU: College of Information at UNT** <br/> $7000 \| University of North Texas, Denton, TX |
| 03/2021 | **Gartner Group Information Technology Fund** <br/> $1000 \| University of Florida, Gainesville, FL |
| 05/2020 | **Russell and Mary Hyatt McCaughan Scholarship** <br/> $1000 \| University of Florida, Gainesville, FL |
| 02/2020 | **University Scholars Program Stipend** <br/> $1750 \| University of Florida, Gainesville, FL |
| 05/2018 | **University Freshman Scholarship** <br/> $1200 / Semester • Florida State University, Tallahassee, FL |

---

## Skills
**Programming/Scripting/DB languages** <br/>
Python, C++, Java, Ruby, JavaScript/TypeScript, SQL

**Tools** <br/>
core AWS services, Ruby-on-Rails, Flask, Linux utilities, Docker, Kubernetes, Node.js
