DOC_DIAG_SRC := $(shell find -wholename './documentation/src/diagrams/*.plantuml')
DOC_DIAG_PDF := $(patsubst ./documentation/src/diagrams/%.plantuml, ./documentation/src/diagrams/%.pdf, $(DOC_DIAG_SRC))

DOC_TEX_SRC := $(shell find -wholename './documentation/src/documents/*.tex')
DOC_TEX_DIST_PDF := $(patsubst ./documentation/src/documents/%.tex, ./documentation/dist/%.pdf, $(DOC_TEX_SRC))


.PHONY: documentation
documentation: documentation/dist/Architecture.pdf


.PHONY: diagrams
diagrams : $(DOC_DIAG_PDF)
	echo "PMLs: [$(DOC_DIAG_SRC)]"
	echo "Dist PDFs: [$(DOC_DIAG_PDF)]"


.PHONY: tex
tex : diagrams $(DOC_TEX_DIST_PDF)
	echo "TEXs: [$(DOC_TEX_SRC)]"
	echo "Dist PDFs: [$(DOC_TEX_DIST_PDF)]"


.PHONY: clean
clean : 
	rm -rf "./documentation/dist"


documentation/src/diagrams/%.pdf : documentation/src/diagrams/%.plantuml
	plantuml -tpdf $^


documentation/src/documents/%.pdf : documentation/src/documents/%.tex
	cd $(@D) ; \
	pdflatex $(patsubst documentation/src/documents/%, %, $^) --shell-escape


documentation/dist/%.pdf : documentation/src/documents/%.pdf
	@mkdir -p $(@D)
	cp $^ $@
