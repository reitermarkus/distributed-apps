files := $(filter-out presentation.tex biblio.bib report.tex, $(wildcard presentation.* biblio.* report.*))

all: pdf


.PHONY: presentation
presentation:
	tectonic --reruns 0 presentation.tex

.PHONY: report
report:
	tectonic --reruns 0 report.tex

.PHONY: pdf
pdf: presentation report

.PHONY: clean
clean: $(files)
	rm $(files)

