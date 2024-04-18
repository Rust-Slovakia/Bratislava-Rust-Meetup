.PHONY: all clean watch

all: talk.html
watch:
	firefox talk.html
	echo talk.md | entr -s 'make'

clean:
	rm talk.html serde.svg

talk.html: talk.md crystallite.theme
	pandoc $<  \
		-t slidy --katex --standalone \
		--css ./slidy_black.css \
		--highlight-style=./crystallite.theme -o $@


talk.pdf: talk.md
	pandoc $< -t pdf -o $@

%.svg: %.d2
	d2 --dark-theme 200 $< $@

