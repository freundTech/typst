---
title: Unreleased
description: Changes that are not yet released
---

# Unreleased changes

## Highlights
- Added support for multi-column floating [placement]($place.scope) and
  [figures]($figure.scope)
- Added support for automatic [line numbering]($par.line) (often used in
  academic papers)
- Typst's layout engine is now multithreaded. Typical speedups are 2-3x for
  larger documents. The multithreading operates on page break boundaries, so
  explicit page breaks are necessary for it to kick in.
- Paragraph justification was optimized with a new two-pass algorithm. Speedups
  are larger for shorter paragraphs and go up to 6x.
- Highly reduced PDF file sizes due to better font subsetting (thanks to
  [@LaurenzV](https://github.com/LaurenzV))
- Emoji are now exported properly in PDF
- Added initial support for PDF/A. For now, only the PDF/A-2b profile is
  supported, but more is planned for the future.
- Added various options for configuring the CLI's environment (fonts, package
  paths, etc.)
- Text show rules now match across multiple text elements
- Block-level equations can now optionally break over multiple pages
- Fixed a bug where some fonts would not print correctly on professional
  printers
- Fixed a long-standing bug which could cause headings to be orphaned at the
  bottom of the page

## Layout
- Added support for multi-column floating placement and figures via
  [`place.scope`] and [`figure.scope`].  Two-column documents should now prefer
  `{set page(columns: 2)}` over `{show: column.with(2)}` (see the
  [page setup guide]($guides/page-setup-guide/#columns)).
- Added support for automatic [line numbering]($par.line) (often used in
  academic papers)
- Added [`par.spacing`] property for configuring paragraph spacing. This should
  now be used instead of `{show par: set block(spacing: ..)}`
  **(Breaking change)**
- Block-level elements like lists, grids, and stacks now show themselves as
  blocks and are thus affected by all block properties (e.g. `stroke`) rather
  than just `spacing` **(Breaking change)**
- Added [`block.sticky`] property which prevents a page break after a block
- Added [`place.flush`] function which forces all floating figures to be placed
  before any further content
- Added [`skew`] function
- Added `{auto}` option for [`page.header`] and [`page.footer`] which results in
  an automatic header/footer based on the numbering (which was previously
  inaccessible after a change)
- Added `gap` and `justify` parameters to [`repeat`] function
- Added `width` and `height` parameters to the [`measure`] function to define
  the space in which the content should be measured. Especially useful in
  combination with [`layout`].
- The height of a `block`, `image`, `rect`, `square`, `ellipse`, or `circle` can
  now be specified in [fractional units]($fraction)
- The [`scale`] function now supports absolute lengths for `x`, `y`, `factor`.
  This way an element of unknown size can be scaled to a fixed size.
- The values of `block.above` and `block.below` can now be retrieved in context
  expressions.
- Increased accuracy of conversions between absolute units (pt, mm, cm, in)
- Fixed a bug which could cause headings to be orphaned at the bottom of the
  page
- Fixed footnotes within breakable blocks appearing on the page where the
  breakable block ends instead of at the page where the footnote marker is
- Fixed numbering of nested footnotes and footnotes in floats
- Fixed empty pages appearing when a [context] expression wraps whole pages
- Fixed `{set block(spacing: x)}` behaving differently from
  `{set block(above: x, below: x)}`
- Fixed behavior of [`rotate`] and [`scale`] with `{reflow: true}`
- Fixed interaction of `{align(horizon)}` and `{v(1fr)}`
- Fixed various bugs where floating placement would yield overlapping results
- Fixed a bug where widow/orphan prevention would unnecessarily move text into
  the next column
- Fixed [weak spacing]($h.weak) not being trimmed at the start and end of lines
  in a paragraph (only at the start and end of paragraphs)
- Fixed interaction of weak page break and [`pagebreak.to`]
- Fixed compilation output of a single weak page break
- Fixed crash when [padding]($pad) by `{100%}`

## Text
- Tuned hyphenation: It is less eager by default and hyphenations close to the
  edges of words are now discouraged more strongly
  **(May lead to larger layout reflows)**
- New default font: Libertinus Serif. This is the maintained successor to the
  old default font Linux Libertine. **(May lead to smaller reflows)**
- Setting the font to an unavailable family will now result in a warning
- Implemented a new smart quote algorithm, fixing various bugs where smart
  quotes weren't all that smart
- Added [`text.costs`] parameter for tweaking various parameters that affect the
  choices of the layout engine during text layout
- Added `typm` highlighting mode for math in [raw blocks]($raw.lang)
- Added basic i18n for Galician, Catalan, Latin, Icelandic, Hebrew
- Implemented hyphenation duplication for Czech, Croatian, Lower Sorbian,
  Polish, Portuguese, Slovak, and Spanish.
- The [`smallcaps`] function is now an element function and can thereby be used
  in show(-set) rules.
- The [`raw.theme`] parameter can now be set to `{none}` to disable highlighting
  even in the presence of a language tag, and to `{auto}` to reset to the
  default
- Multiple [stylistic sets]($text.stylistic-set) can now be enabled at once
- Fixed the Chinese translation for "Equation"
- Fixed that hyphenation could occur outside of words
- Fixed incorrect layout of bidirectional text in edge cases
- Fixed layout of paragraphs with explicit trailing whitespace
- Fixed bugs related to empty paragraphs created via `#""`
- Fixed accidental trailing spaces for line breaks immediately preceding an
  inline equation
- Fixed [`text.historical-ligatures`] not working correctly
- Fixed accidental repetition of Thai characters around line breaks in some
  circumstances
- Fixed [smart quotes]($smartquote) for Swiss French
- New font metadata exceptions for Archivo, Kaiti SC, and Kaiti TC
- Updated bundled New Computer Modern fonts to version 6.0

## Math
- Block-level equations can now break over multiple pages if enabled via
  `{show math.equation: set block(breakable: true)}`.
- Matrix and vector sizing is now more consistent across different cell contents
- Added [`stretch`]($math.stretch) function for manually or automatically
  stretching characters like arrows or parentheses horizontally or vertically
- Improved layout of attachments on parenthesized as well as under- or overlined
  expressions
- Improved layout of nested attachments resulting from code like
  `[#let a0 = $a_0$; $a0^1$]`
- Improved layout of primes close to superscripts
- Improved layout of fractions
- Typst now makes use of math-specific height-dependent kerning information in
  some fonts for better attachment layout
- The `floor` and `ceil` functions in math are now callable symbols, such that
  `[$ floor(x) = lr(floor.l x floor.r) $]`
- The [`mat.delim`]($math.mat.delim), [`vec.delim`]($math.vec.delim), and
  [`cases.delim`]($math.cases.delim) parameters now allow any character that is
  considered a delimiter or "fence" (e.g. |) by Unicode. The `{delim: "||"}`
  notation is _not_ supported anymore and should be replaced by
  `{delim: bar.double}` **(Minor breaking change)**
- Added [`vec.align`]($math.vec.align) and [`mat.align`]($math.mat.align)
  parameters
- Added [`underparen`]($math.underparen), [`overparen`]($math.overparen),
  [`undershell`]($math.undershell), and [`overshell`]($math.overshell)
- Added `~` shorthand for `tilde.op` in math mode **(Minor breaking change)**
- Fixed baseline alignment of equation numbers
- Fixed positioning of corner brackets (⌜, ⌝, ⌞, ⌟)
- Fixed baseline of large roots
- Fixed multiple minor layout bugs with attachments
- Fixed that alignment points could affect line height in math
- Fixed that spaces could show up between text and invisible elements like
  [`metadata`] in math
- Fixed a crash with recursive show rules in math
- Fixed [`lr.size`]($math.lr.size) not affecting characters enclosed in
  [`mid`]($math.mid) in some cases
- Fixed resolving of em units in sub- and superscripts
- Fixed bounding box of inline equations when a [text edge]($text.top-edge) is
  set to `{"bounds"}`

## Introspection
- Implemented a new system by which Typst tracks where elements end up on the
  pages. This may lead to subtly different behavior in introspections.
  **(Breaking change)**
- Fixed various bugs with wrong counter behavior in complex layout situations,
  through a new, more principled implementation
- Counter updates can now be before the first, in between, and after the last
  page when isolated by weak page breaks. This allows, for instance, updating a
  counter before the first page header and background.
- Fixed logical ordering of introspections within footnotes and figures
- Fixed incorrect [`here().position()`]($here) when [`place`] was used in a
  context expression
- Fixed resolved positions of elements (in particular, headings) whose show rule
  emits an invisible element (like a state update) before a page break
- Fixed behavior of stepping a counter at a deeper level than its current state
  has
- Fixed citation formatting not working in table headers and a few other places
- Displaying the footnote counter will now respect the footnote numbering style

## Model
- Document set rules do not need to be at the very start of the document
  anymore. The only restriction is that they must not occur inside of layout
  containers.
- The `spacing` property of [lists]($list.spacing),
  [enumerations]($enum.spacing), and [term lists]($terms.spacing) is now also
  respected for tight lists
- Tight lists now only attach (with tighter spacing) to preceding paragraphs,
  not arbitrary blocks
- The [`quote`] element is now locatable (can be used in queries)
- The bibliography heading now uses `depth` instead of `level` so that its level
  can still be configured via a show-set rule
- Added support for more [numbering] formats: Devanagari, Eastern Arabic,
  Bengali, and circled numbers
- Added [`hanging-indent`]($heading.hanging-indent) parameter to heading
  function to tweak the appearance of multi-line headings and improved default
  appearance of multi-line headings
- Improved handling of bidirectional text in outline entry
- Fixed document set rules being ignored in an otherwise empty document
- Fixed document set rules not being usable in context expressions
- Fixed bad interaction between `{set document}` and `{set page}`
- Fixed `{show figure: set align(..)}`. Since the default figure alignment is
  now a show-set rule, it is not revoked by `{show figure: it => it.body}`
  anymore. **(Minor breaking change)**
- Fixed numbering of footnote references
- Fixed spacing after bibliography heading

## Bibliography
- The Hayagriva YAML `publisher` field can now accept a dictionary with a
  `location` key. The top-level `location` key is now primarily intended for
  event and item locations.
- Multiple page ranges with prefixes and suffixes are now allowed
- Added `director` and catch-all editor types to BibLaTeX parsing
- Added support for disambiguation to alphanumeric citation style
- The year 0 will now render as 1BC
- Fixes for sorting of bibliography entries
- Fixed pluralization of page range labels
- Fixed sorting of citations by their number
- Fixed how citation number ranges collapse
- Fixed when the short form of a title is used
- Fixed parsing of unbalanced dollars in BibLaTeX `url` field
- Updated built-in citation styles

## Visualization
- Added `fill-rule` parameter to [`path`]($path.fill-rule) and
  [`polygon`]($polygon.fill-rule) functions
- Fixed color mixing and gradients for [Luma colors]($color.luma)
- Fixed conversion from Luma to CMYK colors
- Fixed offset gradient strokes in PNG export
- Fixed unintended cropping of some SVGs
- SVGs with foreign objects now produce a warning as they will likely not render
  correctly in Typst

## Syntax
- Added support for nested imports like `{import "file.typ": module.item}`
- Added support for parenthesized imports like `{import "file.typ": (a, b, c)}`.
  With those, the import list can break over multiple lines.
- Fixed edge case in parsing of reference syntax
- Fixed edge case in parsing of heading, list, enum, and term markers
  immediately followed by comments
- Fixed rare crash in parsing of parenthesized expressions

## Scripting
- Added new fixed-point [`decimal`] number type for highly precise arithmetic on
  numbers in base 10, as needed for finance
- Added `std` module for accessing standard library definitions even when a
  variable with the same name shadows/overwrites it
- Added [`array.to-dict`], [`array.reduce`], [`array.windows`] methods
- Added `exact` argument to [`array.zip`]
- Added [`arguments.at`] method
- Added [`int.from-bytes`], [`int.to-bytes`], [`float.from-bytes`], and
  [`float.to-bytes`]
- Added proper support for negative values of the `digits` parameter of
  [`calc.round`] (the behaviour existed before but was subtly broken)
- Conversions from [`int`] to [`float`] will now error instead of saturating if
  the float is too large **(Minor breaking change)**
- Added `float.nan` and `float.inf`, removed `calc.nan`
  **(Minor breaking change)**
- Certain symbols are now generally callable like functions and not only
  specifically in math. Examples are accents or [`floor`]($math.floor) and
  [`ceil`]($math.ceil).
- Improved [`repr`] of relative values, sequences, infinities, NaN,
  `{type(none)}` and `{type(auto)}`
- Fixed crash on whole packages (rather than just files) cyclically importing
  each other
- Fixed return type of [`calc.round`] on integers when a non-zero value is
  provided for `digits`

## Styling
- Text show rules now match across multiple text elements
- The string `{"}` in a text show rule now matches smart quotes
- Fixed a long-standing styling bug where the header and footer would
  incorrectly inherit styles from a lone element on the page (e.g. a heading)
- Fixed `{set page}` not working directly after a counter/state update
- Page fields configured via an explicit `{page(..)[..]}` call can now be
  properly retrieved in context expressions

## Export
- Highly reduced PDF file sizes due to better font subsetting
- Emoji are now exported properly in PDF
- Added initial support for PDF/A. For now, only the standard PDF/A-2b is
  supported, but more is planned for the future. Enabled via `--pdf-standard
  a-2b` in the CLI and via the UI in File > Export as > PDF in the web app.
- Setting [`page.fill`] to `{none}` will now lead to transparent pages instead
  of white ones in PNG and SVG. The new default of `{auto}` means transparent
  for PDF and white for PNG and SVG.
- Improved text copy-paste from PDF in complex scenarios
- Exported SVGs now contain the `data-typst-label` attribute on groups resulting
  from labelled [boxes]($box) and [blocks]($block)
- Fixed a bug where some fonts would not print correctly on professional
  printers
- Fixed a bug where transparency could leak from one PDF object to another
- Fixed a bug with CMYK gradients in PDF
- Fixed various bugs with export of Oklab gradients in PDF
- Fixed crashes related to rendering of non-outline glyphs
- Two small fixes for PDF standard conformance

## Performance
- Typst's layout engine is now multithreaded. Typical speedups are 2-3x for
  larger documents. The multithreading operates on page break boundaries, so
  explicit page breaks are necessary for it to kick in.
- Paragraph justification was optimized with a new two-pass algorithm. Speedups
  are larger for shorter paragraphs and range from 1-6x.

## Command Line Interface
- Added `--pages` option to select specific page ranges to export
- Added `--package-path` and `--package-cache-path` as well as
  `TYPST_PACKAGE_PATH` and `TYPST_PACKAGE_CACHE_PATH` environment variables for
  configuring where packages are loaded from and cached in, respectively
- Added `--ignore-system-fonts` flag to disable system fonts fully for better
  reproducibility
- Added `--make-deps` argument for outputting the dependencies of the current
  compilation as a Makefile
- Added `--pretty` option to `typst query`, with the default now being to minify
  (only applies to JSON format)
- Added `--backup-path` to `typst update` to configure where the previous
  version is backed up
- Added useful links to help output
- The CLI will now greet users who invoke just `typst` for the first time
- The document can now be written to stdout by passing `-` as the output
  filename (for PDF or single-page image export)
- Typst will now emit a proper error message instead of failing silently when
  the certificate specified by `--cert` or `TYPST_CERT` could not be loaded
- The CLI now respects the `SOURCE_DATE_EPOCH` environment variable for better
  reproducibility
- When exporting multiple images, you can now use `{t}` (total pages), `{p}`
  (current page), and `{0p}` (zero-padded current page, same as current `{n}`)
  in the output path
- The input and output paths now allow non-UTF-8 values
- Times are now formatted more consistently across the CLI
- Fixed a bug related to the `--open` flag
- Fixed path completions for `typst` not working in zsh

## Tooling and Diagnostics
- The "compiler" field for specifying the minimum Typst version required by a
  package now supports imprecise bounds like 0.11 instead of 0.11.0
- Added warning when a label is ignored by Typst because no preceding labellable
  element exists
- Added hint when trying to apply labels in code mode
- Added hint when trying to call a standard library function that has been
  shadowed/overwritten by a local definition
- Added hint when trying to set both the language and the region in the `lang`
  parameter
- Added hints when trying to compile non-Typst files (e.g. after having typed
  `typst c file.pdf` by accident)
- Added hint when a string is used where a label is expected
- Added hint when a stray end of a block comment (`*/`) is encountered
- Added hints when destructuring arrays with the wrong number of elements
- Improved error message when trying to use a keyword as an identifier in a let
  binding
- Improved error messages when accessing nonexistent fields
- Improved error message when a package exists, but not the specified version
- Improved hints for unknown variables
- Improved hint when trying to convert a length with non-zero em component to an
  absolute unit
- Fixed a crash that could be triggered by certain hover tooltips
- Fixed an off-by-one error in to-source jumps when first-line-indent is enabled
- Fixed suggestions for `.` after the end of an inline code expressions
- Fixed autocompletions being duplicated in a specific case

## Symbols
- New: `parallelogram`, `original`, `image`, `crossmark`, `rest`, `natural`,
  `flat`, `sharp`, `tiny`, `miny`, `copyleft`, `trademark`, `emoji.beet`,
  `emoji.fingerprint`, `emoji.harp`, `emoji.shovel`, `emoji.splatter`,
  `emoji.tree.leafless`,
- New variants: `club.stroked`, `diamond.stroked`, `heart.stroked`,
  `spade.stroked`, `gt.neq`, `lt.neq`, `checkmark.heavy`, `paren.double`,
  `brace.double`, `shell.double`, `arrow.turn`, `plus.double`, `plus.triple`,
  `infinity.bar`, `infinity.incomplete`, `infinity.tie`, `multimap.double`,
  `ballot.check`, `ballot.check.heavy`, `emptyset.bar`, `emptyset.circle`,
  `emptyset.arrow.l`, `emptyset.arrow.r`, `parallel.struck`, `parallel.eq`,
  `parallel.equiv`, `parallel.slanted`, `parallel.tilde`, `angle.l.curly`,
  `angle.l.dot`, `angle.r.curly`, `angle.r.dot`, `angle.oblique`, `angle.s`,
  `em.two`, `em.three`
- Renamed: `turtle` to `shell`, `notes` to `note`, `ballot.x` to `ballot.cross`,
  `succ.eq` to `succ.curly.eq`, `prec.eq` to `prec.curly.eq`, `servicemark` to
  `trademark.service`, `emoji.face.tired` to `emoji.face.distress`
  **(Breaking change)**
- Changed codepoint: `prec.eq`, `prec.neq`, `succ.eq`, `succ.neq`, `triangle`
  from ▷ to △, `emoji.face.tired` **(Breaking change)**
- Removed: `lt.curly` in favor of `prec`, `gt.curly` in favor of `succ`
  **(Breaking change)**

## Deprecations
- [`counter.display`] without an established context
- [`counter.final`] with a location
- [`state.final`] with a location
- [`state.display`]
- [`query`] with a location as the second argument
- [`locate`] with a callback function
- [`measure`] with styles
- [`style`]

## Development
- Added `typst-kit` crate which provides useful APIs for `World` implementors
- Added go-to-definition API in `typst-ide`
- Added package manifest parsing APIs to `typst-syntax`
- As the compiler is now capable of multithreading, `World` implementations must
  satisfy `Send` and `Sync`
- Changed signature of `World::main` to allow for the scenario where the main
  file could not be loaded
- Removed `Tracer` in favor of `Warned<T>` and `typst::trace` function
- The `xz2` dependency used by the self-updater is now statically linked
- The Dockerfile now has an `ENTRYPOINT` directive