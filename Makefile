.PHONY: svg clean

DIAGRAMS_SRC := $(shell find -name '*.plantuml')
DIAGRAMS_SVG := $(addsuffix .svg, $(basename $(DIAGRAMS_SRC)))

svg: $(DIAGRAMS_SVG)

clean:
	rm -f $(DIAGRAMS_SVG)

%.svg: %.plantuml
	plantuml -tsvg $^
