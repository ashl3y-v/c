#!/bin/sh
gs -q -o $2 -dNOPAUSE -dBATCH -dSAFER -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dEmbedAllFonts=true -dSubsetFonts=true $1
qpdf --compress-streams=y --object-streams=generate --recompress-flate --optimize-images --compression-level=9 --replace-input $2
