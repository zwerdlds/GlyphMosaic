.SUFFIXES:

DOC_PML_SRC := $(shell find -wholename './documentation/*.plantuml')
DOC_PML_BLD_PDF := $(patsubst ./documentation/%.plantuml,.doc_build/pml/%.pdf,$(DOC_PML_SRC))

DOC_TEX_SRC := $(shell find -wholename './documentation/*.tex')
DOC_TEX_DIST_PDF := $(patsubst ./documentation/%/main.tex,documentation/%.pdf,$(DOC_TEX_SRC))

PML_BLD_PDF_TO_LCL_PDF = $(patsubst .doc_build/pml/%.pdf,documentation/%.pdf,$@)
PML_BLD_PDF_TO_LCL_PML = $(patsubst .doc_build/pml/%.pdf,documentation/%.plantuml,$@)

TEX_BLD_IDX_TO_BLD_DIR = $(patsubst .doc_build/tex/%/main.ind,.doc_build/tex/%,$@)
TEX_BLD_PDF_TO_BLD_DIR = $(patsubst .doc_build/tex/%/main.pdf,.doc_build/tex/%,$@)
TEX_BLD_PDF_TO_ABS_DIR = $(shell pwd)/$(TEX_BLD_PDF_TO_BLD_DIR)
TEX_BLD_IDX_TO_ABS_DIR = $(shell pwd)/$(TEX_BLD_IDX_TO_BLD_DIR)
TEX_OUT_PDF_TO_BLD_DIR = $(patsubst documentation/%.pdf,.doc_build/tex/%,$@)
TEX_BLD_IDX_TO_SRC_DIR = $(patsubst .doc_build/tex/%/main.ind,documentation/%,$@)
TEX_BLD_PDF_TO_SRC_DIR = $(patsubst .doc_build/tex/%/main.pdf,documentation/%,$@)
TEX_PDF_TO_DIAG_DIR = $(shell pwd)/$(patsubst .doc_build/tex/%/main.pdf,.doc_build/pml/%,$@)
TEX_IDX_TO_DIAG_DIR = $(shell pwd)/$(patsubst .doc_build/tex/%/main.ind,.doc_build/pml/%,$@)

TEX_PDF_BLD_OPT = TEXINPUTS=.:$(TEX_PDF_TO_DIAG_DIR)//: xelatex -halt-on-error -output-directory=$(TEX_BLD_PDF_TO_ABS_DIR) main.tex
TEX_PDF_BLD_CMD = TEXINPUTS=.:$(TEX_PDF_TO_DIAG_DIR)//: xelatex -halt-on-error -output-directory=$(TEX_BLD_PDF_TO_ABS_DIR) main.tex
TEX_PDF_BLD_ALL = $(TEX_PDF_BLD_OPT) $(TEX_PDF_BLD_CMD)

TEX_IDX_BLD_OPT = TEXINPUTS=.:$(TEX_IDX_TO_DIAG_DIR)//: xelatex -halt-on-error -output-directory=$(TEX_BLD_IDX_TO_ABS_DIR) main.tex
TEX_IDX_BLD_CMD = TEXINPUTS=.:$(TEX_IDX_TO_DIAG_DIR)//: xelatex -halt-on-error -output-directory=$(TEX_BLD_IDX_TO_ABS_DIR) main.tex
TEX_IDX_BLD_ALL = $(TEX_IDX_BLD_OPT) $(TEX_IDX_BLD_CMD)


.PHONY: documentation
documentation: documentation/Specification.pdf
	

.PHONY: clean
clean:
	rm -rf .doc_build
	

.SECONDEXPANSION:
.doc_build/tex/%/main.pdf : $$(wildcard documentation/%/*) $$(wildcard .doc_build/pml/%/**/*) $(DOC_PML_BLD_PDF) .doc_build/tex/%/main.ind $(DOC_TEX_SRC)
	mkdir -p $(@D)
	cd $(TEX_BLD_PDF_TO_SRC_DIR) ; \
	$(TEX_PDF_BLD_ALL) ; \
	$(TEX_PDF_BLD_ALL) ;


documentation/%.pdf : .doc_build/tex/%/main.pdf
	cp $(TEX_OUT_PDF_TO_BLD_DIR)/main.pdf $@


.doc_build/pml/%.pdf : documentation/%.plantuml
	plantuml -tpdf $(PML_BLD_PDF_TO_LCL_PML)
	mkdir -p $(@D)
	mv $(PML_BLD_PDF_TO_LCL_PDF) $(@D)


.SECONDEXPANSION:
.doc_build/tex/%/main.ind : $$(wildcard documentation/%/*) $$(wildcard .doc_build/pml/%/**/*) $(DOC_PML_BLD_PDF) $(DOC_TEX_SRC)
	mkdir -p $(@D)
	cd $(TEX_BLD_IDX_TO_SRC_DIR) ; \
	$(TEX_IDX_BLD_ALL)
	cd $(@D) ; \
	makeindex main.idx