.SUFFIXES:

OUT_FILE = documentation/Specification.pdf

DOC_PML_SRC := $(shell find -wholename './documentation/*.plantuml')
DOC_PML_BLD_PDF := $(patsubst ./documentation/%.plantuml,.doc_build/pml/%.pdf,$(DOC_PML_SRC))

DOC_TEX_SRC := $(shell find -wholename './documentation/*.tex')
DOC_TEX_DIST_PDF := $(patsubst ./documentation/%/main.tex,documentation/%.pdf,$(DOC_TEX_SRC))

PML_BLD_PDF_TO_LCL_PDF = $(patsubst .doc_build/pml/%.pdf,documentation/%.pdf,$@)
PML_BLD_PDF_TO_LCL_PML = $(patsubst .doc_build/pml/%.pdf,documentation/%.plantuml,$@)

TEX_BLD_TO_ABS_DIR = $(shell pwd)/$(@D)
TEX_OUT_PDF_TO_BLD_DIR = $(patsubst documentation/%.pdf,.doc_build/tex/%,$@)
TEX_BLD_DIR_TO_SRC_DIR = $(patsubst .doc_build/tex/%,documentation/%,$(@D))
TEX_BLD_DIR_TO_DIAG_DIR = $(shell pwd)/$(patsubst .doc_build/tex/%,.doc_build/pml/%,$(@D))

TEX_BLD_OPT = TEXINPUTS=.:$(TEX_BLD_DIR_TO_DIAG_DIR)//:
TEX_BLD_CMD = xelatex -halt-on-error -output-directory=$(TEX_BLD_TO_ABS_DIR) main.tex
TEX_BLD_ALL = $(TEX_BLD_OPT) $(TEX_BLD_CMD)


.PHONY: documentation
documentation: $(OUT_FILE)
	

.PHONY: clean
clean:
	rm -rf .doc_build $(OUT_FILE)
	

.SECONDARY:
.SECONDEXPANSION:
.doc_build/tex/%/main.pdf : $$(wildcard documentation/%/*) $$(wildcard .doc_build/pml/%/**/*) $(DOC_PML_BLD_PDF) .doc_build/tex/%/main.ind .doc_build/tex/%/main.gls $(DOC_TEX_SRC)
	mkdir -p $(@D)
	cd $(TEX_BLD_DIR_TO_SRC_DIR) ; \
	$(TEX_BLD_ALL)


documentation/%.pdf : .doc_build/tex/%/main.pdf
	cp $(TEX_OUT_PDF_TO_BLD_DIR)/main.pdf $@


.SECONDARY:
.doc_build/pml/%.pdf : documentation/%.plantuml
	plantuml -tpdf $(PML_BLD_PDF_TO_LCL_PML)
	mkdir -p $(@D)
	mv $(PML_BLD_PDF_TO_LCL_PDF) $(@D)


.SECONDARY:
.SECONDEXPANSION:
.doc_build/tex/%/main.ind : $$(wildcard documentation/%/*) $$(wildcard .doc_build/pml/%/**/*) $(DOC_PML_BLD_PDF) $(DOC_TEX_SRC) .doc_build/tex/%/main.gls
	mkdir -p $(@D)
	cd $(TEX_BLD_DIR_TO_SRC_DIR) ; \
	$(TEX_BLD_ALL)
	cd $(@D) ; \
	makeindex main.idx


.SECONDARY:
.SECONDEXPANSION:
.doc_build/tex/%/main.gls : $$(wildcard documentation/%/*) $$(wildcard .doc_build/pml/%/**/*) $(DOC_PML_BLD_PDF) $(DOC_TEX_SRC)
	mkdir -p $(@D)
	cd $(TEX_BLD_DIR_TO_SRC_DIR) ; \
	$(TEX_BLD_ALL)
	cd $(@D) ; \
	makeglossaries main