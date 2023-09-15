.PHONY: svg clean

DIAGRAMS_SRC := $(shell find -name '*.plantuml')
DIAGRAMS_SVG := $(addsuffix .svg, $(basename $(DIAGRAMS_SRC)))

MAIN_LATEX_SRC := $(shell find -name 'main.tex')
MAIN_LATEX_PDF := $(addsuffix .pdf, $(basename $(MAIN_LATEX_SRC)))

svg: $(DIAGRAMS_SVG)
pdf: $(MAIN_LATEX_PDF)

clean:
	rm -f $(DIAGRAMS_SVG)
	rm -f $(MAIN_LATEX_PDF)

%.svg: %.plantuml
	plantuml -tsvg $^

%.pdf: %.tex
	latexmk -pdf $^