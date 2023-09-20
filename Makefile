.SUFFIXES:

DOC_PML_SRC := $(shell find -wholename './documentation/*.plantuml')
DOC_PML_BLD_PDF := $(patsubst ./documentation/%.plantuml,.doc_build/pml/%.pdf,$(DOC_PML_SRC))

DOC_TEX_SRC := $(shell find -wholename './documentation/*.tex')
DOC_TEX_DIST_PDF := $(patsubst ./documentation/%/main.tex,documentation/%.pdf,$(DOC_TEX_SRC))

PML_BLD_EPS_TO_LCL_EPS = $(patsubst .doc_build/pml/%.eps,documentation/%.eps,$@)
PML_BLD_EPS_TO_LCL_PML = $(patsubst .doc_build/pml/%.eps,documentation/%.plantuml,$@)
PML_BLD_PDF_TO_BLD_EPS = $(patsubst .doc_build/pml/%.pdf,.doc_build/pml/%.eps,$@)
PML_LCL_EPS_TO_LCL_PML = $(patsubst documentation/%.eps,documentation/%.plantuml,$@)

TEX_BLD_PDF_TO_BLD_DIR = $(patsubst .doc_build/tex/%/main.pdf,.doc_build/tex/%,$@)
TEX_BLD_PDF_TO_ABS_DIR = $(shell pwd)/$(TEX_BLD_PDF_TO_BLD_DIR)
TEX_OUT_PDF_TO_BLD_DIR = $(patsubst documentation/%.pdf,.doc_build/tex/%,$@)
TEX_BLD_PDF_TO_SRC_DIR = $(patsubst .doc_build/tex/%/main.pdf,documentation/%,$@)
TEX_PDF_TO_DIAG_DIR = $(shell pwd)/$(patsubst .doc_build/tex/%/main.pdf,.doc_build/pml/%,$@)


.PHONY: documentation
documentation: documentation/*.pdf
	

.PHONY: clean
clean:
	rm -rf .doc_build
	

.SECONDEXPANSION:
.doc_build/tex/%/main.pdf : $$(wildcard documentation/%/*) $$(wildcard .doc_build/pml/%/**/*) $(DOC_PML_BLD_PDF)
	echo $^
	mkdir -p $(@D)
	cd $(TEX_BLD_PDF_TO_SRC_DIR) ; \
	TEXINPUTS=.:$(TEX_PDF_TO_DIAG_DIR)//: \
	pdflatex -output-directory=$(TEX_BLD_PDF_TO_ABS_DIR) main.tex


documentation/%.pdf : .doc_build/tex/%/main.pdf
	cp $(TEX_OUT_PDF_TO_BLD_DIR)/main.pdf $@


documentation/%.eps : documentation/%.plantuml
	plantuml -teps $(PML_LCL_EPS_TO_LCL_PML)


.doc_build/pml/%.eps : documentation/%.eps
	mkdir -p $(@D)
	mv $(PML_BLD_EPS_TO_LCL_EPS) $(@D)


.doc_build/pml/%.pdf : .doc_build/pml/%.eps
	epstopdf $(PML_BLD_PDF_TO_BLD_EPS)
	rm $(PML_BLD_PDF_TO_BLD_EPS)