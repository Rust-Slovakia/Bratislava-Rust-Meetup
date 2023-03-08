.PHONY: all clean

all: talk.html
clean:
	rm talk.html serde.svg

talk.html: talk.md serde.svg crystallite.theme
	pandoc $<  \
		-t slidy --katex --standalone \
		--css ./slidy_black.css \
		--highlight-style=./crystallite.theme -o $@


talk.pdf: talk.md
	pandoc $< -t pdf -o $@

%.svg: %.d2
	d2 --dark-theme 200 $< $@
