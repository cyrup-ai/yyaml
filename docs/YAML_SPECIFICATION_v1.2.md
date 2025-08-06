---
title: "YAML Ain’t Markup Language (YAML™) revision 1.2.2"
source: "https://yaml.org/spec/1.2.2/"
author:
published:
created: 2025-08-06
description:
tags:
  - "clippings"
---
## YAML Ain’t Markup Language (YAML™) version 1.2

## Revision 1.2.2 (2021-10-01)

Copyright presently by YAML Language Development Team [^1]  
Copyright 2001-2009 by Oren Ben-Kiki, Clark Evans, Ingy döt Net

This document may be freely copied, provided it is not modified.

**Status of this Document**

This is the **YAML specification v1.2.2**. It defines the **YAML 1.2 data language**. There are no normative changes from the **YAML specification v1.2**. The primary objectives of this revision are to correct errors and add clarity.

This revision also strives to make the YAML language development process more open, more transparent and easier for people to contribute to. The input format is now Markdown instead of DocBook, and the images are made from plain text LaTeX files rather than proprietary drawing software. All the source content for the specification is publicly hosted [^2].

The previous YAML specification [^3] was published 12 years ago. In that time span, YAML’s popularity has grown significantly. Efforts are ongoing to improve the language and grow it to meet the needs and expectations of its users. While this revision of the specification makes no actual changes to YAML, it begins a process by which the language intends to evolve and stay modern.

The YAML specification is often seen as overly complicated for something which appears to be so simple. Even though YAML often is used for software configuration, it has always been and will continue to be a complete data serialization language. Future YAML plans are focused on making the language and ecosystem more powerful and reliable while simultaneously simplifying the development process for implementers.

While this revision of the specification is limiting itself to informational changes only, there is companion documentation intended to guide YAML framework implementers and YAML language users. This documentation can continue to evolve and expand continually between published revisions of this specification.

See:

- [YAML Resources Index](https://yaml.org/spec/1.2.2/ext/resources)
- [YAML Vocabulary Glossary](https://yaml.org/spec/1.2.2/ext/glossary)
- [YAML Specification Changes](https://yaml.org/spec/1.2.2/ext/changes)
- [YAML Specification Errata](https://yaml.org/spec/1.2.2/ext/errata)

**Abstract**

YAML™ (rhymes with “camel”) is a human-friendly, cross language, Unicode based data serialization language designed around the common native data types of dynamic programming languages. It is broadly useful for programming needs ranging from configuration files to internet messaging to object persistence to data auditing and visualization. Together with the Unicode standard for characters [^4], this specification provides all the information necessary to understand YAML version 1.2 and to create programs that process YAML information.

**Contents**

- [YAML Ain’t Markup Language (YAML™) version 1.2](https://yaml.org/spec/1.2.2/#yaml-aint-markup-language-yaml-version-12)
	- [Revision 1.2.2 (2021-10-01)](https://yaml.org/spec/1.2.2/#revision-122-2021-10-01)
- [Chapter 1. Introduction to YAML](https://yaml.org/spec/1.2.2/#chapter-1-introduction-to-yaml)
	- [1.1. Goals](https://yaml.org/spec/1.2.2/#11-goals)
	- [1.2. YAML History](https://yaml.org/spec/1.2.2/#12-yaml-history)
	- [1.3. Terminology](https://yaml.org/spec/1.2.2/#13-terminology)
- [Chapter 2. Language Overview](https://yaml.org/spec/1.2.2/#chapter-2-language-overview)
	- [2.2. Structures](https://yaml.org/spec/1.2.2/#22-structures)
	- [2.3. Scalars](https://yaml.org/spec/1.2.2/#23-scalars)
	- [2.4. Tags](https://yaml.org/spec/1.2.2/#24-tags)
	- [2.5. Full Length Example](https://yaml.org/spec/1.2.2/#25-full-length-example)
- [Chapter 3. Processes and Models](https://yaml.org/spec/1.2.2/#chapter-3-processes-and-models)
	- [3.1. Processes](https://yaml.org/spec/1.2.2/#31-processes)
		- [3.1.1. Dump](https://yaml.org/spec/1.2.2/#311-dump)
		- [3.1.2. Load](https://yaml.org/spec/1.2.2/#312-load)
	- [3.2. Information Models](https://yaml.org/spec/1.2.2/#32-information-models)
		- [3.2.1. Representation Graph](https://yaml.org/spec/1.2.2/#321-representation-graph)
			- [3.2.1.1. Nodes](https://yaml.org/spec/1.2.2/#3211-nodes)
			- [3.2.1.2. Tags](https://yaml.org/spec/1.2.2/#3212-tags)
			- [3.2.1.3. Node Comparison](https://yaml.org/spec/1.2.2/#3213-node-comparison)
		- [3.2.2. Serialization Tree](https://yaml.org/spec/1.2.2/#322-serialization-tree)
			- [3.2.2.1. Mapping Key Order](https://yaml.org/spec/1.2.2/#3221-mapping-key-order)
			- [3.2.2.2. Anchors and Aliases](https://yaml.org/spec/1.2.2/#3222-anchors-and-aliases)
		- [3.2.3. Presentation Stream](https://yaml.org/spec/1.2.2/#323-presentation-stream)
			- [3.2.3.1. Node Styles](https://yaml.org/spec/1.2.2/#3231-node-styles)
			- [3.2.3.2. Scalar Formats](https://yaml.org/spec/1.2.2/#3232-scalar-formats)
			- [3.2.3.4. Directives](https://yaml.org/spec/1.2.2/#3234-directives)
	- - [3.3.1. Well-Formed Streams and Identified Aliases](https://yaml.org/spec/1.2.2/#331-well-formed-streams-and-identified-aliases)
		- [3.3.2. Resolved Tags](https://yaml.org/spec/1.2.2/#332-resolved-tags)
		- [3.3.3. Recognized and Valid Tags](https://yaml.org/spec/1.2.2/#333-recognized-and-valid-tags)
		- [3.3.4. Available Tags](https://yaml.org/spec/1.2.2/#334-available-tags)
- [Chapter 4. Syntax Conventions](https://yaml.org/spec/1.2.2/#chapter-4-syntax-conventions)
	- [4.1. Production Syntax](https://yaml.org/spec/1.2.2/#41-production-syntax)
	- [4.2. Production Parameters](https://yaml.org/spec/1.2.2/#42-production-parameters)
	- [4.3. Production Naming Conventions](https://yaml.org/spec/1.2.2/#43-production-naming-conventions)
- [Chapter 5. Character Productions](https://yaml.org/spec/1.2.2/#chapter-5-character-productions)
	- [5.1. Character Set](https://yaml.org/spec/1.2.2/#51-character-set)
	- [5.2. Character Encodings](https://yaml.org/spec/1.2.2/#52-character-encodings)
	- [5.3. Indicator Characters](https://yaml.org/spec/1.2.2/#53-indicator-characters)
	- [5.4. Line Break Characters](https://yaml.org/spec/1.2.2/#54-line-break-characters)
	- [5.5. White Space Characters](https://yaml.org/spec/1.2.2/#55-white-space-characters)
	- [5.6. Miscellaneous Characters](https://yaml.org/spec/1.2.2/#56-miscellaneous-characters)
	- [5.7. Escaped Characters](https://yaml.org/spec/1.2.2/#57-escaped-characters)
- [Chapter 6. Structural Productions](https://yaml.org/spec/1.2.2/#chapter-6-structural-productions)
	- [6.1. Indentation Spaces](https://yaml.org/spec/1.2.2/#61-indentation-spaces)
	- [6.2. Separation Spaces](https://yaml.org/spec/1.2.2/#62-separation-spaces)
	- [6.3. Line Prefixes](https://yaml.org/spec/1.2.2/#63-line-prefixes)
	- [6.4. Empty Lines](https://yaml.org/spec/1.2.2/#64-empty-lines)
	- [6.5. Line Folding](https://yaml.org/spec/1.2.2/#65-line-folding)
	- [6.7. Separation Lines](https://yaml.org/spec/1.2.2/#67-separation-lines)
	- [6.8. Directives](https://yaml.org/spec/1.2.2/#68-directives)
		- [6.8.1. “ `YAML` ” Directives](https://yaml.org/spec/1.2.2/#681-yaml-directives)
		- [6.8.2. “ `TAG` ” Directives](https://yaml.org/spec/1.2.2/#682-tag-directives)
			- [6.8.2.1. Tag Handles](https://yaml.org/spec/1.2.2/#6821-tag-handles)
			- [6.8.2.2. Tag Prefixes](https://yaml.org/spec/1.2.2/#6822-tag-prefixes)
	- [6.9. Node Properties](https://yaml.org/spec/1.2.2/#69-node-properties)
		- [6.9.1. Node Tags](https://yaml.org/spec/1.2.2/#691-node-tags)
		- [6.9.2. Node Anchors](https://yaml.org/spec/1.2.2/#692-node-anchors)
- [Chapter 7. Flow Style Productions](https://yaml.org/spec/1.2.2/#chapter-7-flow-style-productions)
	- [7.1. Alias Nodes](https://yaml.org/spec/1.2.2/#71-alias-nodes)
	- [7.2. Empty Nodes](https://yaml.org/spec/1.2.2/#72-empty-nodes)
	- [7.3. Flow Scalar Styles](https://yaml.org/spec/1.2.2/#73-flow-scalar-styles)
		- [7.3.1. Double-Quoted Style](https://yaml.org/spec/1.2.2/#731-double-quoted-style)
		- [7.3.2. Single-Quoted Style](https://yaml.org/spec/1.2.2/#732-single-quoted-style)
		- [7.3.3. Plain Style](https://yaml.org/spec/1.2.2/#733-plain-style)
	- [7.4. Flow Collection Styles](https://yaml.org/spec/1.2.2/#74-flow-collection-styles)
		- [7.4.1. Flow Sequences](https://yaml.org/spec/1.2.2/#741-flow-sequences)
		- [7.4.2. Flow Mappings](https://yaml.org/spec/1.2.2/#742-flow-mappings)
	- [7.5. Flow Nodes](https://yaml.org/spec/1.2.2/#75-flow-nodes)
- [Chapter 8. Block Style Productions](https://yaml.org/spec/1.2.2/#chapter-8-block-style-productions)
	- [8.1. Block Scalar Styles](https://yaml.org/spec/1.2.2/#81-block-scalar-styles)
		- [8.1.1. Block Scalar Headers](https://yaml.org/spec/1.2.2/#811-block-scalar-headers)
			- [8.1.1.1. Block Indentation Indicator](https://yaml.org/spec/1.2.2/#8111-block-indentation-indicator)
			- [8.1.1.2. Block Chomping Indicator](https://yaml.org/spec/1.2.2/#8112-block-chomping-indicator)
		- [8.1.2. Literal Style](https://yaml.org/spec/1.2.2/#812-literal-style)
		- [8.1.3. Folded Style](https://yaml.org/spec/1.2.2/#813-folded-style)
	- [8.2. Block Collection Styles](https://yaml.org/spec/1.2.2/#82-block-collection-styles)
		- [8.2.1. Block Sequences](https://yaml.org/spec/1.2.2/#821-block-sequences)
		- [8.2.2. Block Mappings](https://yaml.org/spec/1.2.2/#822-block-mappings)
		- [8.2.3. Block Nodes](https://yaml.org/spec/1.2.2/#823-block-nodes)
- [Chapter 9. Document Stream Productions](https://yaml.org/spec/1.2.2/#chapter-9-document-stream-productions)
	- [9.1. Documents](https://yaml.org/spec/1.2.2/#91-documents)
		- [9.1.1. Document Prefix](https://yaml.org/spec/1.2.2/#911-document-prefix)
		- [9.1.2. Document Markers](https://yaml.org/spec/1.2.2/#912-document-markers)
		- [9.1.3. Bare Documents](https://yaml.org/spec/1.2.2/#913-bare-documents)
		- [9.1.4. Explicit Documents](https://yaml.org/spec/1.2.2/#914-explicit-documents)
		- [9.1.5. Directives Documents](https://yaml.org/spec/1.2.2/#915-directives-documents)
	- [9.2. Streams](https://yaml.org/spec/1.2.2/#92-streams)
- - [10.1. Failsafe Schema](https://yaml.org/spec/1.2.2/#101-failsafe-schema)
		- [10.1.1. Tags](https://yaml.org/spec/1.2.2/#1011-tags)
			- [10.1.1.1. Generic Mapping](https://yaml.org/spec/1.2.2/#10111-generic-mapping)
			- [10.1.1.2. Generic Sequence](https://yaml.org/spec/1.2.2/#10112-generic-sequence)
			- [10.1.1.3. Generic String](https://yaml.org/spec/1.2.2/#10113-generic-string)
		- [10.1.2. Tag Resolution](https://yaml.org/spec/1.2.2/#1012-tag-resolution)
	- [10.2. JSON Schema](https://yaml.org/spec/1.2.2/#102-json-schema)
		- [10.2.1. Tags](https://yaml.org/spec/1.2.2/#1021-tags)
			- [10.2.1.1. Null](https://yaml.org/spec/1.2.2/#10211-null)
			- [10.2.1.2. Boolean](https://yaml.org/spec/1.2.2/#10212-boolean)
			- [10.2.1.3. Integer](https://yaml.org/spec/1.2.2/#10213-integer)
			- [10.2.1.4. Floating Point](https://yaml.org/spec/1.2.2/#10214-floating-point)
		- [10.2.2. Tag Resolution](https://yaml.org/spec/1.2.2/#1022-tag-resolution)
	- [10.3. Core Schema](https://yaml.org/spec/1.2.2/#103-core-schema)
		- [10.3.1. Tags](https://yaml.org/spec/1.2.2/#1031-tags)
		- [10.3.2. Tag Resolution](https://yaml.org/spec/1.2.2/#1032-tag-resolution)
	- [10.4. Other Schemas](https://yaml.org/spec/1.2.2/#104-other-schemas)
- [Reference Links](https://yaml.org/spec/1.2.2/#reference-links)

## Chapter 1. Introduction to YAML

YAML (a recursive acronym for “YAML Ain’t Markup Language”) is a data serialization language designed to be human-friendly and work well with modern programming languages for common everyday tasks. This specification is both an introduction to the YAML language and the concepts supporting it. It is also a complete specification of the information needed to develop [applications](https://yaml.org/spec/1.2.2/#processes-and-models) for processing YAML.

Open, interoperable and readily understandable tools have advanced computing immensely. YAML was designed from the start to be useful and friendly to people working with data. It uses Unicode [printable](https://yaml.org/spec/1.2.2/#character-set) characters, [some](https://yaml.org/spec/1.2.2/#indicator-characters) of which provide structural information and the rest containing the data itself. YAML achieves a unique cleanness by minimizing the amount of structural characters and allowing the data to show itself in a natural and meaningful way. For example, [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) may be used for structure, [colons](https://yaml.org/spec/1.2.2/#flow-mappings) separate [key/value pairs](https://yaml.org/spec/1.2.2/#mapping) and [dashes](https://yaml.org/spec/1.2.2/#block-sequences) are used to create “bulleted” [lists](https://yaml.org/spec/1.2.2/#sequence).

There are many kinds of [data structures](https://yaml.org/spec/1.2.2/#dump), but they can all be adequately [represented](https://yaml.org/spec/1.2.2/#representation-graph) with three basic primitives: [mappings](https://yaml.org/spec/1.2.2/#mapping) (hashes/dictionaries),[sequences](https://yaml.org/spec/1.2.2/#sequence) (arrays/lists) and [scalars](https://yaml.org/spec/1.2.2/#scalars) (strings/numbers). YAML leverages these primitives and adds a simple typing system and [aliasing](https://yaml.org/spec/1.2.2/#anchors-and-aliases) mechanism to form a complete language for [serializing](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) any [native data structure](https://yaml.org/spec/1.2.2/#representing-native-data-structures). While most programming languages can use YAML for data serialization, YAML excels in working with those languages that are fundamentally built around the three basic primitives. These include common dynamic languages such as JavaScript, Perl, PHP, Python and Ruby.

There are hundreds of different languages for programming, but only a handful of languages for storing and transferring data. Even though its potential is virtually boundless, YAML was specifically created to work well for common use cases such as: configuration files, log files, interprocess messaging, cross-language data sharing, object persistence and debugging of complex data structures. When data is easy to view and understand, programming becomes a simpler task.

## 1.1. Goals

The design goals for YAML are, in decreasing priority:

1. YAML should be easily readable by humans.
2. YAML data should be portable between programming languages.
3. YAML should match the [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) of dynamic languages.
4. YAML should have a consistent model to support generic tools.
5. YAML should support one-pass processing.
6. YAML should be expressive and extensible.
7. YAML should be easy to implement and use.

## 1.2. YAML History

The YAML 1.0 specification was published in early 2004 by by Clark Evans, Oren Ben-Kiki, and Ingy döt Net after 3 years of collaborative design work through the yaml-core mailing list [^5]. The project was initially rooted in Clark and Oren’s work on the SML-DEV [^6] mailing list (for simplifying XML) and Ingy’s plain text serialization module [^7] for Perl. The language took a lot of inspiration from many other technologies and formats that preceded it.

The first YAML framework was written in Perl in 2001 and Ruby was the first language to ship a YAML framework as part of its core language distribution in 2003.

The YAML 1.1 [^8] specification was published in 2005. Around this time, the developers became aware of JSON [^9]. By sheer coincidence, JSON was almost a complete subset of YAML (both syntactically and semantically).

In 2006, Kyrylo Simonov produced PyYAML [^10] and LibYAML [^11]. A lot of the YAML frameworks in various programming languages are built over LibYAML and many others have looked to PyYAML as a solid reference for their implementations.

The YAML 1.2 [^1] specification was published in 2009. Its primary focus was making YAML a strict superset of JSON. It also removed many of the problematic implicit typing recommendations.

Since the release of the 1.2 specification, YAML adoption has continued to grow, and many large-scale projects use it as their primary interface language. In 2020, the new [YAML language design team](https://yaml.org/spec/1.2.2/ext/team) began meeting regularly to discuss improvements to the YAML language and specification; to better meet the needs and expectations of its users and use cases.

This YAML 1.2.2 specification, published in October 2021, is the first step in YAML’s rejuvenated development journey. YAML is now more popular than it has ever been, but there is a long list of things that need to be addressed for it to reach its full potential. The YAML design team is focused on making YAML as good as possible.

## 1.3. Terminology

The key words “MUST”, “MUST NOT”, “REQUIRED”, “SHALL”, “SHALL NOT”, “SHOULD”, “SHOULD NOT”, “RECOMMENDED”, “MAY”, and “OPTIONAL” in this document are to be interpreted as described in RFC 2119 [^12].

The rest of this document is arranged as follows. Chapter [2](https://yaml.org/spec/1.2.2/#language-overview) provides a short preview of the main YAML features. Chapter [3](https://yaml.org/spec/1.2.2/#processes-and-models) describes the YAML information model and the processes for converting from and to this model and the YAML text format. The bulk of the document, chapters [4](https://yaml.org/spec/1.2.2/#syntax-conventions), [5](https://yaml.org/spec/1.2.2/#character-productions), [6](https://yaml.org/spec/1.2.2/#structural-productions), [7](https://yaml.org/spec/1.2.2/#flow-style-productions), [8](https://yaml.org/spec/1.2.2/#block-style-productions) and [9](https://yaml.org/spec/1.2.2/#document-stream-productions), formally define this text format. Finally, chapter [10](https://yaml.org/spec/1.2.2/#recommended-schemas) recommends basic YAML schemas.

## Chapter 2. Language Overview

This section provides a quick glimpse into the expressive power of YAML. It is not expected that the first-time reader grok all of the examples. Rather, these selections are used as motivation for the remainder of the specification.

YAML’s [block collections](https://yaml.org/spec/1.2.2/#block-collection-styles) use [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) for scope and begin each entry on its own line.[Block sequences](https://yaml.org/spec/1.2.2/#block-sequences) indicate each entry with a dash and space (“ `- ` ”).[Mappings](https://yaml.org/spec/1.2.2/#mapping) use a colon and space (“`: `”) to mark each [key/value pair](https://yaml.org/spec/1.2.2/#mapping).[Comments](https://yaml.org/spec/1.2.2/#comments) begin with an octothorpe (also called a “hash”, “sharp”, “pound” or “number sign” - “ `#` ”).

**Example 2.1 Sequence of Scalars (ball players)**

```
- Mark McGwire
- Sammy Sosa
- Ken Griffey
```

**Example 2.2 Mapping Scalars to Scalars (player statistics)**

```
hr:  65    # Home runs
avg: 0.278 # Batting average
rbi: 147   # Runs Batted In
```

**Example 2.3 Mapping Scalars to Sequences (ball clubs in each league)**

```
american:
- Boston Red Sox
- Detroit Tigers
- New York Yankees
national:
- New York Mets
- Chicago Cubs
- Atlanta Braves
```

**Example 2.4 Sequence of Mappings (players’ statistics)**

```
-
  name: Mark McGwire
  hr:   65
  avg:  0.278
-
  name: Sammy Sosa
  hr:   63
  avg:  0.288
```

YAML also has [flow styles](https://yaml.org/spec/1.2.2/#flow-style-productions), using explicit [indicators](https://yaml.org/spec/1.2.2/#indicator-characters) rather than [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) to denote scope. The [flow sequence](https://yaml.org/spec/1.2.2/#flow-sequences) is written as a [comma](https://yaml.org/spec/1.2.2/#flow-collection-styles) separated list within [square](https://yaml.org/spec/1.2.2/#flow-sequences) [brackets](https://yaml.org/spec/1.2.2/#flow-sequences). In a similar manner, the [flow mapping](https://yaml.org/spec/1.2.2/#flow-mappings) uses [curly](https://yaml.org/spec/1.2.2/#flow-mappings) [braces](https://yaml.org/spec/1.2.2/#flow-mappings).

**Example 2.5 Sequence of Sequences**

```
- [name        , hr, avg  ]
- [Mark McGwire, 65, 0.278]
- [Sammy Sosa  , 63, 0.288]
```

**Example 2.6 Mapping of Mappings**

```
Mark McGwire: {hr: 65, avg: 0.278}
Sammy Sosa: {
    hr: 63,
    avg: 0.288,
 }
```

## 2.2. Structures

YAML uses three dashes (“ `---` ”) to separate [directives](https://yaml.org/spec/1.2.2/#directives) from [document](https://yaml.org/spec/1.2.2/#documents) [content](https://yaml.org/spec/1.2.2/#nodes). This also serves to signal the start of a document if no [directives](https://yaml.org/spec/1.2.2/#directives) are present. Three dots ( “`...`”) indicate the end of a document without starting a new one, for use in communication channels.

**Example 2.7 Two Documents in a Stream (each with a leading comment)**

```
# Ranking of 1998 home runs
---
- Mark McGwire
- Sammy Sosa
- Ken Griffey

# Team ranking
---
- Chicago Cubs
- St Louis Cardinals
```

**Example 2.8 Play by Play Feed from a Game**

```
---
time: 20:03:20
player: Sammy Sosa
action: strike (miss)
...
---
time: 20:03:47
player: Sammy Sosa
action: grand slam
...
```

Repeated [nodes](https://yaml.org/spec/1.2.2/#nodes) (objects) are first [identified](https://yaml.org/spec/1.2.2/#anchors-and-aliases) by an [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases) (marked with the ampersand - “ `&` ”) and are then [aliased](https://yaml.org/spec/1.2.2/#anchors-and-aliases) (referenced with an asterisk - “ `*` ”) thereafter.

**Example 2.10 Node for “ `Sammy Sosa` ” appears twice in this document**

```
---
hr:
- Mark McGwire
# Following node labeled SS
- &SS Sammy Sosa
rbi:
- *SS # Subsequent occurrence
- Ken Griffey
```

A question mark and space (“`? `”) indicate a complex [mapping](https://yaml.org/spec/1.2.2/#mapping) [key](https://yaml.org/spec/1.2.2/#nodes). Within a [block collection](https://yaml.org/spec/1.2.2/#block-collection-styles), [key/value pairs](https://yaml.org/spec/1.2.2/#mapping) can start immediately following the [dash](https://yaml.org/spec/1.2.2/#block-sequences), [colon](https://yaml.org/spec/1.2.2/#flow-mappings) or [question mark](https://yaml.org/spec/1.2.2/#flow-mappings).

**Example 2.11 Mapping between Sequences**

```
? - Detroit Tigers
  - Chicago cubs
: - 2001-07-23

? [ New York Yankees,
    Atlanta Braves ]
: [ 2001-07-02, 2001-08-12,
    2001-08-14 ]
```

**Example 2.12 Compact Nested Mapping**

```
---
# Products purchased
- item    : Super Hoop
  quantity: 1
- item    : Basketball
  quantity: 4
- item    : Big Shoes
  quantity: 1
```

## 2.3. Scalars

[Scalar content](https://yaml.org/spec/1.2.2/#scalar) can be written in [block](https://yaml.org/spec/1.2.2/#scalars) notation, using a [literal style](https://yaml.org/spec/1.2.2/#literal-style) (indicated by “ `|` ”) where all [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) are significant. Alternatively, they can be written with the [folded style](https://yaml.org/spec/1.2.2/#folded-style) (denoted by “ `>` ”) where each [line break](https://yaml.org/spec/1.2.2/#line-break-characters) is [folded](https://yaml.org/spec/1.2.2/#line-folding) to a [space](https://yaml.org/spec/1.2.2/#white-space-characters) unless it ends an [empty](https://yaml.org/spec/1.2.2/#empty-lines) or a [more-indented](https://yaml.org/spec/1.2.2/#example-more-indented-lines) line.

**Example 2.13 In literals, newlines are preserved**

```
# ASCII Art
--- |
  \//||\/||
  // ||  ||__
```

**Example 2.14 In the folded scalars, newlines become spaces**

```
--- >
  Mark McGwire's
  year was crippled
  by a knee injury.
```

**Example 2.16 Indentation determines scope**

```
name: Mark McGwire
accomplishment: >
  Mark set a major league
  home run record in 1998.
stats: |
  65 Home Runs
  0.278 Batting Average
```

YAML’s [flow scalars](https://yaml.org/spec/1.2.2/#flow-scalar-styles) include the [plain style](https://yaml.org/spec/1.2.2/#plain-style) (most examples thus far) and two quoted styles. The [double-quoted style](https://yaml.org/spec/1.2.2/#double-quoted-style) provides [escape sequences](https://yaml.org/spec/1.2.2/#escaped-characters). The [single-quoted style](https://yaml.org/spec/1.2.2/#single-quoted-style) is useful when [escaping](https://yaml.org/spec/1.2.2/#escaped-characters) is not needed. All [flow scalars](https://yaml.org/spec/1.2.2/#flow-scalar-styles) can span multiple lines; [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) are always [folded](https://yaml.org/spec/1.2.2/#line-folding).

**Example 2.17 Quoted Scalars**

```
unicode: "Sosa did fine.\u263A"
control: "\b1998\t1999\t2000\n"
hex esc: "\x0d\x0a is \r\n"

single: '"Howdy!" he cried.'
quoted: ' # Not a ''comment''.'
tie-fighter: '|\-*-/|'
```

**Example 2.18 Multi-line Flow Scalars**

```
plain:
  This unquoted scalar
  spans many lines.

quoted: "So does this
  quoted scalar.\n"
```

## 2.4. Tags

In YAML, [untagged nodes](https://yaml.org/spec/1.2.2/#resolved-tags) are given a type depending on the [application](https://yaml.org/spec/1.2.2/#processes-and-models). The examples in this specification generally use the `seq`, `map` and `str` types from the [fail safe schema](https://yaml.org/spec/1.2.2/#failsafe-schema). A few examples also use the `int`, `float` and `null` types from the [JSON schema](https://yaml.org/spec/1.2.2/#json-schema).

**Example 2.19 Integers**

```
canonical: 12345
decimal: +12345
octal: 0o14
hexadecimal: 0xC
```

**Example 2.20 Floating Point**

```
canonical: 1.23015e+3
exponential: 12.3015e+02
fixed: 1230.15
negative infinity: -.inf
not a number: .nan
```

**Example 2.21 Miscellaneous**

```
null:
booleans: [ true, false ]
string: '012345'
```

Explicit typing is denoted with a [tag](https://yaml.org/spec/1.2.2/#tags) using the exclamation point (“`!`”) symbol.[Global tags](https://yaml.org/spec/1.2.2/#tags) are URIs and may be specified in a [tag shorthand](https://yaml.org/spec/1.2.2/#tag-shorthands) notation using a [handle](https://yaml.org/spec/1.2.2/#tag-handles).[Application](https://yaml.org/spec/1.2.2/#processes-and-models) -specific [local tags](https://yaml.org/spec/1.2.2/#tags) may also be used.

**Example 2.23 Various Explicit Tags**

```
---
not-date: !!str 2002-04-28

picture: !!binary |
 R0lGODlhDAAMAIQAAP//9/X
 17unp5WZmZgAAAOfn515eXv
 Pz7Y6OjuDg4J+fn5OTk6enp
 56enmleECcgggoBADs=

application specific tag: !something |
 The semantics of the tag
 above may be different for
 different documents.
```

**Example 2.24 Global Tags**

```
%TAG ! tag:clarkevans.com,2002:
--- !shape
  # Use the ! handle for presenting
  # tag:clarkevans.com,2002:circle
- !circle
  center: &ORIGIN {x: 73, y: 129}
  radius: 7
- !line
  start: *ORIGIN
  finish: { x: 89, y: 102 }
- !label
  start: *ORIGIN
  color: 0xFFEEBB
  text: Pretty vector drawing.
```

**Example 2.25 Unordered Sets**

```
# Sets are represented as a
# Mapping where each key is
# associated with a null value
--- !!set
? Mark McGwire
? Sammy Sosa
? Ken Griffey
```

**Example 2.26 Ordered Mappings**

```
# Ordered maps are represented as
# A sequence of mappings, with
# each mapping having one key
--- !!omap
- Mark McGwire: 65
- Sammy Sosa: 63
- Ken Griffey: 58
```

## 2.5. Full Length Example

Below are two full-length examples of YAML. The first is a sample invoice; the second is a sample log file.

**Example 2.27 Invoice**

```
--- !<tag:clarkevans.com,2002:invoice>
invoice: 34843
date   : 2001-01-23
bill-to: &id001
  given  : Chris
  family : Dumars
  address:
    lines: |
      458 Walkman Dr.
      Suite #292
    city    : Royal Oak
    state   : MI
    postal  : 48046
ship-to: *id001
product:
- sku         : BL394D
  quantity    : 4
  description : Basketball
  price       : 450.00
- sku         : BL4438H
  quantity    : 1
  description : Super Hoop
  price       : 2392.00
tax  : 251.42
total: 4443.52
comments:
  Late afternoon is best.
  Backup contact is Nancy
  Billsmer @ 338-4338.
```

**Example 2.28 Log File**

```
---
Time: 2001-11-23 15:01:42 -5
User: ed
Warning:
  This is an error message
  for the log file
---
Time: 2001-11-23 15:02:31 -5
User: ed
Warning:
  A slightly different error
  message.
---
Date: 2001-11-23 15:03:17 -5
User: ed
Fatal:
  Unknown variable "bar"
Stack:
- file: TopClass.py
  line: 23
  code: |
    x = MoreObject("345\n")
- file: MoreClass.py
  line: 58
  code: |-
    foo = bar
```

## Chapter 3. Processes and Models

YAML is both a text format and a method for [presenting](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) any [native data structure](https://yaml.org/spec/1.2.2/#representing-native-data-structures) in this format. Therefore, this specification defines two concepts: a class of data objects called YAML [representations](https://yaml.org/spec/1.2.2/#representation-graph) and a syntax for [presenting](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) YAML [representations](https://yaml.org/spec/1.2.2/#representation-graph) as a series of characters, called a YAML [stream](https://yaml.org/spec/1.2.2/#streams).

A YAML *processor* is a tool for converting information between these complementary views. It is assumed that a YAML processor does its work on behalf of another module, called an *application*. This chapter describes the information structures a YAML processor must provide to or obtain from the application.

YAML information is used in two ways: for machine processing and for human consumption. The challenge of reconciling these two perspectives is best done in three distinct translation stages: [representation](https://yaml.org/spec/1.2.2/#representation-graph), [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) and [presentation](https://yaml.org/spec/1.2.2/#presentation-stream).[Representation](https://yaml.org/spec/1.2.2/#representation-graph) addresses how YAML views [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) to achieve portability between programming environments.[Serialization](https://yaml.org/spec/1.2.2/#serialization-tree) concerns itself with turning a YAML [representation](https://yaml.org/spec/1.2.2/#representation-graph) into a serial form, that is, a form with sequential access constraints.[Presentation](https://yaml.org/spec/1.2.2/#presentation-stream) deals with the formatting of a YAML [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) as a series of characters in a human-friendly manner.

## 3.1. Processes

Translating between [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) and a character [stream](https://yaml.org/spec/1.2.2/#streams) is done in several logically distinct stages, each with a well defined input and output data model, as shown in the following diagram:

**Figure 3.1. Processing Overview**

![Processing Overview](https://yaml.org/spec/1.2.2/img/overview2.svg)

A YAML processor need not expose the [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) or [representation](https://yaml.org/spec/1.2.2/#representation-graph) stages. It may translate directly between [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) and a character [stream](https://yaml.org/spec/1.2.2/#streams) ([dump](https://yaml.org/spec/1.2.2/#dump) and [load](https://yaml.org/spec/1.2.2/#load) in the diagram above). However, such a direct translation should take place so that the [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) are [constructed](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) only from information available in the [representation](https://yaml.org/spec/1.2.2/#representation-graph). In particular, [mapping key order](https://yaml.org/spec/1.2.2/#mapping), [comments](https://yaml.org/spec/1.2.2/#comments) and [tag handles](https://yaml.org/spec/1.2.2/#tag-handles) should not be referenced during [construction](https://yaml.org/spec/1.2.2/#constructing-native-data-structures).

### 3.1.1. Dump

*Dumping* native data structures to a character [stream](https://yaml.org/spec/1.2.2/#streams) is done using the following three stages:

Representing Native Data Structures

YAML *represents* any *native data structure* using three [node kinds](https://yaml.org/spec/1.2.2/#nodes):[sequence](https://yaml.org/spec/1.2.2/#sequence) - an ordered series of entries; [mapping](https://yaml.org/spec/1.2.2/#mapping) - an unordered association of [unique](https://yaml.org/spec/1.2.2/#node-comparison) [keys](https://yaml.org/spec/1.2.2/#nodes) to [values](https://yaml.org/spec/1.2.2/#nodes); and [scalar](https://yaml.org/spec/1.2.2/#scalar) - any datum with opaque structure presentable as a series of Unicode characters.

Combined, these primitives generate directed graph structures. These primitives were chosen because they are both powerful and familiar: the [sequence](https://yaml.org/spec/1.2.2/#sequence) corresponds to a Perl array and a Python list, the [mapping](https://yaml.org/spec/1.2.2/#mapping) corresponds to a Perl hash table and a Python dictionary. The [scalar](https://yaml.org/spec/1.2.2/#scalar) represents strings, integers, dates and other atomic data types.

Each YAML [node](https://yaml.org/spec/1.2.2/#nodes) requires, in addition to its [kind](https://yaml.org/spec/1.2.2/#nodes) and [content](https://yaml.org/spec/1.2.2/#nodes), a [tag](https://yaml.org/spec/1.2.2/#tags) specifying its data type. Type specifiers are either [global](https://yaml.org/spec/1.2.2/#tags) URIs or are [local](https://yaml.org/spec/1.2.2/#tags) in scope to a single [application](https://yaml.org/spec/1.2.2/#processes-and-models). For example, an integer is represented in YAML with a [scalar](https://yaml.org/spec/1.2.2/#scalar) plus the [global tag](https://yaml.org/spec/1.2.2/#tags) “ `tag:yaml.org,2002:int` ”. Similarly, an invoice object, particular to a given organization, could be represented as a [mapping](https://yaml.org/spec/1.2.2/#mapping) together with the [local tag](https://yaml.org/spec/1.2.2/#tags) “`!invoice` ”. This simple model can represent any data structure independent of programming language.

Serializing the Representation Graph

For sequential access mediums, such as an event callback API, a YAML [representation](https://yaml.org/spec/1.2.2/#representation-graph) must be *serialized* to an ordered tree. Since in a YAML [representation](https://yaml.org/spec/1.2.2/#representation-graph), [mapping keys](https://yaml.org/spec/1.2.2/#nodes) are unordered and [nodes](https://yaml.org/spec/1.2.2/#nodes) may be referenced more than once (have more than one incoming “arrow”), the serialization process is required to impose an [ordering](https://yaml.org/spec/1.2.2/#mapping-key-order) on the [mapping keys](https://yaml.org/spec/1.2.2/#nodes) and to replace the second and subsequent references to a given [node](https://yaml.org/spec/1.2.2/#nodes) with place holders called [aliases](https://yaml.org/spec/1.2.2/#anchors-and-aliases). YAML does not specify how these *serialization details* are chosen. It is up to the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) to come up with human-friendly [key order](https://yaml.org/spec/1.2.2/#mapping-key-order) and [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases) names, possibly with the help of the [application](https://yaml.org/spec/1.2.2/#processes-and-models). The result of this process, a YAML [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree), can then be traversed to produce a series of event calls for one-pass processing of YAML data.

Presenting the Serialization Tree

The final output process is *presenting* the YAML [serializations](https://yaml.org/spec/1.2.2/#serialization-tree) as a character [stream](https://yaml.org/spec/1.2.2/#streams) in a human-friendly manner. To maximize human readability, YAML offers a rich set of stylistic options which go far beyond the minimal functional needs of simple data storage. Therefore the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) is required to introduce various *presentation details* when creating the [stream](https://yaml.org/spec/1.2.2/#streams), such as the choice of [node styles](https://yaml.org/spec/1.2.2/#node-styles), how to [format scalar content](https://yaml.org/spec/1.2.2/#scalar-formats), the amount of [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces), which [tag handles](https://yaml.org/spec/1.2.2/#tag-handles) to use, the [node tags](https://yaml.org/spec/1.2.2/#node-tags) to leave [unspecified](https://yaml.org/spec/1.2.2/#resolved-tags), the set of [directives](https://yaml.org/spec/1.2.2/#directives) to provide and possibly even what [comments](https://yaml.org/spec/1.2.2/#comments) to add. While some of this can be done with the help of the [application](https://yaml.org/spec/1.2.2/#processes-and-models), in general this process should be guided by the preferences of the user.

### 3.1.2. Load

*Loading* [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) from a character [stream](https://yaml.org/spec/1.2.2/#streams) is done using the following three stages:

Parsing the Presentation Stream

*Parsing* is the inverse process of [presentation](https://yaml.org/spec/1.2.2/#presentation-stream), it takes a [stream](https://yaml.org/spec/1.2.2/#streams) of characters and produces a [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree). Parsing discards all the [details](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) introduced in the [presentation](https://yaml.org/spec/1.2.2/#presentation-stream) process, reporting only the [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree). Parsing can fail due to [ill-formed](https://yaml.org/spec/1.2.2/#well-formed-streams-and-identified-aliases) input.

Composing the Representation Graph

*Composing* takes a [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree) and produces a [representation graph](https://yaml.org/spec/1.2.2/#representation-graph). Composing discards all the [details](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) introduced in the [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) process, producing only the [representation graph](https://yaml.org/spec/1.2.2/#representation-graph). Composing can fail due to any of several reasons, detailed [below](https://yaml.org/spec/1.2.2/#loading-failure-points).

Constructing Native Data Structures

The final input process is *constructing* [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) from the YAML [representation](https://yaml.org/spec/1.2.2/#representation-graph). Construction must be based only on the information available in the [representation](https://yaml.org/spec/1.2.2/#representation-graph) and not on additional [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) or [presentation details](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) such as [comments](https://yaml.org/spec/1.2.2/#comments), [directives](https://yaml.org/spec/1.2.2/#directives), [mapping key order](https://yaml.org/spec/1.2.2/#mapping), [node styles](https://yaml.org/spec/1.2.2/#node-styles),[scalar content format](https://yaml.org/spec/1.2.2/#scalar-formats), [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) levels etc. Construction can fail due to the [unavailability](https://yaml.org/spec/1.2.2/#available-tags) of the required [native data types](https://yaml.org/spec/1.2.2/#representing-native-data-structures).

## 3.2. Information Models

This section specifies the formal details of the results of the above processes. To maximize data portability between programming languages and implementations, users of YAML should be mindful of the distinction between [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) or [presentation](https://yaml.org/spec/1.2.2/#presentation-stream) properties and those which are part of the YAML [representation](https://yaml.org/spec/1.2.2/#representation-graph). Thus, while imposing a [order](https://yaml.org/spec/1.2.2/#mapping-key-order) on [mapping keys](https://yaml.org/spec/1.2.2/#nodes) is necessary for flattening YAML [representations](https://yaml.org/spec/1.2.2/#representation-graph) to a sequential access medium, this [serialization detail](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) must not be used to convey [application](https://yaml.org/spec/1.2.2/#processes-and-models) level information. In a similar manner, while [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) technique and a choice of a [node style](https://yaml.org/spec/1.2.2/#node-styles) are needed for the human readability, these [presentation details](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) are neither part of the YAML [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) nor the YAML [representation](https://yaml.org/spec/1.2.2/#representation-graph). By carefully separating properties needed for [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) and [presentation](https://yaml.org/spec/1.2.2/#presentation-stream), YAML [representations](https://yaml.org/spec/1.2.2/#representation-graph) of [application](https://yaml.org/spec/1.2.2/#processes-and-models) information will be consistent and portable between various programming environments.

The following diagram summarizes the three *information models*. Full arrows denote composition, hollow arrows denote inheritance, “ `1` ” and “ `*` ” denote “one” and “many” relationships. A single “ `+` ” denotes [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) details, a double “ `++` ” denotes [presentation](https://yaml.org/spec/1.2.2/#presentation-stream) details.

**Figure 3.2. Information Models**

![Information Models](https://yaml.org/spec/1.2.2/img/model2.svg)

### 3.2.1. Representation Graph

YAML’s *representation* of [native data structure](https://yaml.org/spec/1.2.2/#representing-native-data-structures) is a rooted, connected, directed graph of [tagged](https://yaml.org/spec/1.2.2/#tags) [nodes](https://yaml.org/spec/1.2.2/#nodes). By “directed graph” we mean a set of [nodes](https://yaml.org/spec/1.2.2/#nodes) and directed edges (“arrows”), where each edge connects one [node](https://yaml.org/spec/1.2.2/#nodes) to another (see a formal directed graph definition [^13]). All the [nodes](https://yaml.org/spec/1.2.2/#nodes) must be reachable from the *root node* via such edges. Note that the YAML graph may include cycles and a [node](https://yaml.org/spec/1.2.2/#nodes) may have more than one incoming edge.

[Nodes](https://yaml.org/spec/1.2.2/#nodes) that are defined in terms of other [nodes](https://yaml.org/spec/1.2.2/#nodes) are [collections](https://yaml.org/spec/1.2.2/#collections); [nodes](https://yaml.org/spec/1.2.2/#nodes) that are independent of any other [nodes](https://yaml.org/spec/1.2.2/#nodes) are [scalars](https://yaml.org/spec/1.2.2/#scalars). YAML supports two [kinds](https://yaml.org/spec/1.2.2/#nodes) of [collection nodes](https://yaml.org/spec/1.2.2/#mapping): [sequences](https://yaml.org/spec/1.2.2/#sequence) and [mappings](https://yaml.org/spec/1.2.2/#mapping).[Mapping nodes](https://yaml.org/spec/1.2.2/#mapping) are somewhat tricky because their [keys](https://yaml.org/spec/1.2.2/#nodes) are unordered and must be [unique](https://yaml.org/spec/1.2.2/#node-comparison).

**Figure 3.3. Representation Model**

![Representation Model](https://yaml.org/spec/1.2.2/img/represent2.svg)

#### 3.2.1.1. Nodes

A YAML *node* [represents](https://yaml.org/spec/1.2.2/#representation-graph) a single [native data structure](https://yaml.org/spec/1.2.2/#representing-native-data-structures). Such nodes have *content* of one of three *kinds*: scalar, sequence or mapping. In addition, each node has a [tag](https://yaml.org/spec/1.2.2/#tags) which serves to restrict the set of possible values the content can have.

Scalar

The content of a *scalar* node is an opaque datum that can be [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) as a series of zero or more Unicode characters.

Sequence

The content of a *sequence* node is an ordered series of zero or more nodes. In particular, a sequence may contain the same node more than once. It could even contain itself.

Mapping

The content of a *mapping* node is an unordered set of *key/value* node *pairs*, with the restriction that each of the keys is [unique](https://yaml.org/spec/1.2.2/#node-comparison). YAML places no further restrictions on the nodes. In particular, keys may be arbitrary nodes, the same node may be used as the value of several key/value pairs and a mapping could even contain itself as a key or a value.

#### 3.2.1.2. Tags

YAML [represents](https://yaml.org/spec/1.2.2/#representation-graph) type information of [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) with a simple identifier, called a *tag*.*Global tags* are URIs and hence globally unique across all [applications](https://yaml.org/spec/1.2.2/#processes-and-models). The “ `tag:`” URI scheme [^14] is recommended for all global YAML tags. In contrast, *local tags* are specific to a single [application](https://yaml.org/spec/1.2.2/#processes-and-models). Local tags start with “`!`”, are not URIs and are not expected to be globally unique. YAML provides a “ `TAG` ” directive to make tag notation less verbose; it also offers easy migration from local to global tags. To ensure this, local tags are restricted to the URI character set and use URI character [escaping](https://yaml.org/spec/1.2.2/#escaped-characters).

YAML does not mandate any special relationship between different tags that begin with the same substring. Tags ending with URI fragments (containing “ `#` ”) are no exception; tags that share the same base URI but differ in their fragment part are considered to be different, independent tags. By convention, fragments are used to identify different “variants” of a tag, while “ `/` ” is used to define nested tag “namespace” hierarchies. However, this is merely a convention and each tag may employ its own rules. For example, Perl tags may use “`::`” to express namespace hierarchies, Java tags may use “`.`”, etc.

YAML tags are used to associate meta information with each [node](https://yaml.org/spec/1.2.2/#nodes). In particular, each tag must specify the expected [node kind](https://yaml.org/spec/1.2.2/#nodes) ([scalar](https://yaml.org/spec/1.2.2/#scalar),[sequence](https://yaml.org/spec/1.2.2/#sequence) or [mapping](https://yaml.org/spec/1.2.2/#mapping)).[Scalar](https://yaml.org/spec/1.2.2/#scalar) tags must also provide a mechanism for converting [formatted content](https://yaml.org/spec/1.2.2/#scalar-formats) to a [canonical form](https://yaml.org/spec/1.2.2/#canonical-form) for supporting [equality](https://yaml.org/spec/1.2.2/#equality) testing. Furthermore, a tag may provide additional information such as the set of allowed [content](https://yaml.org/spec/1.2.2/#nodes) values for validation, a mechanism for [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution) or any other data that is applicable to all of the tag’s [nodes](https://yaml.org/spec/1.2.2/#nodes).

#### 3.2.1.3. Node Comparison

Since YAML [mappings](https://yaml.org/spec/1.2.2/#mapping) require [key](https://yaml.org/spec/1.2.2/#nodes) uniqueness, [representations](https://yaml.org/spec/1.2.2/#representation-graph) must include a mechanism for testing the equality of [nodes](https://yaml.org/spec/1.2.2/#nodes). This is non-trivial since YAML allows various ways to [format scalar content](https://yaml.org/spec/1.2.2/#scalar-formats). For example, the integer eleven can be written as “ `0o13` ” (octal) or “ `0xB` ” (hexadecimal). If both notations are used as [keys](https://yaml.org/spec/1.2.2/#nodes) in the same [mapping](https://yaml.org/spec/1.2.2/#mapping), only a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) which recognizes integer [formats](https://yaml.org/spec/1.2.2/#scalar-formats) would correctly flag the duplicate [key](https://yaml.org/spec/1.2.2/#nodes) as an error.

Canonical Form

YAML supports the need for [scalar](https://yaml.org/spec/1.2.2/#scalar) equality by requiring that every [scalar](https://yaml.org/spec/1.2.2/#scalar) [tag](https://yaml.org/spec/1.2.2/#tags) must specify a mechanism for producing the *canonical form* of any [formatted content](https://yaml.org/spec/1.2.2/#scalar-formats). This form is a Unicode character string which also [presents](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) the same [content](https://yaml.org/spec/1.2.2/#nodes) and can be used for equality testing.

Equality

Two [nodes](https://yaml.org/spec/1.2.2/#nodes) must have the same [tag](https://yaml.org/spec/1.2.2/#tags) and [content](https://yaml.org/spec/1.2.2/#nodes) to be *equal*. Since each [tag](https://yaml.org/spec/1.2.2/#tags) applies to exactly one [kind](https://yaml.org/spec/1.2.2/#nodes), this implies that the two [nodes](https://yaml.org/spec/1.2.2/#nodes) must have the same [kind](https://yaml.org/spec/1.2.2/#nodes) to be equal.

Two [scalars](https://yaml.org/spec/1.2.2/#scalars) are equal only when their [tags](https://yaml.org/spec/1.2.2/#tags) and canonical forms are equal character-by-character. Equality of [collections](https://yaml.org/spec/1.2.2/#collections) is defined recursively.

Two [sequences](https://yaml.org/spec/1.2.2/#sequence) are equal only when they have the same [tag](https://yaml.org/spec/1.2.2/#tags) and length and each [node](https://yaml.org/spec/1.2.2/#nodes) in one [sequence](https://yaml.org/spec/1.2.2/#sequence) is equal to the corresponding [node](https://yaml.org/spec/1.2.2/#nodes) in the other [sequence](https://yaml.org/spec/1.2.2/#sequence).

Two [mappings](https://yaml.org/spec/1.2.2/#mapping) are equal only when they have the same [tag](https://yaml.org/spec/1.2.2/#tags) and an equal set of [keys](https://yaml.org/spec/1.2.2/#nodes) and each [key](https://yaml.org/spec/1.2.2/#nodes) in this set is associated with equal [values](https://yaml.org/spec/1.2.2/#nodes) in both [mappings](https://yaml.org/spec/1.2.2/#mapping).

Different URI schemes may define different rules for testing the equality of URIs. Since a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) cannot be reasonably expected to be aware of them all, it must resort to a simple character-by-character comparison of [tags](https://yaml.org/spec/1.2.2/#tags) to ensure consistency. This also happens to be the comparison method defined by the “ `tag:`” URI scheme.[Tags](https://yaml.org/spec/1.2.2/#tags) in a YAML stream must therefore be [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) in a canonical way so that such comparison would yield the correct results.

If a node has itself as a descendant (via an alias), then determining the equality of that node is implementation-defined.

A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) may treat equal [scalars](https://yaml.org/spec/1.2.2/#scalars) as if they were identical.

Uniqueness

A [mapping’s](https://yaml.org/spec/1.2.2/#mapping) [keys](https://yaml.org/spec/1.2.2/#nodes) are *unique* if no two keys are equal to each other. Obviously, identical nodes are always considered equal.

### 3.2.2. Serialization Tree

To express a YAML [representation](https://yaml.org/spec/1.2.2/#representation-graph) using a serial API, it is necessary to impose an [order](https://yaml.org/spec/1.2.2/#mapping-key-order) on [mapping keys](https://yaml.org/spec/1.2.2/#nodes) and employ [alias nodes](https://yaml.org/spec/1.2.2/#alias-nodes) to indicate a subsequent occurrence of a previously encountered [node](https://yaml.org/spec/1.2.2/#nodes). The result of this process is a *serialization tree*, where each [node](https://yaml.org/spec/1.2.2/#nodes) has an ordered set of children. This tree can be traversed for a serial event-based API.[Construction](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) of [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) from the serial interface should not use [key order](https://yaml.org/spec/1.2.2/#mapping-key-order) or [anchor names](https://yaml.org/spec/1.2.2/#anchors-and-aliases) for the preservation of [application](https://yaml.org/spec/1.2.2/#processes-and-models) data.

**Figure 3.4. Serialization Model**

![Serialization Model](https://yaml.org/spec/1.2.2/img/serialize2.svg)

#### 3.2.2.1. Mapping Key Order

In the [representation](https://yaml.org/spec/1.2.2/#representation-graph) model, [mapping keys](https://yaml.org/spec/1.2.2/#nodes) do not have an order. To [serialize](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) a [mapping](https://yaml.org/spec/1.2.2/#mapping), it is necessary to impose an *ordering* on its [keys](https://yaml.org/spec/1.2.2/#nodes). This order is a [serialization detail](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) and should not be used when [composing](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) the [representation graph](https://yaml.org/spec/1.2.2/#representation-graph) (and hence for the preservation of [application](https://yaml.org/spec/1.2.2/#processes-and-models) data). In every case where [node](https://yaml.org/spec/1.2.2/#nodes) order is significant, a [sequence](https://yaml.org/spec/1.2.2/#sequence) must be used. For example, an ordered [mapping](https://yaml.org/spec/1.2.2/#mapping) can be [represented](https://yaml.org/spec/1.2.2/#representation-graph) as a [sequence](https://yaml.org/spec/1.2.2/#sequence) of [mappings](https://yaml.org/spec/1.2.2/#mapping), where each [mapping](https://yaml.org/spec/1.2.2/#mapping) is a single [key/value pair](https://yaml.org/spec/1.2.2/#mapping). YAML provides convenient [compact notation](https://yaml.org/spec/1.2.2/#example-flow-mapping-adjacent-values) for this case.

#### 3.2.2.2. Anchors and Aliases

In the [representation graph](https://yaml.org/spec/1.2.2/#representation-graph), a [node](https://yaml.org/spec/1.2.2/#nodes) may appear in more than one [collection](https://yaml.org/spec/1.2.2/#collections). When [serializing](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) such data, the first occurrence of the [node](https://yaml.org/spec/1.2.2/#nodes) is *identified* by an *anchor*. Each subsequent occurrence is [serialized](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) as an [alias node](https://yaml.org/spec/1.2.2/#alias-nodes) which refers back to this anchor. Otherwise, anchor names are a [serialization detail](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) and are discarded once [composing](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) is completed. When [composing](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) a [representation graph](https://yaml.org/spec/1.2.2/#representation-graph) from [serialized](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) events, an alias event refers to the most recent event in the [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) having the specified anchor. Therefore, anchors need not be unique within a [serialization](https://yaml.org/spec/1.2.2/#serialization-tree). In addition, an anchor need not have an alias node referring to it.

### 3.2.3. Presentation Stream

A YAML *presentation* is a [stream](https://yaml.org/spec/1.2.2/#streams) of Unicode characters making use of [styles](https://yaml.org/spec/1.2.2/#node-styles), [scalar content formats](https://yaml.org/spec/1.2.2/#scalar-formats), [comments](https://yaml.org/spec/1.2.2/#comments), [directives](https://yaml.org/spec/1.2.2/#directives) and other [presentation details](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) to [present](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) a YAML [serialization](https://yaml.org/spec/1.2.2/#serialization-tree) in a human readable way. YAML allows several [serialization trees](https://yaml.org/spec/1.2.2/#serialization-tree) to be contained in the same YAML presentation stream, as a series of [documents](https://yaml.org/spec/1.2.2/#documents) separated by [markers](https://yaml.org/spec/1.2.2/#document-markers).

**Figure 3.5. Presentation Model**

![Presentation Model](https://yaml.org/spec/1.2.2/img/present2.svg)

#### 3.2.3.1. Node Styles

Each [node](https://yaml.org/spec/1.2.2/#nodes) is presented in some *style*, depending on its [kind](https://yaml.org/spec/1.2.2/#nodes). The node style is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and is not reflected in the [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree) or [representation graph](https://yaml.org/spec/1.2.2/#representation-graph). There are two groups of styles.[Block styles](https://yaml.org/spec/1.2.2/#block-style-productions) use [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) to denote structure. In contrast, [flow styles](https://yaml.org/spec/1.2.2/#flow-style-productions) rely on explicit [indicators](https://yaml.org/spec/1.2.2/#indicator-characters).

YAML provides a rich set of *scalar styles*.[Block scalar](https://yaml.org/spec/1.2.2/#block-scalar-styles) styles include the [literal style](https://yaml.org/spec/1.2.2/#literal-style) and the [folded style](https://yaml.org/spec/1.2.2/#folded-style).[Flow scalar](https://yaml.org/spec/1.2.2/#flow-scalar-styles) styles include the [plain style](https://yaml.org/spec/1.2.2/#plain-style) and two quoted styles, the [single-quoted style](https://yaml.org/spec/1.2.2/#single-quoted-style) and the [double-quoted style](https://yaml.org/spec/1.2.2/#double-quoted-style). These styles offer a range of trade-offs between expressive power and readability.

Normally, [block sequences](https://yaml.org/spec/1.2.2/#block-sequences) and [mappings](https://yaml.org/spec/1.2.2/#mapping) begin on the next line. In some cases, YAML also allows nested [block](https://yaml.org/spec/1.2.2/#scalars) [collections](https://yaml.org/spec/1.2.2/#collections) to start in-line for a more [compact notation](https://yaml.org/spec/1.2.2/#example-flow-mapping-adjacent-values). In addition, YAML provides a [compact notation](https://yaml.org/spec/1.2.2/#example-flow-mapping-adjacent-values) for [flow mappings](https://yaml.org/spec/1.2.2/#flow-mappings) with a single [key/value pair](https://yaml.org/spec/1.2.2/#mapping), nested inside a [flow sequence](https://yaml.org/spec/1.2.2/#flow-sequences). These allow for a natural “ordered mapping” notation.

**Figure 3.6. Kind/Style Combinations**

![Kind/Style Combinations](https://yaml.org/spec/1.2.2/img/styles2.svg)

#### 3.2.3.2. Scalar Formats

YAML allows [scalars](https://yaml.org/spec/1.2.2/#scalars) to be [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) in several *formats*. For example, the integer “ `11` ” might also be written as “ `0xB` ”.[Tags](https://yaml.org/spec/1.2.2/#tags) must specify a mechanism for converting the formatted content to a [canonical form](https://yaml.org/spec/1.2.2/#canonical-form) for use in [equality](https://yaml.org/spec/1.2.2/#equality) testing. Like [node style](https://yaml.org/spec/1.2.2/#node-styles), the format is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and is not reflected in the [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree) and [representation graph](https://yaml.org/spec/1.2.2/#representation-graph).

[Comments](https://yaml.org/spec/1.2.2/#comments) are a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not have any effect on the [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree) or [representation graph](https://yaml.org/spec/1.2.2/#representation-graph). In particular, comments are not associated with a particular [node](https://yaml.org/spec/1.2.2/#nodes). The usual purpose of a comment is to communicate between the human maintainers of a file. A typical example is comments in a configuration file. Comments must not appear inside [scalars](https://yaml.org/spec/1.2.2/#scalars), but may be interleaved with such [scalars](https://yaml.org/spec/1.2.2/#scalars) inside [collections](https://yaml.org/spec/1.2.2/#collections).

#### 3.2.3.4. Directives

Each [document](https://yaml.org/spec/1.2.2/#documents) may be associated with a set of [directives](https://yaml.org/spec/1.2.2/#directives). A directive has a name and an optional sequence of parameters. Directives are instructions to the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) and like all other [presentation details](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) are not reflected in the YAML [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree) or [representation graph](https://yaml.org/spec/1.2.2/#representation-graph). This version of YAML defines two directives, “ `YAML` ” and “ `TAG` ”. All other directives are [reserved](https://yaml.org/spec/1.2.2/#directives) for future versions of YAML.

The process of [loading](https://yaml.org/spec/1.2.2/#load) [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) from a YAML [stream](https://yaml.org/spec/1.2.2/#streams) has several potential *failure points*. The character [stream](https://yaml.org/spec/1.2.2/#streams) may be [ill-formed](https://yaml.org/spec/1.2.2/#well-formed-streams-and-identified-aliases), [aliases](https://yaml.org/spec/1.2.2/#anchors-and-aliases) may be [unidentified](https://yaml.org/spec/1.2.2/#well-formed-streams-and-identified-aliases),[unspecified tags](https://yaml.org/spec/1.2.2/#resolved-tags) may be [unresolvable](https://yaml.org/spec/1.2.2/#resolved-tags), [tags](https://yaml.org/spec/1.2.2/#tags) may be [unrecognized](https://yaml.org/spec/1.2.2/#recognized-and-valid-tags), the [content](https://yaml.org/spec/1.2.2/#nodes) may be [invalid](https://yaml.org/spec/1.2.2/#recognized-and-valid-tags), [mapping](https://yaml.org/spec/1.2.2/#mapping) [keys](https://yaml.org/spec/1.2.2/#nodes) may not be [unique](https://yaml.org/spec/1.2.2/#node-comparison) and a native type may be [unavailable](https://yaml.org/spec/1.2.2/#available-tags). Each of these failures results with an incomplete loading.

A *partial representation* need not [resolve](https://yaml.org/spec/1.2.2/#resolved-tags) the [tag](https://yaml.org/spec/1.2.2/#tags) of each [node](https://yaml.org/spec/1.2.2/#nodes) and the [canonical form](https://yaml.org/spec/1.2.2/#canonical-form) of [formatted scalar content](https://yaml.org/spec/1.2.2/#scalar-formats) need not be available. This weaker representation is useful for cases of incomplete knowledge of the types used in the [document](https://yaml.org/spec/1.2.2/#documents).

In contrast, a *complete representation* specifies the [tag](https://yaml.org/spec/1.2.2/#tags) of each [node](https://yaml.org/spec/1.2.2/#nodes) and provides the [canonical form](https://yaml.org/spec/1.2.2/#canonical-form) of [formatted scalar content](https://yaml.org/spec/1.2.2/#scalar-formats), allowing for [equality](https://yaml.org/spec/1.2.2/#equality) testing. A complete representation is required in order to [construct](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures).

**Figure 3.7. Loading Failure Points**

![Loading Failure Points](https://yaml.org/spec/1.2.2/img/validity2.svg)

### 3.3.1. Well-Formed Streams and Identified Aliases

A [well-formed](https://yaml.org/spec/1.2.2/#example-stream) character [stream](https://yaml.org/spec/1.2.2/#streams) must match the BNF productions specified in the following chapters. Successful loading also requires that each [alias](https://yaml.org/spec/1.2.2/#anchors-and-aliases) shall refer to a previous [node](https://yaml.org/spec/1.2.2/#nodes) [identified](https://yaml.org/spec/1.2.2/#anchors-and-aliases) by the [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases). A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should reject *ill-formed streams* and *unidentified aliases*. A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) may recover from syntax errors, possibly by ignoring certain parts of the input, but it must provide a mechanism for reporting such errors.

### 3.3.2. Resolved Tags

Typically, most [tags](https://yaml.org/spec/1.2.2/#tags) are not explicitly specified in the character [stream](https://yaml.org/spec/1.2.2/#streams). During [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream), [nodes](https://yaml.org/spec/1.2.2/#nodes) lacking an explicit [tag](https://yaml.org/spec/1.2.2/#tags) are given a *non-specific tag*: “`!`” for non- [plain scalars](https://yaml.org/spec/1.2.2/#plain-style) and “`?`” for all other [nodes](https://yaml.org/spec/1.2.2/#nodes).[Composing](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) a [complete representation](https://yaml.org/spec/1.2.2/#loading-failure-points) requires each such non-specific tag to be *resolved* to a *specific tag*, be it a [global tag](https://yaml.org/spec/1.2.2/#tags) or a [local tag](https://yaml.org/spec/1.2.2/#tags).

Resolving the [tag](https://yaml.org/spec/1.2.2/#tags) of a [node](https://yaml.org/spec/1.2.2/#nodes) must only depend on the following three parameters: (1) the non-specific tag of the [node](https://yaml.org/spec/1.2.2/#nodes), (2) the path leading from the [root](https://yaml.org/spec/1.2.2/#representation-graph) to the [node](https://yaml.org/spec/1.2.2/#nodes) and (3) the [content](https://yaml.org/spec/1.2.2/#nodes) (and hence the [kind](https://yaml.org/spec/1.2.2/#nodes)) of the [node](https://yaml.org/spec/1.2.2/#nodes). When a [node](https://yaml.org/spec/1.2.2/#nodes) has more than one occurrence (using [aliases](https://yaml.org/spec/1.2.2/#anchors-and-aliases)), tag resolution must depend only on the path to the first ([anchored](https://yaml.org/spec/1.2.2/#anchors-and-aliases)) occurrence of the [node](https://yaml.org/spec/1.2.2/#nodes).

Note that resolution must not consider [presentation details](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) such as [comments](https://yaml.org/spec/1.2.2/#comments), [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) and [node style](https://yaml.org/spec/1.2.2/#node-styles). Also, resolution must not consider the [content](https://yaml.org/spec/1.2.2/#nodes) of any other [node](https://yaml.org/spec/1.2.2/#nodes), except for the [content](https://yaml.org/spec/1.2.2/#nodes) of the [key nodes](https://yaml.org/spec/1.2.2/#mapping) directly along the path leading from the [root](https://yaml.org/spec/1.2.2/#representation-graph) to the resolved [node](https://yaml.org/spec/1.2.2/#nodes). Finally, resolution must not consider the [content](https://yaml.org/spec/1.2.2/#nodes) of a sibling [node](https://yaml.org/spec/1.2.2/#nodes) in a [collection](https://yaml.org/spec/1.2.2/#collections) or the [content](https://yaml.org/spec/1.2.2/#nodes) of the [value node](https://yaml.org/spec/1.2.2/#nodes) associated with a [key node](https://yaml.org/spec/1.2.2/#mapping) being resolved.

These rules ensure that tag resolution can be performed as soon as a [node](https://yaml.org/spec/1.2.2/#nodes) is first encountered in the [stream](https://yaml.org/spec/1.2.2/#streams), typically before its [content](https://yaml.org/spec/1.2.2/#nodes) is [parsed](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream). Also, tag resolution only requires referring to a relatively small number of previously parsed [nodes](https://yaml.org/spec/1.2.2/#nodes). Thus, in most cases, tag resolution in one-pass [processors](https://yaml.org/spec/1.2.2/#processes-and-models) is both possible and practical.

YAML [processors](https://yaml.org/spec/1.2.2/#processes-and-models) should resolve [nodes](https://yaml.org/spec/1.2.2/#nodes) having the “`!`” non-specific tag as “ `tag:yaml.org,2002:seq` ”, “ `tag:yaml.org,2002:map` ” or “ `tag:yaml.org,2002:str` ” depending on their [kind](https://yaml.org/spec/1.2.2/#nodes). This *tag resolution convention* allows the author of a YAML character [stream](https://yaml.org/spec/1.2.2/#streams) to effectively “disable” the tag resolution process. By explicitly specifying a “`!`” non-specific [tag property](https://yaml.org/spec/1.2.2/#node-tags), the [node](https://yaml.org/spec/1.2.2/#nodes) would then be resolved to a “vanilla” [sequence](https://yaml.org/spec/1.2.2/#sequence), [mapping](https://yaml.org/spec/1.2.2/#mapping) or string, according to its [kind](https://yaml.org/spec/1.2.2/#nodes).

[Application](https://yaml.org/spec/1.2.2/#processes-and-models) specific tag resolution rules should be restricted to resolving the “`?`” non-specific tag, most commonly to resolving [plain scalars](https://yaml.org/spec/1.2.2/#plain-style). These may be matched against a set of regular expressions to provide automatic resolution of integers, floats, timestamps and similar types. An [application](https://yaml.org/spec/1.2.2/#processes-and-models) may also match the [content](https://yaml.org/spec/1.2.2/#nodes) of [mapping nodes](https://yaml.org/spec/1.2.2/#mapping) against sets of expected [keys](https://yaml.org/spec/1.2.2/#nodes) to automatically resolve points, complex numbers and similar types. Resolved [sequence node](https://yaml.org/spec/1.2.2/#sequence) types such as the “ordered mapping” are also possible.

That said, tag resolution is specific to the [application](https://yaml.org/spec/1.2.2/#processes-and-models). YAML [processors](https://yaml.org/spec/1.2.2/#processes-and-models) should therefore provide a mechanism allowing the [application](https://yaml.org/spec/1.2.2/#processes-and-models) to override and expand these default tag resolution rules.

If a [document](https://yaml.org/spec/1.2.2/#documents) contains *unresolved tags*, the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) is unable to [compose](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) a [complete representation](https://yaml.org/spec/1.2.2/#loading-failure-points) graph. In such a case, the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) may [compose](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) a [partial representation](https://yaml.org/spec/1.2.2/#loading-failure-points), based on each [node’s kind](https://yaml.org/spec/1.2.2/#nodes) and allowing for non-specific tags.

### 3.3.3. Recognized and Valid Tags

To be *valid*, a [node](https://yaml.org/spec/1.2.2/#nodes) must have a [tag](https://yaml.org/spec/1.2.2/#tags) which is *recognized* by the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) and its [content](https://yaml.org/spec/1.2.2/#nodes) must satisfy the constraints imposed by this [tag](https://yaml.org/spec/1.2.2/#tags). If a [document](https://yaml.org/spec/1.2.2/#documents) contains a [scalar node](https://yaml.org/spec/1.2.2/#nodes) with an *unrecognized tag* or *invalid content*, only a [partial representation](https://yaml.org/spec/1.2.2/#loading-failure-points) may be [composed](https://yaml.org/spec/1.2.2/#composing-the-representation-graph). In contrast, a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) can always [compose](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) a [complete representation](https://yaml.org/spec/1.2.2/#loading-failure-points) for an unrecognized or an invalid [collection](https://yaml.org/spec/1.2.2/#collections), since [collection](https://yaml.org/spec/1.2.2/#collections) [equality](https://yaml.org/spec/1.2.2/#equality) does not depend upon knowledge of the [collection’s](https://yaml.org/spec/1.2.2/#mapping) data type. However, such a [complete representation](https://yaml.org/spec/1.2.2/#loading-failure-points) cannot be used to [construct](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) a [native data structure](https://yaml.org/spec/1.2.2/#representing-native-data-structures).

### 3.3.4. Available Tags

In a given processing environment, there need not be an *available* native type corresponding to a given [tag](https://yaml.org/spec/1.2.2/#tags). If a [node’s tag](https://yaml.org/spec/1.2.2/#tags) is *unavailable*, a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) will not be able to [construct](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) a [native data structure](https://yaml.org/spec/1.2.2/#representing-native-data-structures) for it. In this case, a [complete representation](https://yaml.org/spec/1.2.2/#loading-failure-points) may still be [composed](https://yaml.org/spec/1.2.2/#composing-the-representation-graph) and an [application](https://yaml.org/spec/1.2.2/#processes-and-models) may wish to use this [representation](https://yaml.org/spec/1.2.2/#representation-graph) directly.

## Chapter 4. Syntax Conventions

The following chapters formally define the syntax of YAML character [streams](https://yaml.org/spec/1.2.2/#streams), using parameterized BNF productions. Each BNF production is both named and numbered for easy reference. Whenever possible, basic structures are specified before the more complex structures using them in a “bottom up” fashion.

The productions are accompanied by examples which are presented in a two-pane side-by-side format. The left-hand side is the YAML example and the right-hand side is an alternate YAML view of the example. The right-hand view uses JSON when possible. Otherwise it uses a YAML form that is as close to JSON as possible.

## 4.1. Production Syntax

Productions are defined using the syntax `production-name ::= term`, where a term is either:

An atomic term

- A quoted string (`"abc"`), which matches that concatenation of characters. A single character is usually written with single quotes (`'a'`).
- A hexadecimal number (`x0A`), which matches the character at that Unicode code point.
- A range of hexadecimal numbers (`[x20-x7E]`), which matches any character whose Unicode code point is within that range.
- The name of a production (`c-printable`), which matches that production.

A lookaround

- `[ lookahead = term ]`, which matches the empty string if `term` would match.
- `[ lookahead ≠ term ]`, which matches the empty string if `term` would not match.
- `[ lookbehind = term ]`, which matches the empty string if `term` would match beginning at any prior point on the line and ending at the current position.

A special production

- `<start-of-line>`, which matches the empty string at the beginning of a line.
- `<end-of-input>`, matches the empty string at the end of the input.
- `<empty>`, which (always) matches the empty string.

A parenthesized term

Matches its contents.

A concatenation

Is `term-one term-two`, which matches `term-one` followed by `term-two`.

A alternation

Is `term-one | term-two`, which matches the `term-one` if possible, or `term-two` otherwise.

A quantified term:

- `term?`, which matches `(term | <empty>)`.
- `term*`, which matches `(term term* | <empty>)`.
- `term+`, which matches `(term term*)`.

> Note: Quantified terms are always greedy.

The order of precedence is parenthesization, then quantification, then concatenation, then alternation.

Some lines in a production definition might have a comment like:

```
production-a ::=
  production-b      # clarifying comment
```

These comments are meant to be informative only. For instance a comment that says `# not followed by non-ws char` just means that you should be aware that actual production rules will behave as described even though it might not be obvious from the content of that particular production alone.

## 4.2. Production Parameters

Some productions have parameters in parentheses after the name, such as [`s-line-prefix(n,c)`](https://yaml.org/spec/1.2.2/#rule-s-line-prefix). A parameterized production is shorthand for a (infinite) series of productions, each with a fixed value for each parameter.

For instance, this production:

```
production-a(n) ::= production-b(n)
```

Is shorthand for:

```
production-a(0) ::= production-b(0)
production-a(1) ::= production-b(1)
…
```

And this production:

```
production-a(n) ::=
  ( production-b(n+m) production-c(n+m) )+
```

Is shorthand for:

```
production-a(0) ::=
    ( production-b(0) production-c(0) )+
  | ( production-b(1) production-c(1) )+
  | …
production-a(1) ::=
    ( production-b(1) production-c(1) )+
  | ( production-b(2) production-c(2) )+
  | …
…
```

The parameters are as follows:

Indentation: `n` or `m`

May be any natural number, including zero. `n` may also be -1.

Context: `c`

This parameter allows productions to tweak their behavior according to their surrounding. YAML supports two groups of *contexts*, distinguishing between [block styles](https://yaml.org/spec/1.2.2/#block-style-productions) and [flow styles](https://yaml.org/spec/1.2.2/#flow-style-productions).

May be any of the following values:

- `BLOCK-IN` – inside block context
- `BLOCK-OUT` – outside block context
- `BLOCK-KEY` – inside block key context
- `FLOW-IN` – inside flow context
- `FLOW-OUT` – outside flow context
- `FLOW-KEY` – inside flow key context

(Block) Chomping: `t`

The [line break](https://yaml.org/spec/1.2.2/#line-break-characters) chomping behavior for flow scalars. May be any of the following values:

- `STRIP` – remove all trailing newlines
- `CLIP` – remove all trailing newlines except the first
- `KEEP` – retain all trailing newlines

## 4.3. Production Naming Conventions

To make it easier to follow production combinations, production names use a prefix-style naming convention. Each production is given a prefix based on the type of characters it begins and ends with.

`e-`

A production matching no characters.

`c-`

A production starting and ending with a special character.

`b-`

A production matching a single [line break](https://yaml.org/spec/1.2.2/#line-break-characters).

`nb-`

A production starting and ending with a non- [break](https://yaml.org/spec/1.2.2/#line-break-characters) character.

`s-`

A production starting and ending with a [white space](https://yaml.org/spec/1.2.2/#white-space-characters) character.

`ns-`

A production starting and ending with a non- [space](https://yaml.org/spec/1.2.2/#white-space-characters) character.

`l-`

A production matching complete line(s).

`X-Y-`

A production starting with an `X-` character and ending with a `Y-` character, where `X-` and `Y-` are any of the above prefixes.

`X+`, `X-Y+`

A production as above, with the additional property that the matched content [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) level is greater than the specified `n` parameter.

## Chapter 5. Character Productions

## 5.1. Character Set

To ensure readability, YAML [streams](https://yaml.org/spec/1.2.2/#streams) use only the *printable* subset of the Unicode character set. The allowed character range explicitly excludes the C0 control block [^15] `x00-x1F` (except for TAB `x09`, LF `x0A` and CR `x0D` which are allowed), DEL `x7F`, the C1 control block `x80-x9F` (except for NEL `x85` which is allowed), the surrogate block [^16] `xD800-xDFFF`, `xFFFE` and `xFFFF`.

On input, a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) must accept all characters in this printable subset.

On output, a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) must only produce only characters in this printable subset. Characters outside this set must be [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) using [escape](https://yaml.org/spec/1.2.2/#escaped-characters) sequences. In addition, any allowed characters known to be non-printable should also be [escaped](https://yaml.org/spec/1.2.2/#escaped-characters).

> Note: This isn’t mandatory since a full implementation would require extensive character property tables.

```
[1] c-printable ::=
                         # 8 bit
    x09                  # Tab (\t)
  | x0A                  # Line feed (LF \n)
  | x0D                  # Carriage Return (CR \r)
  | [x20-x7E]            # Printable ASCII
                         # 16 bit
  | x85                  # Next Line (NEL)
  | [xA0-xD7FF]          # Basic Multilingual Plane (BMP)
  | [xE000-xFFFD]        # Additional Unicode Areas
  | [x010000-x10FFFF]    # 32 bit
```

To ensure [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives), YAML [processors](https://yaml.org/spec/1.2.2/#processes-and-models) must allow all non-C0 characters inside [quoted scalars](https://yaml.org/spec/1.2.2/#double-quoted-style). To ensure readability, non-printable characters should be [escaped](https://yaml.org/spec/1.2.2/#escaped-characters) on output, even inside such [scalars](https://yaml.org/spec/1.2.2/#scalars).

> Note: JSON [quoted scalars](https://yaml.org/spec/1.2.2/#double-quoted-style) cannot span multiple lines or contain [tabs](https://yaml.org/spec/1.2.2/#white-space-characters), but YAML [quoted scalars](https://yaml.org/spec/1.2.2/#double-quoted-style) can.

```
[2] nb-json ::=
    x09              # Tab character
  | [x20-x10FFFF]    # Non-C0-control characters
```

> Note: The production name `nb-json` means “non-break JSON compatible” here.

## 5.2. Character Encodings

All characters mentioned in this specification are Unicode code points. Each such code point is written as one or more bytes depending on the *character encoding* used. Note that in UTF-16, characters above `xFFFF` are written as four bytes, using a surrogate pair.

The character encoding is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

On input, a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) must support the UTF-8 and UTF-16 character encodings. For [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives), the UTF-32 encodings must also be supported.

If a character [stream](https://yaml.org/spec/1.2.2/#streams) begins with a *byte order mark*, the character encoding will be taken to be as indicated by the byte order mark. Otherwise, the [stream](https://yaml.org/spec/1.2.2/#streams) must begin with an ASCII character. This allows the encoding to be deduced by the pattern of null (`x00`) characters.

Byte order marks may appear at the start of any [document](https://yaml.org/spec/1.2.2/#documents), however all [documents](https://yaml.org/spec/1.2.2/#documents) in the same [stream](https://yaml.org/spec/1.2.2/#streams) must use the same character encoding.

To allow for [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives), byte order marks are also allowed inside [quoted scalars](https://yaml.org/spec/1.2.2/#double-quoted-style). For readability, such [content](https://yaml.org/spec/1.2.2/#nodes) byte order marks should be [escaped](https://yaml.org/spec/1.2.2/#escaped-characters) on output.

The encoding can therefore be deduced by matching the first few bytes of the [stream](https://yaml.org/spec/1.2.2/#streams) with the following table rows (in order):

|  | Byte0 | Byte1 | Byte2 | Byte3 | Encoding |
| --- | --- | --- | --- | --- | --- |
| Explicit BOM | x00 | x00 | xFE | xFF | UTF-32BE |
| ASCII first character | x00 | x00 | x00 | any | UTF-32BE |
| Explicit BOM | xFF | xFE | x00 | x00 | UTF-32LE |
| ASCII first character | any | x00 | x00 | x00 | UTF-32LE |
| Explicit BOM | xFE | xFF |  |  | UTF-16BE |
| ASCII first character | x00 | any |  |  | UTF-16BE |
| Explicit BOM | xFF | xFE |  |  | UTF-16LE |
| ASCII first character | any | x00 |  |  | UTF-16LE |
| Explicit BOM | xEF | xBB | xBF |  | UTF-8 |
| Default |  |  |  |  | UTF-8 |

The recommended output encoding is UTF-8. If another encoding is used, it is recommended that an explicit byte order mark be used, even if the first [stream](https://yaml.org/spec/1.2.2/#streams) character is ASCII.

For more information about the byte order mark and the Unicode character encoding schemes see the Unicode FAQ [^17].

```
[3] c-byte-order-mark ::= xFEFF
```

In the examples, byte order mark characters are displayed as “ `⇔` ”.

**Example 5.2 Invalid Byte Order Mark**

| ``` - Invalid use of BOM ⇔ - Inside a document. ``` | ``` ERROR:  A BOM must not appear  inside a document. ``` |
| --- | --- |

## 5.3. Indicator Characters

*Indicators* are characters that have special semantics.

” `-` ” (`x2D`, hyphen) denotes a [block sequence](https://yaml.org/spec/1.2.2/#block-sequences) entry.

```
[4] c-sequence-entry ::= '-'
```

”`?`” (`x3F`, question mark) denotes a [mapping key](https://yaml.org/spec/1.2.2/#nodes).

```
[5] c-mapping-key ::= '?'
```

”`:`” (`x3A`, colon) denotes a [mapping value](https://yaml.org/spec/1.2.2/#mapping).

```
[6] c-mapping-value ::= ':'
```

**Example 5.3 Block Structure Indicators**

| ``` sequence: - one - two mapping:   ? sky   : blue   sea : green ``` | ```json { "sequence": [     "one",     "two" ],   "mapping": {     "sky": "blue",     "sea": "green" } } ``` |
| --- | --- |

**Legend:**

- `[c-sequence-entry](https://yaml.org/spec/1.2.2/#rule-c-sequence-entry)`
- `[c-mapping-key](https://yaml.org/spec/1.2.2/#rule-c-mapping-key)`
- `[c-mapping-value](https://yaml.org/spec/1.2.2/#rule-c-mapping-value)`

”`,`” (`x2C`, comma) ends a [flow collection](https://yaml.org/spec/1.2.2/#flow-collection-styles) entry.

```
[7] c-collect-entry ::= ','
```

” `[`” (`x5B`, left bracket) starts a [flow sequence](https://yaml.org/spec/1.2.2/#flow-sequences).

```
[8] c-sequence-start ::= '['
```

”`]` ” (`x5D`, right bracket) ends a [flow sequence](https://yaml.org/spec/1.2.2/#flow-sequences).

```
[9] c-sequence-end ::= ']'
```

” `{` ” (`x7B`, left brace) starts a [flow mapping](https://yaml.org/spec/1.2.2/#flow-mappings).

```
[10] c-mapping-start ::= '{'
```

” `}` ” (`x7D`, right brace) ends a [flow mapping](https://yaml.org/spec/1.2.2/#flow-mappings).

```
[11] c-mapping-end ::= '}'
```

**Example 5.4 Flow Collection Indicators**

| ``` sequence: [ one, two, ] mapping: { sky: blue, sea: green } ``` | ```json { "sequence": [ "one", "two" ],   "mapping":     { "sky": "blue", "sea": "green" } } ``` |
| --- | --- |

” `#` ” (`x23`, octothorpe, hash, sharp, pound, number sign) denotes a [comment](https://yaml.org/spec/1.2.2/#comments).

```
[12] c-comment ::= '#'
```

” `&` ” (`x26`, ampersand) denotes a [node’s anchor property](https://yaml.org/spec/1.2.2/#anchors-and-aliases).

```
[13] c-anchor ::= '&'
```

” `*` ” (`x2A`, asterisk) denotes an [alias node](https://yaml.org/spec/1.2.2/#alias-nodes).

```
[14] c-alias ::= '*'
```

The “`!`” (`x21`, exclamation) is used for specifying [node tags](https://yaml.org/spec/1.2.2/#node-tags). It is used to denote [tag handles](https://yaml.org/spec/1.2.2/#tag-handles) used in [tag directives](https://yaml.org/spec/1.2.2/#tag-directives) and [tag properties](https://yaml.org/spec/1.2.2/#node-tags); to denote [local tags](https://yaml.org/spec/1.2.2/#tags); and as the [non-specific tag](https://yaml.org/spec/1.2.2/#resolved-tags) for non- [plain scalars](https://yaml.org/spec/1.2.2/#plain-style).

```
[15] c-tag ::= '!'
```

**Example 5.6 Node Property Indicators**

| ``` anchored: !local &anchor value alias: *anchor ``` | ```json { "anchored": !local &A1 "value",   "alias": *A1 } ``` |
| --- | --- |

**Legend:**

- `[c-tag](https://yaml.org/spec/1.2.2/#rule-c-tag)`
- `[c-anchor](https://yaml.org/spec/1.2.2/#rule-c-anchor)`
- `[c-alias](https://yaml.org/spec/1.2.2/#rule-c-alias)`

” `|` ” (`7C`, vertical bar) denotes a [literal block scalar](https://yaml.org/spec/1.2.2/#literal-style).

```
[16] c-literal ::= '|'
```

” `>` ” (`x3E`, greater than) denotes a [folded block scalar](https://yaml.org/spec/1.2.2/#block-folding).

```
[17] c-folded ::= '>'
```

**Example 5.7 Block Scalar Indicators**

| ``` literal: \|   some   text folded: >   some   text ``` | ```json { "literal": "some\ntext\n",   "folded": "some text\n" } ``` |
| --- | --- | --- |

**Legend:**

- `[c-literal](https://yaml.org/spec/1.2.2/#rule-c-literal)`
- `[c-folded](https://yaml.org/spec/1.2.2/#rule-c-folded)`

” `'` ” (`x27`, apostrophe, single quote) surrounds a [single-quoted flow scalar](https://yaml.org/spec/1.2.2/#single-quoted-style).

```
[18] c-single-quote ::= "'"
```

” `"` ” (`x22`, double quote) surrounds a [double-quoted flow scalar](https://yaml.org/spec/1.2.2/#double-quoted-style).

```
[19] c-double-quote ::= '"'
```

**Example 5.8 Quoted Scalar Indicators**

| ``` single: 'text' double: "text" ``` | ```json { "single": "text",   "double": "text" } ``` |
| --- | --- |

**Legend:**

- `[c-single-quote](https://yaml.org/spec/1.2.2/#rule-c-single-quote)`
- `[c-double-quote](https://yaml.org/spec/1.2.2/#rule-c-double-quote)`

” `%` ” (`x25`, percent) denotes a [directive](https://yaml.org/spec/1.2.2/#directives) line.

```
[20] c-directive ::= '%'
```

**Example 5.9 Directive Indicator**

| ``` %YAML 1.2 --- text ``` | ```json "text" ``` |
| --- | --- |

**Legend:**

- `[c-directive](https://yaml.org/spec/1.2.2/#rule-c-directive)`

The “ `@` ” (`x40`, at) and “ `` ` `` ” (`x60`, grave accent) are *reserved* for future use.

```
[21] c-reserved ::=
    '@' | '\`'
```

**Example 5.10 Invalid use of Reserved Indicators**

| ``` commercial-at: @text grave-accent: \`text ``` | ``` ERROR:  Reserved indicators can't  start a plain scalar. ``` |
| --- | --- |

Any indicator character:

```
[22] c-indicator ::=
    c-sequence-entry    # '-'
  | c-mapping-key       # '?'
  | c-mapping-value     # ':'
  | c-collect-entry     # ','
  | c-sequence-start    # '['
  | c-sequence-end      # ']'
  | c-mapping-start     # '{'
  | c-mapping-end       # '}'
  | c-comment           # '#'
  | c-anchor            # '&'
  | c-alias             # '*'
  | c-tag               # '!'
  | c-literal           # '|'
  | c-folded            # '>'
  | c-single-quote      # "'"
  | c-double-quote      # '"'
  | c-directive         # '%'
  | c-reserved          # '@' '\`'
```

The “ `[`”, “`]` ”, “ `{` ”, “ `}` ” and “`,`” indicators denote structure in [flow collections](https://yaml.org/spec/1.2.2/#flow-collection-styles). They are therefore forbidden in some cases, to avoid ambiguity in several constructs. This is handled on a case-by-case basis by the relevant productions.

```
[23] c-flow-indicator ::=
    c-collect-entry     # ','
  | c-sequence-start    # '['
  | c-sequence-end      # ']'
  | c-mapping-start     # '{'
  | c-mapping-end       # '}'
```

## 5.4. Line Break Characters

YAML recognizes the following ASCII *line break* characters.

```
[24] b-line-feed ::= x0A
```
```
[25] b-carriage-return ::= x0D
```
```
[26] b-char ::=
    b-line-feed          # x0A
  | b-carriage-return    # X0D
```

All other characters, including the form feed (`x0C`), are considered to be non-break characters. Note that these include the *non-ASCII line breaks*: next line (`x85`), line separator (`x2028`) and paragraph separator (`x2029`).

[YAML version 1.1](https://yaml.org/spec/1.2.2/#yaml-directives) did support the above non-ASCII line break characters; however, JSON does not. Hence, to ensure [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives), YAML treats them as non-break characters as of version 1.2. YAML 1.2 [processors](https://yaml.org/spec/1.2.2/#processes-and-models) [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) a [version 1.1](https://yaml.org/spec/1.2.2/#yaml-directives) [document](https://yaml.org/spec/1.2.2/#documents) should therefore treat these line breaks as non-break characters, with an appropriate warning.

```
[27] nb-char ::=
  c-printable - b-char - c-byte-order-mark
```

Line breaks are interpreted differently by different systems and have multiple widely used formats.

```
[28] b-break ::=
    (
      b-carriage-return  # x0A
      b-line-feed
    )                    # x0D
  | b-carriage-return
  | b-line-feed
```

Line breaks inside [scalar content](https://yaml.org/spec/1.2.2/#scalar) must be *normalized* by the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models). Each such line break must be [parsed](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) into a single line feed character. The original line break format is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

```
[29] b-as-line-feed ::=
  b-break
```

Outside [scalar content](https://yaml.org/spec/1.2.2/#scalar), YAML allows any line break to be used to terminate lines.

```
[30] b-non-content ::=
  b-break
```

On output, a YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) is free to emit line breaks using whatever convention is most appropriate.

In the examples, line breaks are sometimes displayed using the “ `↓` ” glyph for clarity.

**Example 5.11 Line Break Characters**

| ``` \|   Line break (no glyph)   Line break (glyphed)↓ ``` | ```json "Line break (no glyph)\nLine break (glyphed)\n" ``` |
| --- | --- | --- |

**Legend:**

- `[b-break](https://yaml.org/spec/1.2.2/#rule-b-break)`

## 5.5. White Space Characters

YAML recognizes two *white space* characters: *space* and *tab*.

```
[31] s-space ::= x20
```
```
[32] s-tab ::= x09
```
```
[33] s-white ::=
  s-space | s-tab
```

The rest of the ([printable](https://yaml.org/spec/1.2.2/#character-set)) non- [break](https://yaml.org/spec/1.2.2/#line-break-characters) characters are considered to be non-space characters.

```
[34] ns-char ::=
  nb-char - s-white
```

In the examples, tab characters are displayed as the glyph “ `→` ”. Space characters are sometimes displayed as the glyph “ `·` ” for clarity.

## 5.6. Miscellaneous Characters

The YAML syntax productions make use of the following additional character classes:

A decimal digit for numbers:

```
[35] ns-dec-digit ::=
  [x30-x39]             # 0-9
```

A hexadecimal digit for [escape sequences](https://yaml.org/spec/1.2.2/#escaped-characters):

```
[36] ns-hex-digit ::=
    ns-dec-digit        # 0-9
  | [x41-x46]           # A-F
  | [x61-x66]           # a-f
```

ASCII letter (alphabetic) characters:

```
[37] ns-ascii-letter ::=
    [x41-x5A]           # A-Z
  | [x61-x7A]           # a-z
```

Word (alphanumeric) characters for identifiers:

```
[38] ns-word-char ::=
    ns-dec-digit        # 0-9
  | ns-ascii-letter     # A-Z a-z
  | '-'                 # '-'
```

URI characters for [tags](https://yaml.org/spec/1.2.2/#tags), as defined in the URI specification [^18].

By convention, any URI characters other than the allowed printable ASCII characters are first *encoded* in UTF-8 and then each byte is *escaped* using the “ `%` ” character. The YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) must not expand such escaped characters.[Tag](https://yaml.org/spec/1.2.2/#tags) characters must be preserved and compared exactly as [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) in the YAML [stream](https://yaml.org/spec/1.2.2/#streams), without any processing.

```
[39] ns-uri-char ::=
    (
      '%'
      ns-hex-digit{2}
    )
  | ns-word-char
  | '#'
  | ';'
  | '/'
  | '?'
  | ':'
  | '@'
  | '&'
  | '='
  | '+'
  | '$'
  | ','
  | '_'
  | '.'
  | '!'
  | '~'
  | '*'
  | "'"
  | '('
  | ')'
  | '['
  | ']'
```

The “`!`” character is used to indicate the end of a [named tag handle](https://yaml.org/spec/1.2.2/#tag-handles); hence its use in [tag shorthands](https://yaml.org/spec/1.2.2/#tag-shorthands) is restricted. In addition, such [shorthands](https://yaml.org/spec/1.2.2/#tag-shorthands) must not contain the “ `[`”, “`]` ”, “ `{` ”, “ `}` ” and “`,`” characters. These characters would cause ambiguity with [flow collection](https://yaml.org/spec/1.2.2/#flow-collection-styles) structures.

```
[40] ns-tag-char ::=
    ns-uri-char
  - c-tag               # '!'
  - c-flow-indicator
```

## 5.7. Escaped Characters

All non- [printable](https://yaml.org/spec/1.2.2/#character-set) characters must be *escaped*. YAML escape sequences use the “ `\` ” notation common to most modern computer languages. Each escape sequence must be [parsed](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) into the appropriate Unicode character. The original escape sequence is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

Note that escape sequences are only interpreted in [double-quoted scalars](https://yaml.org/spec/1.2.2/#double-quoted-style). In all other [scalar styles](https://yaml.org/spec/1.2.2/#node-styles), the “ `\` ” character has no special meaning and non- [printable](https://yaml.org/spec/1.2.2/#character-set) characters are not available.

```
[41] c-escape ::= '\'
```

YAML escape sequences are a superset of C’s escape sequences:

Escaped ASCII null (`x00`) character.

```
[42] ns-esc-null ::= '0'
```

Escaped ASCII bell (`x07`) character.

```
[43] ns-esc-bell ::= 'a'
```

Escaped ASCII backspace (`x08`) character.

```
[44] ns-esc-backspace ::= 'b'
```

Escaped ASCII horizontal tab (`x09`) character. This is useful at the start or the end of a line to force a leading or trailing tab to become part of the [content](https://yaml.org/spec/1.2.2/#nodes).

```
[45] ns-esc-horizontal-tab ::=
  't' | x09
```

Escaped ASCII line feed (`x0A`) character.

```
[46] ns-esc-line-feed ::= 'n'
```

Escaped ASCII vertical tab (`x0B`) character.

```
[47] ns-esc-vertical-tab ::= 'v'
```

Escaped ASCII form feed (`x0C`) character.

```
[48] ns-esc-form-feed ::= 'f'
```

Escaped ASCII carriage return (`x0D`) character.

```
[49] ns-esc-carriage-return ::= 'r'
```

Escaped ASCII escape (`x1B`) character.

```
[50] ns-esc-escape ::= 'e'
```

Escaped ASCII space (`x20`) character. This is useful at the start or the end of a line to force a leading or trailing space to become part of the [content](https://yaml.org/spec/1.2.2/#nodes).

```
[51] ns-esc-space ::= x20
```

Escaped ASCII double quote (`x22`).

```
[52] ns-esc-double-quote ::= '"'
```

Escaped ASCII slash (`x2F`), for [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives).

```
[53] ns-esc-slash ::= '/'
```

Escaped ASCII back slash (`x5C`).

```
[54] ns-esc-backslash ::= '\'
```

Escaped Unicode next line (`x85`) character.

```
[55] ns-esc-next-line ::= 'N'
```

Escaped Unicode non-breaking space (`xA0`) character.

```
[56] ns-esc-non-breaking-space ::= '_'
```

Escaped Unicode line separator (`x2028`) character.

```
[57] ns-esc-line-separator ::= 'L'
```

Escaped Unicode paragraph separator (`x2029`) character.

```
[58] ns-esc-paragraph-separator ::= 'P'
```

Escaped 8-bit Unicode character.

```
[59] ns-esc-8-bit ::=
  'x'
  ns-hex-digit{2}
```

Escaped 16-bit Unicode character.

```
[60] ns-esc-16-bit ::=
  'u'
  ns-hex-digit{4}
```

Escaped 32-bit Unicode character.

```
[61] ns-esc-32-bit ::=
  'U'
  ns-hex-digit{8}
```

Any escaped character:

```
[62] c-ns-esc-char ::=
  c-escape         # '\'
  (
      ns-esc-null
    | ns-esc-bell
    | ns-esc-backspace
    | ns-esc-horizontal-tab
    | ns-esc-line-feed
    | ns-esc-vertical-tab
    | ns-esc-form-feed
    | ns-esc-carriage-return
    | ns-esc-escape
    | ns-esc-space
    | ns-esc-double-quote
    | ns-esc-slash
    | ns-esc-backslash
    | ns-esc-next-line
    | ns-esc-non-breaking-space
    | ns-esc-line-separator
    | ns-esc-paragraph-separator
    | ns-esc-8-bit
    | ns-esc-16-bit
    | ns-esc-32-bit
  )
```

**Example 5.13 Escaped Characters**

| ``` - "Fun with \\" - "\" \a \b \e \f" - "\n \r \t \v \0" - "\  \_ \N \L \P \   \x41 \u0041 \U00000041" ``` | ```json [ "Fun with \\",   "\" \u0007 \b \u001b \f",   "\n \r \t \u000b \u0000",   "\u0020 \u00a0 \u0085 \u2028 \u2029 A A A" ] ``` |
| --- | --- |

**Legend:**

- `[c-ns-esc-char](https://yaml.org/spec/1.2.2/#rule-c-ns-esc-char)`

**Example 5.14 Invalid Escaped Characters**

| ``` Bad escapes:   "\c   \xq-" ``` | ``` ERROR: - c is an invalid escaped character. - q and - are invalid hex digits. ``` |
| --- | --- |

## Chapter 6. Structural Productions

## 6.1. Indentation Spaces

In YAML [block styles](https://yaml.org/spec/1.2.2/#block-style-productions), structure is determined by *indentation*. In general, indentation is defined as a zero or more [space](https://yaml.org/spec/1.2.2/#white-space-characters) characters at the start of a line.

To maintain portability, [tab](https://yaml.org/spec/1.2.2/#white-space-characters) characters must not be used in indentation, since different systems treat [tabs](https://yaml.org/spec/1.2.2/#white-space-characters) differently. Note that most modern editors may be configured so that pressing the [tab](https://yaml.org/spec/1.2.2/#white-space-characters) key results in the insertion of an appropriate number of [spaces](https://yaml.org/spec/1.2.2/#white-space-characters).

The amount of indentation is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

```
[63]
s-indent(0) ::=
  <empty>

# When n≥0
s-indent(n+1) ::=
  s-space s-indent(n)
```

A [block style](https://yaml.org/spec/1.2.2/#block-style-productions) construct is terminated when encountering a line which is less indented than the construct. The productions use the notation “ `s-indent-less-than(n)` ” and “ `s-indent-less-or-equal(n)` ” to express this.

```
[64]
s-indent-less-than(1) ::=
  <empty>

# When n≥1
s-indent-less-than(n+1) ::=
  s-space s-indent-less-than(n)
  | <empty>
```
```
[65]
s-indent-less-or-equal(0) ::=
  <empty>

# When n≥0
s-indent-less-or-equal(n+1) ::=
  s-space s-indent-less-or-equal(n)
  | <empty>
```

Each [node](https://yaml.org/spec/1.2.2/#nodes) must be indented further than its parent [node](https://yaml.org/spec/1.2.2/#nodes). All sibling [nodes](https://yaml.org/spec/1.2.2/#nodes) must use the exact same indentation level. However the [content](https://yaml.org/spec/1.2.2/#nodes) of each sibling [node](https://yaml.org/spec/1.2.2/#nodes) may be further indented independently.

**Example 6.1 Indentation Spaces**

| ``` ··# Leading comment line spaces are ···# neither content nor indentation. ···· Not indented: ·By one space: \| ····By four ······spaces ·Flow style: [    # Leading spaces ···By two,        # in flow style ··Also by two,    # are neither ··→Still by two   # content nor ····]             # indentation. ``` | ```json { "Not indented": {     "By one space": "By four\n  spaces\n",     "Flow style": [       "By two",       "Also by two",       "Still by two" ] } } ``` |
| --- | --- | --- |

**Legend:**

- `[s-indent(n)](https://yaml.org/spec/1.2.2/#rule-s-indent)`
- `Content`
- `Neither content nor indentation`

The “ `-` ”, “`?`” and “`:`” characters used to denote [block collection](https://yaml.org/spec/1.2.2/#block-collection-styles) entries are perceived by people to be part of the indentation. This is handled on a case-by-case basis by the relevant productions.

**Example 6.2 Indentation Indicators**

| ``` ?·a :·-→b ··-··-→c ·····-·d ``` | ```json { "a":   [ "b",     [ "c",       "d" ] ] } ``` |
| --- | --- |

**Legend:**

- `Total Indentation`
- `[s-indent(n)](https://yaml.org/spec/1.2.2/#rule-s-indent)`
- `Indicator as indentation`

## 6.2. Separation Spaces

Outside [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) and [scalar content](https://yaml.org/spec/1.2.2/#scalar), YAML uses [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters for *separation* between tokens within a line. Note that such [white space](https://yaml.org/spec/1.2.2/#white-space-characters) may safely include [tab](https://yaml.org/spec/1.2.2/#white-space-characters) characters.

Separation spaces are a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

```
[66] s-separate-in-line ::=
    s-white+
  | <start-of-line>
```

**Example 6.3 Separation Spaces**

| ``` -·foo:→·bar - -·baz   -→baz ``` | ```json [ { "foo": "bar" },   [ "baz",     "baz" ] ] ``` |
| --- | --- |

**Legend:**

- `[s-separate-in-line](https://yaml.org/spec/1.2.2/#rule-s-separate-in-line)`

## 6.3. Line Prefixes

Inside [scalar content](https://yaml.org/spec/1.2.2/#scalar), each line begins with a non- [content](https://yaml.org/spec/1.2.2/#nodes) *line prefix*. This prefix always includes the [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces). For [flow scalar styles](https://yaml.org/spec/1.2.2/#flow-scalar-styles) it additionally includes all leading [white space](https://yaml.org/spec/1.2.2/#white-space-characters), which may contain [tab](https://yaml.org/spec/1.2.2/#white-space-characters) characters.

Line prefixes are a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

```
[67]
s-line-prefix(n,BLOCK-OUT) ::= s-block-line-prefix(n)
s-line-prefix(n,BLOCK-IN)  ::= s-block-line-prefix(n)
s-line-prefix(n,FLOW-OUT)  ::= s-flow-line-prefix(n)
s-line-prefix(n,FLOW-IN)   ::= s-flow-line-prefix(n)
```
```
[68] s-block-line-prefix(n) ::=
  s-indent(n)
```
```
[69] s-flow-line-prefix(n) ::=
  s-indent(n)
  s-separate-in-line?
```

**Example 6.4 Line Prefixes**

| ``` plain: text ··lines quoted: "text ··→lines" block: \| ··text ···→lines ``` | ```json { "plain": "text lines",   "quoted": "text lines",   "block": "text\n \tlines\n" } ``` |
| --- | --- | --- |

**Legend:**

- `[s-flow-line-prefix(n)](https://yaml.org/spec/1.2.2/#rule-s-flow-line-prefix)`
- `[s-block-line-prefix(n)](https://yaml.org/spec/1.2.2/#rule-s-block-line-prefix)`
- `[s-indent(n)](https://yaml.org/spec/1.2.2/#rule-s-indent)`

## 6.4. Empty Lines

An *empty line* line consists of the non- [content](https://yaml.org/spec/1.2.2/#nodes) [prefix](https://yaml.org/spec/1.2.2/#tag-prefixes) followed by a [line break](https://yaml.org/spec/1.2.2/#line-break-characters).

```
[70] l-empty(n,c) ::=
  (
      s-line-prefix(n,c)
    | s-indent-less-than(n)
  )
  b-as-line-feed
```

The semantics of empty lines depend on the [scalar style](https://yaml.org/spec/1.2.2/#node-styles) they appear in. This is handled on a case-by-case basis by the relevant productions.

**Example 6.5 Empty Lines**

| ``` Folding:   "Empty line ···→   as a line feed" Chomping: \|   Clipped empty lines · ``` | ```json { "Folding": "Empty line\nas a line feed",   "Chomping": "Clipped empty lines\n" } ``` |
| --- | --- | --- |

**Legend:**

- `[l-empty(n,c)](https://yaml.org/spec/1.2.2/#rule-l-empty)`

## 6.5. Line Folding

*Line folding* allows long lines to be broken for readability, while retaining the semantics of the original long line. If a [line break](https://yaml.org/spec/1.2.2/#line-break-characters) is followed by an [empty line](https://yaml.org/spec/1.2.2/#empty-lines), it is *trimmed*; the first [line break](https://yaml.org/spec/1.2.2/#line-break-characters) is discarded and the rest are retained as [content](https://yaml.org/spec/1.2.2/#nodes).

```
[71] b-l-trimmed(n,c) ::=
  b-non-content
  l-empty(n,c)+
```

Otherwise (the following line is not [empty](https://yaml.org/spec/1.2.2/#empty-lines)), the [line break](https://yaml.org/spec/1.2.2/#line-break-characters) is converted to a single [space](https://yaml.org/spec/1.2.2/#white-space-characters) (`x20`).

```
[72] b-as-space ::=
  b-break
```

A folded non- [empty line](https://yaml.org/spec/1.2.2/#empty-lines) may end with either of the above [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters).

```
[73] b-l-folded(n,c) ::=
  b-l-trimmed(n,c) | b-as-space
```

**Example 6.6 Line Folding**

| ``` >-   trimmed↓ ··↓ ·↓ ↓   as↓   space ``` | ```json "trimmed\n\n\nas space" ``` |
| --- | --- |

**Legend:**

- `[b-l-trimmed(n,c)](https://yaml.org/spec/1.2.2/#rule-b-l-trimmed)`
- `[b-as-space](https://yaml.org/spec/1.2.2/#rule-b-as-space)`

The above rules are common to both the [folded block style](https://yaml.org/spec/1.2.2/#block-folding) and the [scalar flow styles](https://yaml.org/spec/1.2.2/#flow-scalar-styles). Folding does distinguish between these cases in the following way:

Block Folding

In the [folded block style](https://yaml.org/spec/1.2.2/#block-folding), the final [line break](https://yaml.org/spec/1.2.2/#line-break-characters) and trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are subject to [chomping](https://yaml.org/spec/1.2.2/#block-chomping-indicator) and are never folded. In addition, folding does not apply to [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) surrounding text lines that contain leading [white space](https://yaml.org/spec/1.2.2/#white-space-characters). Note that such a [more-indented](https://yaml.org/spec/1.2.2/#example-more-indented-lines) line may consist only of such leading [white space](https://yaml.org/spec/1.2.2/#white-space-characters).

The combined effect of the *block line folding* rules is that each “paragraph” is interpreted as a line, [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are interpreted as a line feed and the formatting of [more-indented](https://yaml.org/spec/1.2.2/#example-more-indented-lines) lines is preserved.

**Example 6.7 Block Folding**

| ``` > ··foo·↓ ·↓ ··→·bar↓ ↓ ··baz↓ ``` | ```json "foo \n\n\t bar\n\nbaz\n" ``` |
| --- | --- |

**Legend:**

- `[b-l-folded(n,c)](https://yaml.org/spec/1.2.2/#rule-b-l-folded)`
- `Non-content spaces`
- `Content spaces`

Flow Folding

Folding in [flow styles](https://yaml.org/spec/1.2.2/#flow-style-productions) provides more relaxed semantics.[Flow styles](https://yaml.org/spec/1.2.2/#flow-style-productions) typically depend on explicit [indicators](https://yaml.org/spec/1.2.2/#indicator-characters) rather than [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) to convey structure. Hence spaces preceding or following the text in a line are a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information. Once all such spaces have been discarded, all [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) are folded without exception.

The combined effect of the *flow line folding* rules is that each “paragraph” is interpreted as a line, [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are interpreted as line feeds and text can be freely [more-indented](https://yaml.org/spec/1.2.2/#example-more-indented-lines) without affecting the [content](https://yaml.org/spec/1.2.2/#nodes) information.

```
[74] s-flow-folded(n) ::=
  s-separate-in-line?
  b-l-folded(n,FLOW-IN)
  s-flow-line-prefix(n)
```

**Example 6.8 Flow Folding**

| ``` "↓ ··foo·↓ ·↓ ··→·bar↓ ↓ ··baz↓ " ``` | ```json " foo\nbar\nbaz " ``` |
| --- | --- |

**Legend:**

- `[s-flow-folded(n)](https://yaml.org/spec/1.2.2/#rule-s-flow-folded)`
- `Non-content spaces`

An explicit *comment* is marked by a “ `#` ” indicator. Comments are a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

Comments must be [separated](https://yaml.org/spec/1.2.2/#separation-spaces) from other tokens by [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters.

> Note: To ensure [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives), YAML [processors](https://yaml.org/spec/1.2.2/#processes-and-models) must allow for the omission of the final comment [line break](https://yaml.org/spec/1.2.2/#line-break-characters) of the input [stream](https://yaml.org/spec/1.2.2/#streams). However, as this confuses many tools, YAML [processors](https://yaml.org/spec/1.2.2/#processes-and-models) should terminate the [stream](https://yaml.org/spec/1.2.2/#streams) with an explicit [line break](https://yaml.org/spec/1.2.2/#line-break-characters) on output.

```
[75] c-nb-comment-text ::=
  c-comment    # '#'
  nb-char*
```
```
[76] b-comment ::=
    b-non-content
  | <end-of-input>
```
```
[77] s-b-comment ::=
  (
    s-separate-in-line
    c-nb-comment-text?
  )?
  b-comment
```

**Example 6.9 Separated Comment**

| ``` key:····# Comment↓   valueeof ``` | ```json { "key": "value" } ``` |
| --- | --- |

**Legend:**

- `[c-nb-comment-text](https://yaml.org/spec/1.2.2/#rule-c-nb-comment-text)`
- `[b-comment](https://yaml.org/spec/1.2.2/#rule-b-comment)`
- `[s-b-comment](https://yaml.org/spec/1.2.2/#rule-s-b-comment)`

Outside [scalar content](https://yaml.org/spec/1.2.2/#scalar), comments may appear on a line of their own, independent of the [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) level. Note that outside [scalar content](https://yaml.org/spec/1.2.2/#scalar), a line containing only [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters is taken to be a comment line.

```
[78] l-comment ::=
  s-separate-in-line
  c-nb-comment-text?
  b-comment
```

In most cases, when a line may end with a comment, YAML allows it to be followed by additional comment lines. The only exception is a comment ending a [block scalar header](https://yaml.org/spec/1.2.2/#block-scalar-headers).

```
[79] s-l-comments ::=
  (
      s-b-comment
    | <start-of-line>
  )
  l-comment*
```

## 6.7. Separation Lines

[Implicit keys](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry) are restricted to a single line. In all other cases, YAML allows tokens to be separated by multi-line (possibly empty) [comments](https://yaml.org/spec/1.2.2/#comments).

Note that structures following multi-line comment separation must be properly [indented](https://yaml.org/spec/1.2.2/#indentation-spaces), even though there is no such restriction on the separation [comment](https://yaml.org/spec/1.2.2/#comments) lines themselves.

```
[80]
s-separate(n,BLOCK-OUT) ::= s-separate-lines(n)
s-separate(n,BLOCK-IN)  ::= s-separate-lines(n)
s-separate(n,FLOW-OUT)  ::= s-separate-lines(n)
s-separate(n,FLOW-IN)   ::= s-separate-lines(n)
s-separate(n,BLOCK-KEY) ::= s-separate-in-line
s-separate(n,FLOW-KEY)  ::= s-separate-in-line
```
```
[81] s-separate-lines(n) ::=
    (
      s-l-comments
      s-flow-line-prefix(n)
    )
  | s-separate-in-line
```

**Example 6.12 Separation Spaces**

| ``` {·first:·Sammy,·last:·Sosa·}:↓ # Statistics: ··hr:··# Home runs ·····65 ··avg:·# Average ···0.278 ``` | ```json { { "first": "Sammy",     "last": "Sosa" }: {     "hr": 65,     "avg": 0.278 } } ``` |
| --- | --- |

**Legend:**

- `[s-separate-in-line](https://yaml.org/spec/1.2.2/#rule-s-separate-in-line)`
- `[s-separate-lines(n)](https://yaml.org/spec/1.2.2/#rule-s-separate-lines)`
- `[s-indent(n)](https://yaml.org/spec/1.2.2/#rule-s-indent)`

## 6.8. Directives

*Directives* are instructions to the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models). This specification defines two directives, “ `YAML` ” and “ `TAG` ”, and *reserves* all other directives for future use. There is no way to define private directives. This is intentional.

Directives are a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

```
[82] l-directive ::=
  c-directive            # '%'
  (
      ns-yaml-directive
    | ns-tag-directive
    | ns-reserved-directive
  )
  s-l-comments
```

Each directive is specified on a separate non- [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) line starting with the “ `%` ” indicator, followed by the directive name and a list of parameters. The semantics of these parameters depends on the specific directive. A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should ignore unknown directives with an appropriate warning.

```
[83] ns-reserved-directive ::=
  ns-directive-name
  (
    s-separate-in-line
    ns-directive-parameter
  )*
```
```
[84] ns-directive-name ::=
  ns-char+
```
```
[85] ns-directive-parameter ::=
  ns-char+
```

**Example 6.13 Reserved Directives**

| ``` %FOO  bar baz # Should be ignored                # with a warning. --- "foo" ``` | ```json "foo" ``` |
| --- | --- |

**Legend:**

- `[ns-reserved-directive](https://yaml.org/spec/1.2.2/#rule-ns-reserved-directive)`
- `[ns-directive-name](https://yaml.org/spec/1.2.2/#rule-ns-directive-name)`
- `[ns-directive-parameter](https://yaml.org/spec/1.2.2/#rule-ns-directive-parameter)`

### 6.8.1. “YAML” Directives

The “ `YAML` ” directive specifies the version of YAML the [document](https://yaml.org/spec/1.2.2/#documents) conforms to. This specification defines version “ `1.2` ”, including recommendations for *YAML 1.1 processing*.

A version 1.2 YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) must accept [documents](https://yaml.org/spec/1.2.2/#documents) with an explicit “ `%YAML 1.2` ” directive, as well as [documents](https://yaml.org/spec/1.2.2/#documents) lacking a “ `YAML` ” directive. Such [documents](https://yaml.org/spec/1.2.2/#documents) are assumed to conform to the 1.2 version specification.[Documents](https://yaml.org/spec/1.2.2/#documents) with a “ `YAML` ” directive specifying a higher minor version (e.g. “ `%YAML 1.3` ”) should be processed with an appropriate warning.[Documents](https://yaml.org/spec/1.2.2/#documents) with a “ `YAML` ” directive specifying a higher major version (e.g. “ `%YAML 2.0` ”) should be rejected with an appropriate error message.

A version 1.2 YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) must also accept [documents](https://yaml.org/spec/1.2.2/#documents) with an explicit “ `%YAML 1.1` ” directive. Note that version 1.2 is mostly a superset of version 1.1, defined for the purpose of ensuring *JSON compatibility*. Hence a version 1.2 [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should process version 1.1 [documents](https://yaml.org/spec/1.2.2/#documents) as if they were version 1.2, giving a warning on points of incompatibility (handling of [non-ASCII line breaks](https://yaml.org/spec/1.2.2/#line-break-characters), as described [above](https://yaml.org/spec/1.2.2/#line-break-characters)).

```
[86] ns-yaml-directive ::=
  "YAML"
  s-separate-in-line
  ns-yaml-version
```
```
[87] ns-yaml-version ::=
  ns-dec-digit+
  '.'
  ns-dec-digit+
```

**Example 6.14 “ `YAML` ” directive**

| ``` %YAML 1.3 # Attempt parsing            # with a warning --- "foo" ``` | ```json "foo" ``` |
| --- | --- |

**Legend:**

- `[ns-yaml-directive](https://yaml.org/spec/1.2.2/#rule-ns-yaml-directive)`
- `[ns-yaml-version](https://yaml.org/spec/1.2.2/#rule-ns-yaml-version)`

It is an error to specify more than one “ `YAML` ” directive for the same document, even if both occurrences give the same version number.

**Example 6.15 Invalid Repeated YAML directive**

| ``` %YAML 1.2 %YAML 1.1 foo ``` | ``` ERROR: The YAML directive must only be given at most once per document. ``` |
| --- | --- |

### 6.8.2. “TAG” Directives

The “ `TAG` ” directive establishes a [tag shorthand](https://yaml.org/spec/1.2.2/#tag-shorthands) notation for specifying [node tags](https://yaml.org/spec/1.2.2/#node-tags). Each “ `TAG` ” directive associates a [handle](https://yaml.org/spec/1.2.2/#tag-handles) with a [prefix](https://yaml.org/spec/1.2.2/#tag-prefixes). This allows for compact and readable [tag](https://yaml.org/spec/1.2.2/#tags) notation.

```
[88] ns-tag-directive ::=
  "TAG"
  s-separate-in-line
  c-tag-handle
  s-separate-in-line
  ns-tag-prefix
```

**Example 6.16 “ `TAG` ” directive**

| ``` %TAG !yaml! tag:yaml.org,2002: --- !yaml!str "foo" ``` | ```json "foo" ``` |
| --- | --- |

**Legend:**

- `[ns-tag-directive](https://yaml.org/spec/1.2.2/#rule-ns-tag-directive)`
- `[c-tag-handle](https://yaml.org/spec/1.2.2/#rule-c-tag-handle)`
- `[ns-tag-prefix](https://yaml.org/spec/1.2.2/#rule-ns-tag-prefix)`

It is an error to specify more than one “ `TAG` ” directive for the same [handle](https://yaml.org/spec/1.2.2/#tag-handles) in the same document, even if both occurrences give the same [prefix](https://yaml.org/spec/1.2.2/#tag-prefixes).

**Example 6.17 Invalid Repeated TAG directive**

| ``` %TAG ! !foo %TAG ! !foo bar ``` | ``` ERROR: The TAG directive must only be given at most once per handle in the same document. ``` |
| --- | --- |

#### 6.8.2.1. Tag Handles

The *tag handle* exactly matches the prefix of the affected [tag shorthand](https://yaml.org/spec/1.2.2/#tag-shorthands). There are three tag handle variants:

```
[89] c-tag-handle ::=
    c-named-tag-handle
  | c-secondary-tag-handle
  | c-primary-tag-handle
```

Primary Handle

The *primary tag handle* is a single “`!`” character. This allows using the most compact possible notation for a single “primary” name space. By default, the prefix associated with this handle is “`!`”. Thus, by default, [shorthands](https://yaml.org/spec/1.2.2/#tag-shorthands) using this handle are interpreted as [local tags](https://yaml.org/spec/1.2.2/#tags).

It is possible to override the default behavior by providing an explicit “ `TAG` ” directive, associating a different prefix for this handle. This provides smooth migration from using [local tags](https://yaml.org/spec/1.2.2/#tags) to using [global tags](https://yaml.org/spec/1.2.2/#tags) by the simple addition of a single “ `TAG` ” directive.

```
[90] c-primary-tag-handle ::= '!'
```

**Example 6.18 Primary Tag Handle**

| ``` # Private !foo "bar" ... # Global %TAG ! tag:example.com,2000:app/ --- !foo "bar" ``` | ```json !<!foo> "bar" --- !<tag:example.com,2000:app/foo> "bar" ``` |
| --- | --- |

**Legend:**

- `[c-primary-tag-handle](https://yaml.org/spec/1.2.2/#rule-c-primary-tag-handle)`

Secondary Handle

The *secondary tag handle* is written as “`!!`”. This allows using a compact notation for a single “secondary” name space. By default, the prefix associated with this handle is “ `tag:yaml.org,2002:`”.

It is possible to override this default behavior by providing an explicit “ `TAG` ” directive associating a different prefix for this handle.

```
[91] c-secondary-tag-handle ::= "!!"
```

**Example 6.19 Secondary Tag Handle**

| ``` %TAG !! tag:example.com,2000:app/ --- !!int 1 - 3 # Interval, not integer ``` | ```json !<tag:example.com,2000:app/int> "1 - 3" ``` |
| --- | --- |

**Legend:**

- `[c-secondary-tag-handle](https://yaml.org/spec/1.2.2/#rule-c-secondary-tag-handle)`

Named Handles

A *named tag handle* surrounds a non-empty name with “`!`” characters. A handle name must not be used in a [tag shorthand](https://yaml.org/spec/1.2.2/#tag-shorthands) unless an explicit “ `TAG` ” directive has associated some prefix with it.

The name of the handle is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information. In particular, the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) need not preserve the handle name once [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) is completed.

```
[92] c-named-tag-handle ::=
  c-tag            # '!'
  ns-word-char+
  c-tag            # '!'
```

**Example 6.20 Tag Handles**

| ``` %TAG !e! tag:example.com,2000:app/ --- !e!foo "bar" ``` | ```json !<tag:example.com,2000:app/foo> "bar" ``` |
| --- | --- |

**Legend:**

- `[c-named-tag-handle](https://yaml.org/spec/1.2.2/#rule-c-named-tag-handle)`

#### 6.8.2.2. Tag Prefixes

There are two *tag prefix* variants:

```
[93] ns-tag-prefix ::=
  c-ns-local-tag-prefix | ns-global-tag-prefix
```

Local Tag Prefix

If the prefix begins with a “`!`” character, [shorthands](https://yaml.org/spec/1.2.2/#tag-shorthands) using the [handle](https://yaml.org/spec/1.2.2/#tag-handles) are expanded to a [local tag](https://yaml.org/spec/1.2.2/#tags). Note that such a [tag](https://yaml.org/spec/1.2.2/#tags) is intentionally not a valid URI and its semantics are specific to the [application](https://yaml.org/spec/1.2.2/#processes-and-models). In particular, two [documents](https://yaml.org/spec/1.2.2/#documents) in the same [stream](https://yaml.org/spec/1.2.2/#streams) may assign different semantics to the same [local tag](https://yaml.org/spec/1.2.2/#tags).

```
[94] c-ns-local-tag-prefix ::=
  c-tag           # '!'
  ns-uri-char*
```

**Example 6.21 Local Tag Prefix**

| ``` %TAG !m! !my- --- # Bulb here !m!light fluorescent ... %TAG !m! !my- --- # Color here !m!light green ``` | ```json !<!my-light> "fluorescent" --- !<!my-light> "green" ``` |
| --- | --- |

**Legend:**

- `[c-ns-local-tag-prefix](https://yaml.org/spec/1.2.2/#rule-c-ns-local-tag-prefix)`

Global Tag Prefix

If the prefix begins with a character other than “`!`”, it must be a valid URI prefix, and should contain at least the scheme.[Shorthands](https://yaml.org/spec/1.2.2/#tag-shorthands) using the associated [handle](https://yaml.org/spec/1.2.2/#tag-handles) are expanded to globally unique URI tags and their semantics is consistent across [applications](https://yaml.org/spec/1.2.2/#processes-and-models). In particular, every [document](https://yaml.org/spec/1.2.2/#documents) in every [stream](https://yaml.org/spec/1.2.2/#streams) must assign the same semantics to the same [global tag](https://yaml.org/spec/1.2.2/#tags).

```
[95] ns-global-tag-prefix ::=
  ns-tag-char
  ns-uri-char*
```

**Example 6.22 Global Tag Prefix**

| ``` %TAG !e! tag:example.com,2000:app/ --- - !e!foo "bar" ``` | ``` - !<tag:example.com,2000:app/foo> "bar" ``` |
| --- | --- |

**Legend:**

- `[ns-global-tag-prefix](https://yaml.org/spec/1.2.2/#rule-ns-global-tag-prefix)`

## 6.9. Node Properties

Each [node](https://yaml.org/spec/1.2.2/#nodes) may have two optional *properties*, [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases) and [tag](https://yaml.org/spec/1.2.2/#tags), in addition to its [content](https://yaml.org/spec/1.2.2/#nodes). Node properties may be specified in any order before the [node’s content](https://yaml.org/spec/1.2.2/#nodes). Either or both may be omitted.

```
[96] c-ns-properties(n,c) ::=
    (
      c-ns-tag-property
      (
        s-separate(n,c)
        c-ns-anchor-property
      )?
    )
  | (
      c-ns-anchor-property
      (
        s-separate(n,c)
        c-ns-tag-property
      )?
    )
```

**Example 6.23 Node Properties**

| ``` !!str &a1 "foo":   !!str bar &a2 baz : *a1 ``` | ```json { &B1 "foo": "bar",   "baz": *B1 } ``` |
| --- | --- |

**Legend:**

- `[c-ns-properties(n,c)](https://yaml.org/spec/1.2.2/#rule-c-ns-properties)`
- `[c-ns-anchor-property](https://yaml.org/spec/1.2.2/#rule-c-ns-anchor-property)`
- `[c-ns-tag-property](https://yaml.org/spec/1.2.2/#rule-c-ns-tag-property)`

### 6.9.1. Node Tags

The *tag property* identifies the type of the [native data structure](https://yaml.org/spec/1.2.2/#representing-native-data-structures) [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) by the [node](https://yaml.org/spec/1.2.2/#nodes). A tag is denoted by the “`!`” indicator.

```
[97] c-ns-tag-property ::=
    c-verbatim-tag
  | c-ns-shorthand-tag
  | c-non-specific-tag
```

Verbatim Tags

A tag may be written *verbatim* by surrounding it with the “ `<` ” and “ `>` ” characters. In this case, the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) must deliver the verbatim tag as-is to the [application](https://yaml.org/spec/1.2.2/#processes-and-models). In particular, verbatim tags are not subject to [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution). A verbatim tag must either begin with a “`!`” (a [local tag](https://yaml.org/spec/1.2.2/#tags)) or be a valid URI (a [global tag](https://yaml.org/spec/1.2.2/#tags)).

```
[98] c-verbatim-tag ::=
  "!<"
  ns-uri-char+
  '>'
```

**Example 6.24 Verbatim Tags**

| ``` !<tag:yaml.org,2002:str> foo :   !<!bar> baz ``` | ```json { "foo": !<!bar> "baz" } ``` |
| --- | --- |

**Legend:**

- `[c-verbatim-tag](https://yaml.org/spec/1.2.2/#rule-c-verbatim-tag)`

**Example 6.25 Invalid Verbatim Tags**

| ``` - !<!> foo - !<$:?> bar ``` | ``` ERROR: - Verbatim tags aren't resolved,   so ! is invalid. - The $:? tag is neither a global   URI tag nor a local tag starting   with '!'. ``` |
| --- | --- |

Tag Shorthands

A *tag shorthand* consists of a valid [tag handle](https://yaml.org/spec/1.2.2/#tag-handles) followed by a non-empty suffix. The [tag handle](https://yaml.org/spec/1.2.2/#tag-handles) must be associated with a [prefix](https://yaml.org/spec/1.2.2/#tag-prefixes), either by default or by using a “ `TAG` ” directive. The resulting [parsed](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) [tag](https://yaml.org/spec/1.2.2/#tags) is the concatenation of the [prefix](https://yaml.org/spec/1.2.2/#tag-prefixes) and the suffix and must either begin with “`!`” (a [local tag](https://yaml.org/spec/1.2.2/#tags)) or be a valid URI (a [global tag](https://yaml.org/spec/1.2.2/#tags)).

The choice of [tag handle](https://yaml.org/spec/1.2.2/#tag-handles) is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information. In particular, the [tag handle](https://yaml.org/spec/1.2.2/#tag-handles) may be discarded once [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) is completed.

The suffix must not contain any “`!`” character. This would cause the tag shorthand to be interpreted as having a [named tag handle](https://yaml.org/spec/1.2.2/#tag-handles). In addition, the suffix must not contain the “ `[`”, “`]` ”, “ `{` ”, “ `}` ” and “`,`” characters. These characters would cause ambiguity with [flow collection](https://yaml.org/spec/1.2.2/#flow-collection-styles) structures. If the suffix needs to specify any of the above restricted characters, they must be [escaped](https://yaml.org/spec/1.2.2/#escaped-characters) using the “ `%` ” character. This behavior is consistent with the URI character escaping rules (specifically, section 2.3 of URI RFC).

```
[99] c-ns-shorthand-tag ::=
  c-tag-handle
  ns-tag-char+
```

**Example 6.26 Tag Shorthands**

| ``` %TAG !e! tag:example.com,2000:app/ --- - !local foo - !!str bar - !e!tag%21 baz ``` | ```json [ !<!local> "foo",   !<tag:yaml.org,2002:str> "bar",   !<tag:example.com,2000:app/tag!> "baz" ] ``` |
| --- | --- |

**Legend:**

- `[c-ns-shorthand-tag](https://yaml.org/spec/1.2.2/#rule-c-ns-shorthand-tag)`

**Example 6.27 Invalid Tag Shorthands**

| ``` %TAG !e! tag:example,2000:app/ --- - !e! foo - !h!bar baz ``` | ``` ERROR: - The !e! handle has no suffix. - The !h! handle wasn't declared. ``` |
| --- | --- |

Non-Specific Tags

If a [node](https://yaml.org/spec/1.2.2/#nodes) has no tag property, it is assigned a [non-specific tag](https://yaml.org/spec/1.2.2/#resolved-tags) that needs to be [resolved](https://yaml.org/spec/1.2.2/#resolved-tags) to a [specific](https://yaml.org/spec/1.2.2/#resolved-tags) one. This [non-specific tag](https://yaml.org/spec/1.2.2/#resolved-tags) is “`!`” for non- [plain scalars](https://yaml.org/spec/1.2.2/#plain-style) and “`?`” for all other [nodes](https://yaml.org/spec/1.2.2/#nodes). This is the only case where the [node style](https://yaml.org/spec/1.2.2/#node-styles) has any effect on the [content](https://yaml.org/spec/1.2.2/#nodes) information.

It is possible for the tag property to be explicitly set to the “`!`” non-specific tag. By [convention](https://yaml.org/spec/1.2.2/#resolved-tags), this “disables” [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution), forcing the [node](https://yaml.org/spec/1.2.2/#nodes) to be interpreted as “ `tag:yaml.org,2002:seq` ”, “ `tag:yaml.org,2002:map` ” or “ `tag:yaml.org,2002:str` ”, according to its [kind](https://yaml.org/spec/1.2.2/#nodes).

There is no way to explicitly specify the “`?`” non-specific tag. This is intentional.

```
[100] c-non-specific-tag ::= '!'
```

**Example 6.28 Non-Specific Tags**

| ``` # Assuming conventional resolution: - "12" - 12 - ! 12 ``` | ```json [ "12",   12,   "12" ] ``` |
| --- | --- |

**Legend:**

- `[c-non-specific-tag](https://yaml.org/spec/1.2.2/#rule-c-non-specific-tag)`

### 6.9.2. Node Anchors

An anchor is denoted by the “ `&` ” indicator. It marks a [node](https://yaml.org/spec/1.2.2/#nodes) for future reference. An [alias node](https://yaml.org/spec/1.2.2/#alias-nodes) can then be used to indicate additional inclusions of the anchored [node](https://yaml.org/spec/1.2.2/#nodes). An anchored [node](https://yaml.org/spec/1.2.2/#nodes) need not be referenced by any [alias nodes](https://yaml.org/spec/1.2.2/#alias-nodes); in particular, it is valid for all [nodes](https://yaml.org/spec/1.2.2/#nodes) to be anchored.

```
[101] c-ns-anchor-property ::=
  c-anchor          # '&'
  ns-anchor-name
```

Note that as a [serialization detail](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph), the anchor name is preserved in the [serialization tree](https://yaml.org/spec/1.2.2/#serialization-tree). However, it is not reflected in the [representation](https://yaml.org/spec/1.2.2/#representation-graph) graph and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information. In particular, the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) need not preserve the anchor name once the [representation](https://yaml.org/spec/1.2.2/#representation-graph) is [composed](https://yaml.org/spec/1.2.2/#composing-the-representation-graph).

Anchor names must not contain the “ `[`”, “`]` ”, “ `{` ”, “ `}` ” and “`,`” characters. These characters would cause ambiguity with [flow collection](https://yaml.org/spec/1.2.2/#flow-collection-styles) structures.

```
[102] ns-anchor-char ::=
    ns-char - c-flow-indicator
```
```
[103] ns-anchor-name ::=
  ns-anchor-char+
```

**Example 6.29 Node Anchors**

| ``` First occurrence: &anchor Value Second occurrence: *anchor ``` | ```json { "First occurrence": &A "Value",   "Second occurrence": *A } ``` |
| --- | --- |

**Legend:**

- `[c-ns-anchor-property](https://yaml.org/spec/1.2.2/#rule-c-ns-anchor-property)`
- `[ns-anchor-name](https://yaml.org/spec/1.2.2/#rule-ns-anchor-name)`

## Chapter 7. Flow Style Productions

YAML’s *flow styles* can be thought of as the natural extension of JSON to cover [folding](https://yaml.org/spec/1.2.2/#line-folding) long content lines for readability, [tagging](https://yaml.org/spec/1.2.2/#tags) nodes to control [construction](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) of [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) and using [anchors](https://yaml.org/spec/1.2.2/#anchors-and-aliases) and [aliases](https://yaml.org/spec/1.2.2/#anchors-and-aliases) to reuse [constructed](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) object instances.

## 7.1. Alias Nodes

Subsequent occurrences of a previously [serialized](https://yaml.org/spec/1.2.2/#serializing-the-representation-graph) node are [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) as *alias nodes*. The first occurrence of the [node](https://yaml.org/spec/1.2.2/#nodes) must be marked by an [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases) to allow subsequent occurrences to be [presented](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) as alias nodes.

An alias node is denoted by the “ `*` ” indicator. The alias refers to the most recent preceding [node](https://yaml.org/spec/1.2.2/#nodes) having the same [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases). It is an error for an alias node to use an [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases) that does not previously occur in the [document](https://yaml.org/spec/1.2.2/#documents). It is not an error to specify an [anchor](https://yaml.org/spec/1.2.2/#anchors-and-aliases) that is not used by any alias node.

Note that an alias node must not specify any [properties](https://yaml.org/spec/1.2.2/#node-properties) or [content](https://yaml.org/spec/1.2.2/#nodes), as these were already specified at the first occurrence of the [node](https://yaml.org/spec/1.2.2/#nodes).

```
[104] c-ns-alias-node ::=
  c-alias           # '*'
  ns-anchor-name
```

**Example 7.1 Alias Nodes**

| ``` First occurrence: &anchor Foo Second occurrence: *anchor Override anchor: &anchor Bar Reuse anchor: *anchor ``` | ```json { "First occurrence": &A "Foo",   "Override anchor": &B "Bar",   "Second occurrence": *A,   "Reuse anchor": *B } ``` |
| --- | --- |

**Legend:**

- `[c-ns-alias-node](https://yaml.org/spec/1.2.2/#rule-c-ns-alias-node)`
- `[ns-anchor-name](https://yaml.org/spec/1.2.2/#rule-ns-anchor-name)`

## 7.2. Empty Nodes

YAML allows the [node content](https://yaml.org/spec/1.2.2/#nodes) to be omitted in many cases.[Nodes](https://yaml.org/spec/1.2.2/#nodes) with empty [content](https://yaml.org/spec/1.2.2/#nodes) are interpreted as if they were [plain scalars](https://yaml.org/spec/1.2.2/#plain-style) with an empty value. Such [nodes](https://yaml.org/spec/1.2.2/#nodes) are commonly resolved to a “ `null` ” value.

```
[105] e-scalar ::= ""
```

In the examples, empty [scalars](https://yaml.org/spec/1.2.2/#scalars) are sometimes displayed as the glyph “ `°` ” for clarity. Note that this glyph corresponds to a position in the characters [stream](https://yaml.org/spec/1.2.2/#streams) rather than to an actual character.

**Example 7.2 Empty Content**

| ``` {   foo : !!str°,   !!str° : bar, } ``` | ```json { "foo": "",   "": "bar" } ``` |
| --- | --- |

**Legend:**

- `[e-scalar](https://yaml.org/spec/1.2.2/#rule-e-scalar)`

Both the [node’s properties](https://yaml.org/spec/1.2.2/#node-properties) and [node content](https://yaml.org/spec/1.2.2/#nodes) are optional. This allows for a *completely empty node*. Completely empty nodes are only valid when following some explicit indication for their existence.

```
[106] e-node ::=
  e-scalar    # ""
```

**Example 7.3 Completely Empty Flow Nodes**

| ``` {   ? foo :°,   °: bar, } ``` | ```json { "foo": null,   null : "bar" } ``` |
| --- | --- |

**Legend:**

- `[e-node](https://yaml.org/spec/1.2.2/#rule-e-node)`

## 7.3. Flow Scalar Styles

YAML provides three *flow scalar styles*: [double-quoted](https://yaml.org/spec/1.2.2/#double-quoted-style), [single-quoted](https://yaml.org/spec/1.2.2/#single-quoted-style) and [plain](https://yaml.org/spec/1.2.2/#plain-style) (unquoted). Each provides a different trade-off between readability and expressive power.

The [scalar style](https://yaml.org/spec/1.2.2/#node-styles) is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information, with the exception that [plain scalars](https://yaml.org/spec/1.2.2/#plain-style) are distinguished for the purpose of [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution).

### 7.3.1. Double-Quoted Style

The *double-quoted style* is specified by surrounding “ `"` ” indicators. This is the only [style](https://yaml.org/spec/1.2.2/#node-styles) capable of expressing arbitrary strings, by using “ `\` ” [escape sequences](https://yaml.org/spec/1.2.2/#escaped-characters). This comes at the cost of having to escape the “ `\` ” and “ `"` ” characters.

```
[107] nb-double-char ::=
    c-ns-esc-char
  | (
        nb-json
      - c-escape          # '\'
      - c-double-quote    # '"'
    )
```
```
[108] ns-double-char ::=
  nb-double-char - s-white
```

Double-quoted scalars are restricted to a single line when contained inside an [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry).

```
[109] c-double-quoted(n,c) ::=
  c-double-quote         # '"'
  nb-double-text(n,c)
  c-double-quote         # '"'
```
```
[110]
nb-double-text(n,FLOW-OUT)  ::= nb-double-multi-line(n)
nb-double-text(n,FLOW-IN)   ::= nb-double-multi-line(n)
nb-double-text(n,BLOCK-KEY) ::= nb-double-one-line
nb-double-text(n,FLOW-KEY)  ::= nb-double-one-line
```
```
[111] nb-double-one-line ::=
  nb-double-char*
```

**Example 7.4 Double Quoted Implicit Keys**

| ``` "implicit block key" : [   "implicit flow key" : value,  ] ``` | ```json { "implicit block key":   [ { "implicit flow key": "value" } ] } ``` |
| --- | --- |

**Legend:**

- `[nb-double-one-line](https://yaml.org/spec/1.2.2/#rule-nb-double-one-line)`
- `[c-double-quoted(n,c)](https://yaml.org/spec/1.2.2/#rule-c-double-quoted)`

In a multi-line double-quoted scalar, [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) are subject to [flow line folding](https://yaml.org/spec/1.2.2/#flow-folding), which discards any trailing [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters. It is also possible to *escape* the [line break](https://yaml.org/spec/1.2.2/#line-break-characters) character. In this case, the escaped [line break](https://yaml.org/spec/1.2.2/#line-break-characters) is excluded from the [content](https://yaml.org/spec/1.2.2/#nodes) and any trailing [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters that precede the escaped line break are preserved. Combined with the ability to [escape](https://yaml.org/spec/1.2.2/#escaped-characters) [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters, this allows double-quoted lines to be broken at arbitrary positions.

```
[112] s-double-escaped(n) ::=
  s-white*
  c-escape         # '\'
  b-non-content
  l-empty(n,FLOW-IN)*
  s-flow-line-prefix(n)
```
```
[113] s-double-break(n) ::=
    s-double-escaped(n)
  | s-flow-folded(n)
```

**Example 7.5 Double Quoted Line Breaks**

| ``` "folded·↓ to a space,→↓ ·↓ to a line feed, or·→\↓ ·\·→non-content" ``` | ```json "folded to a space,\nto a line feed, or \t \tnon-content" ``` |
| --- | --- |

**Legend:**

- `[s-flow-folded(n)](https://yaml.org/spec/1.2.2/#rule-s-flow-folded)`
- `[s-double-escaped(n)](https://yaml.org/spec/1.2.2/#rule-s-double-escaped)`

All leading and trailing [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters on each line are excluded from the [content](https://yaml.org/spec/1.2.2/#nodes). Each continuation line must therefore contain at least one non- [space](https://yaml.org/spec/1.2.2/#white-space-characters) character. Empty lines, if any, are consumed as part of the [line folding](https://yaml.org/spec/1.2.2/#line-folding).

```
[114] nb-ns-double-in-line ::=
  (
    s-white*
    ns-double-char
  )*
```
```
[115] s-double-next-line(n) ::=
  s-double-break(n)
  (
    ns-double-char nb-ns-double-in-line
    (
        s-double-next-line(n)
      | s-white*
    )
  )?
```
```
[116] nb-double-multi-line(n) ::=
  nb-ns-double-in-line
  (
      s-double-next-line(n)
    | s-white*
  )
```

**Example 7.6 Double Quoted Lines**

| ``` "·1st non-empty↓ ↓ ·2nd non-empty· →3rd non-empty·" ``` | ```json " 1st non-empty\n2nd non-empty 3rd non-empty " ``` |
| --- | --- |

**Legend:**

- `[nb-ns-double-in-line](https://yaml.org/spec/1.2.2/#rule-nb-ns-double-in-line)`
- `[s-double-next-line(n)](https://yaml.org/spec/1.2.2/#rule-s-double-next-line)`

### 7.3.2. Single-Quoted Style

The *single-quoted style* is specified by surrounding “ `'` ” indicators. Therefore, within a single-quoted scalar, such characters need to be repeated. This is the only form of *escaping* performed in single-quoted scalars. In particular, the “ `\` ” and “ `"` ” characters may be freely used. This restricts single-quoted scalars to [printable](https://yaml.org/spec/1.2.2/#character-set) characters. In addition, it is only possible to break a long single-quoted line where a [space](https://yaml.org/spec/1.2.2/#white-space-characters) character is surrounded by non- [spaces](https://yaml.org/spec/1.2.2/#white-space-characters).

```
[117] c-quoted-quote ::= "''"
```
```
[118] nb-single-char ::=
    c-quoted-quote
  | (
        nb-json
      - c-single-quote    # "'"
    )
```
```
[119] ns-single-char ::=
  nb-single-char - s-white
```

**Example 7.7 Single Quoted Characters**

| ``` 'here''s to "quotes"' ``` | ```json "here's to \"quotes\"" ``` |
| --- | --- |

**Legend:**

- `[c-quoted-quote](https://yaml.org/spec/1.2.2/#rule-c-quoted-quote)`

Single-quoted scalars are restricted to a single line when contained inside a [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry).

```
[120] c-single-quoted(n,c) ::=
  c-single-quote    # "'"
  nb-single-text(n,c)
  c-single-quote    # "'"
```
```
[121]
nb-single-text(FLOW-OUT)  ::= nb-single-multi-line(n)
nb-single-text(FLOW-IN)   ::= nb-single-multi-line(n)
nb-single-text(BLOCK-KEY) ::= nb-single-one-line
nb-single-text(FLOW-KEY)  ::= nb-single-one-line
```
```
[122] nb-single-one-line ::=
  nb-single-char*
```

**Example 7.8 Single Quoted Implicit Keys**

| ``` 'implicit block key' : [   'implicit flow key' : value,  ] ``` | ```json { "implicit block key":   [ { "implicit flow key": "value" } ] } ``` |
| --- | --- |

**Legend:**

- `[nb-single-one-line](https://yaml.org/spec/1.2.2/#rule-nb-single-one-line)`
- `[c-single-quoted(n,c)](https://yaml.org/spec/1.2.2/#rule-c-single-quoted)`

All leading and trailing [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters are excluded from the [content](https://yaml.org/spec/1.2.2/#nodes). Each continuation line must therefore contain at least one non- [space](https://yaml.org/spec/1.2.2/#white-space-characters) character. Empty lines, if any, are consumed as part of the [line folding](https://yaml.org/spec/1.2.2/#line-folding).

```
[123] nb-ns-single-in-line ::=
  (
    s-white*
    ns-single-char
  )*
```
```
[124] s-single-next-line(n) ::=
  s-flow-folded(n)
  (
    ns-single-char
    nb-ns-single-in-line
    (
        s-single-next-line(n)
      | s-white*
    )
  )?
```
```
[125] nb-single-multi-line(n) ::=
  nb-ns-single-in-line
  (
      s-single-next-line(n)
    | s-white*
  )
```

**Example 7.9 Single Quoted Lines**

| ``` '·1st non-empty↓ ↓ ·2nd non-empty· →3rd non-empty·' ``` | ```json " 1st non-empty\n2nd non-empty 3rd non-empty " ``` |
| --- | --- |

**Legend:**

- `[nb-ns-single-in-line(n)](https://yaml.org/spec/1.2.2/#rule-nb-ns-single-in-line)`
- `[s-single-next-line(n)](https://yaml.org/spec/1.2.2/#rule-s-single-next-line)`

### 7.3.3. Plain Style

The *plain* (unquoted) style has no identifying [indicators](https://yaml.org/spec/1.2.2/#indicator-characters) and provides no form of escaping. It is therefore the most readable, most limited and most [context](https://yaml.org/spec/1.2.2/#context-c) sensitive [style](https://yaml.org/spec/1.2.2/#node-styles). In addition to a restricted character set, a plain scalar must not be empty or contain leading or trailing [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters. It is only possible to break a long plain line where a [space](https://yaml.org/spec/1.2.2/#white-space-characters) character is surrounded by non- [spaces](https://yaml.org/spec/1.2.2/#white-space-characters).

Plain scalars must not begin with most [indicators](https://yaml.org/spec/1.2.2/#indicator-characters), as this would cause ambiguity with other YAML constructs. However, the “`:`”, “`?`” and “ `-` ” [indicators](https://yaml.org/spec/1.2.2/#indicator-characters) may be used as the first character if followed by a non- [space](https://yaml.org/spec/1.2.2/#white-space-characters) “safe” character, as this causes no ambiguity.

```
[126] ns-plain-first(c) ::=
    (
        ns-char
      - c-indicator
    )
  | (
      (
          c-mapping-key       # '?'
        | c-mapping-value     # ':'
        | c-sequence-entry    # '-'
      )
      [ lookahead = ns-plain-safe(c) ]
    )
```

Plain scalars must never contain the “`: `” and “ ` #` ” character combinations. Such combinations would cause ambiguity with [mapping](https://yaml.org/spec/1.2.2/#mapping) [key/value pairs](https://yaml.org/spec/1.2.2/#mapping) and [comments](https://yaml.org/spec/1.2.2/#comments). In addition, inside [flow collections](https://yaml.org/spec/1.2.2/#flow-collection-styles), or when used as [implicit keys](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry), plain scalars must not contain the “ `[`”, “`]` ”, “ `{` ”, “ `}` ” and “`,`” characters. These characters would cause ambiguity with [flow collection](https://yaml.org/spec/1.2.2/#flow-collection-styles) structures.

```
[127]
ns-plain-safe(FLOW-OUT)  ::= ns-plain-safe-out
ns-plain-safe(FLOW-IN)   ::= ns-plain-safe-in
ns-plain-safe(BLOCK-KEY) ::= ns-plain-safe-out
ns-plain-safe(FLOW-KEY)  ::= ns-plain-safe-in
```
```
[128] ns-plain-safe-out ::=
  ns-char
```
```
[129] ns-plain-safe-in ::=
  ns-char - c-flow-indicator
```
```
[130] ns-plain-char(c) ::=
    (
        ns-plain-safe(c)
      - c-mapping-value    # ':'
      - c-comment          # '#'
    )
  | (
      [ lookbehind = ns-char ]
      c-comment          # '#'
    )
  | (
      c-mapping-value    # ':'
      [ lookahead = ns-plain-safe(c) ]
    )
```

**Example 7.10 Plain Characters**

| ``` # Outside flow collection: - ::vector - ": - ()" - Up, up, and away! - -123 - https://example.com/foo#bar # Inside flow collection: - [ ::vector,   ": - ()",   "Up, up and away!",   -123,   https://example.com/foo#bar ] ``` | ```json [ "::vector",   ": - ()",   "Up, up, and away!",   -123,   "http://example.com/foo#bar",   [ "::vector",     ": - ()",     "Up, up, and away!",     -123,     "http://example.com/foo#bar" ] ] ``` |
| --- | --- |

**Legend:**

- `[ns-plain-first(c)](https://yaml.org/spec/1.2.2/#rule-ns-plain-first)`
- `[ns-plain-char(c)](https://yaml.org/spec/1.2.2/#rule-ns-plain-char)`
- `Not ns-plain-first(c)`
- `Not ns-plain-char(c)`

Plain scalars are further restricted to a single line when contained inside an [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry).

```
[131]
ns-plain(n,FLOW-OUT)  ::= ns-plain-multi-line(n,FLOW-OUT)
ns-plain(n,FLOW-IN)   ::= ns-plain-multi-line(n,FLOW-IN)
ns-plain(n,BLOCK-KEY) ::= ns-plain-one-line(BLOCK-KEY)
ns-plain(n,FLOW-KEY)  ::= ns-plain-one-line(FLOW-KEY)
```
```
[132] nb-ns-plain-in-line(c) ::=
  (
    s-white*
    ns-plain-char(c)
  )*
```
```
[133] ns-plain-one-line(c) ::=
  ns-plain-first(c)
  nb-ns-plain-in-line(c)
```

**Example 7.11 Plain Implicit Keys**

| ``` implicit block key : [   implicit flow key : value,  ] ``` | ```json { "implicit block key":   [ { "implicit flow key": "value" } ] } ``` |
| --- | --- |

**Legend:**

- `[ns-plain-one-line(c)](https://yaml.org/spec/1.2.2/#rule-ns-plain-one-line)`

All leading and trailing [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters are excluded from the [content](https://yaml.org/spec/1.2.2/#nodes). Each continuation line must therefore contain at least one non- [space](https://yaml.org/spec/1.2.2/#white-space-characters) character. Empty lines, if any, are consumed as part of the [line folding](https://yaml.org/spec/1.2.2/#line-folding).

```
[134] s-ns-plain-next-line(n,c) ::=
  s-flow-folded(n)
  ns-plain-char(c)
  nb-ns-plain-in-line(c)
```
```
[135] ns-plain-multi-line(n,c) ::=
  ns-plain-one-line(c)
  s-ns-plain-next-line(n,c)*
```

**Example 7.12 Plain Lines**

| ``` 1st non-empty↓ ↓ ·2nd non-empty· →3rd non-empty ``` | ```json "1st non-empty\n2nd non-empty 3rd non-empty" ``` |
| --- | --- |

**Legend:**

- `[nb-ns-plain-in-line(c)](https://yaml.org/spec/1.2.2/#rule-nb-ns-plain-in-line)`
- `[s-ns-plain-next-line(n,c)](https://yaml.org/spec/1.2.2/#rule-s-ns-plain-next-line)`

## 7.4. Flow Collection Styles

A *flow collection* may be nested within a [block collection](https://yaml.org/spec/1.2.2/#block-collection-styles) (\[`FLOW-OUT` context\]), nested within another flow collection (\[`FLOW-IN` context\]) or be a part of an [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry) (\[`FLOW-KEY` context\] or \[`BLOCK-KEY` context\]). Flow collection entries are terminated by the “`,`” indicator. The final “`,`” may be omitted. This does not cause ambiguity because flow collection entries can never be [completely empty](https://yaml.org/spec/1.2.2/#example-empty-content).

```
[136]
in-flow(n,FLOW-OUT)  ::= ns-s-flow-seq-entries(n,FLOW-IN)
in-flow(n,FLOW-IN)   ::= ns-s-flow-seq-entries(n,FLOW-IN)
in-flow(n,BLOCK-KEY) ::= ns-s-flow-seq-entries(n,FLOW-KEY)
in-flow(n,FLOW-KEY)  ::= ns-s-flow-seq-entries(n,FLOW-KEY)
```

### 7.4.1. Flow Sequences

*Flow sequence content* is denoted by surrounding “ `[`” and “`]` ” characters.

```
[137] c-flow-sequence(n,c) ::=
  c-sequence-start    # '['
  s-separate(n,c)?
  in-flow(n,c)?
  c-sequence-end      # ']'
```

Sequence entries are separated by a “`,`” character.

```
[138] ns-s-flow-seq-entries(n,c) ::=
  ns-flow-seq-entry(n,c)
  s-separate(n,c)?
  (
    c-collect-entry     # ','
    s-separate(n,c)?
    ns-s-flow-seq-entries(n,c)?
  )?
```

**Example 7.13 Flow Sequence**

| ``` - [ one, two, ] - [three ,four] ``` | ```json [ [ "one",     "two" ],   [ "three",     "four" ] ] ``` |
| --- | --- |

**Legend:**

- `[c-sequence-start](https://yaml.org/spec/1.2.2/#rule-c-sequence-start) [c-sequence-end](https://yaml.org/spec/1.2.2/#rule-c-sequence-end)`
- `[ns-flow-seq-entry(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-seq-entry)`

Any [flow node](https://yaml.org/spec/1.2.2/#flow-nodes) may be used as a flow sequence entry. In addition, YAML provides a [compact notation](https://yaml.org/spec/1.2.2/#example-flow-mapping-adjacent-values) for the case where a flow sequence entry is a [mapping](https://yaml.org/spec/1.2.2/#mapping) with a [single key/value pair](https://yaml.org/spec/1.2.2/#mapping).

```
[139] ns-flow-seq-entry(n,c) ::=
  ns-flow-pair(n,c) | ns-flow-node(n,c)
```

**Example 7.14 Flow Sequence Entries**

| ``` [ "double  quoted", 'single            quoted', plain  text, [ nested ], single: pair, ] ``` | ```json [ "double quoted",   "single quoted",   "plain text",   [ "nested" ],   { "single": "pair" } ] ``` |
| --- | --- |

**Legend:**

- `[ns-flow-node(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-node)`
- `[ns-flow-pair(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-pair)`

### 7.4.2. Flow Mappings

*Flow mappings* are denoted by surrounding “ `{` ” and “ `}` ” characters.

```
[140] c-flow-mapping(n,c) ::=
  c-mapping-start       # '{'
  s-separate(n,c)?
  ns-s-flow-map-entries(n,in-flow(c))?
  c-mapping-end         # '}'
```

Mapping entries are separated by a “`,`” character.

```
[141] ns-s-flow-map-entries(n,c) ::=
  ns-flow-map-entry(n,c)
  s-separate(n,c)?
  (
    c-collect-entry     # ','
    s-separate(n,c)?
    ns-s-flow-map-entries(n,c)?
  )?
```

**Example 7.15 Flow Mappings**

| ``` - { one : two , three: four , } - {five: six,seven : eight} ``` | ```json [ { "one": "two",     "three": "four" },   { "five": "six",     "seven": "eight" } ] ``` |
| --- | --- |

**Legend:**

- `[c-mapping-start](https://yaml.org/spec/1.2.2/#rule-c-mapping-start) [c-mapping-end](https://yaml.org/spec/1.2.2/#rule-c-mapping-end)`
- `[ns-flow-map-entry(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-map-entry)`

If the optional “`?`” mapping key indicator is specified, the rest of the entry may be [completely empty](https://yaml.org/spec/1.2.2/#example-empty-content).

```
[142] ns-flow-map-entry(n,c) ::=
    (
      c-mapping-key    # '?' (not followed by non-ws char)
      s-separate(n,c)
      ns-flow-map-explicit-entry(n,c)
    )
  | ns-flow-map-implicit-entry(n,c)
```
```
[143] ns-flow-map-explicit-entry(n,c) ::=
    ns-flow-map-implicit-entry(n,c)
  | (
      e-node    # ""
      e-node    # ""
    )
```

**Example 7.16 Flow Mapping Entries**

| ``` { ? explicit: entry, implicit: entry, ?°° } ``` | ```json { "explicit": "entry",   "implicit": "entry",   null: null } ``` |
| --- | --- |

**Legend:**

- `[ns-flow-map-explicit-entry(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-map-explicit-entry)`
- `[ns-flow-map-implicit-entry(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-map-implicit-entry)`
- `[e-node](https://yaml.org/spec/1.2.2/#rule-e-node)`

Normally, YAML insists the “`:`” mapping value indicator be [separated](https://yaml.org/spec/1.2.2/#separation-spaces) from the [value](https://yaml.org/spec/1.2.2/#nodes) by [white space](https://yaml.org/spec/1.2.2/#white-space-characters). A benefit of this restriction is that the “`:`” character can be used inside [plain scalars](https://yaml.org/spec/1.2.2/#plain-style), as long as it is not followed by [white space](https://yaml.org/spec/1.2.2/#white-space-characters). This allows for unquoted URLs and timestamps. It is also a potential source for confusion as “ `a:1` ” is a [plain scalar](https://yaml.org/spec/1.2.2/#plain-style) and not a [key/value pair](https://yaml.org/spec/1.2.2/#mapping).

Note that the [value](https://yaml.org/spec/1.2.2/#nodes) may be [completely empty](https://yaml.org/spec/1.2.2/#example-empty-content) since its existence is indicated by the “`:`”.

```
[144] ns-flow-map-implicit-entry(n,c) ::=
    ns-flow-map-yaml-key-entry(n,c)
  | c-ns-flow-map-empty-key-entry(n,c)
  | c-ns-flow-map-json-key-entry(n,c)
```
```
[145] ns-flow-map-yaml-key-entry(n,c) ::=
  ns-flow-yaml-node(n,c)
  (
      (
        s-separate(n,c)?
        c-ns-flow-map-separate-value(n,c)
      )
    | e-node    # ""
  )
```
```
[146] c-ns-flow-map-empty-key-entry(n,c) ::=
  e-node    # ""
  c-ns-flow-map-separate-value(n,c)
```
```
[147] c-ns-flow-map-separate-value(n,c) ::=
  c-mapping-value    # ':'
  [ lookahead ≠ ns-plain-safe(c) ]
  (
      (
        s-separate(n,c)
        ns-flow-node(n,c)
      )
    | e-node    # ""
  )
```

**Example 7.17 Flow Mapping Separate Values**

| ``` { unquoted·:·"separate", https://foo.com, omitted value:°, °:·omitted key, } ``` | ```json { "unquoted": "separate",   "http://foo.com": null,   "omitted value": null,   null: "omitted key" } ``` |
| --- | --- |

**Legend:**

- `[ns-flow-yaml-node(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-yaml-node)`
- `[e-node](https://yaml.org/spec/1.2.2/#rule-e-node)`
- `[c-ns-flow-map-separate-value(n,c)](https://yaml.org/spec/1.2.2/#rule-c-ns-flow-map-separate-value)`

To ensure [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives), if a [key](https://yaml.org/spec/1.2.2/#nodes) inside a flow mapping is [JSON-like](https://yaml.org/spec/1.2.2/#flow-nodes), YAML allows the following [value](https://yaml.org/spec/1.2.2/#nodes) to be specified adjacent to the “`:`”. This causes no ambiguity, as all [JSON-like](https://yaml.org/spec/1.2.2/#flow-nodes) [keys](https://yaml.org/spec/1.2.2/#nodes) are surrounded by [indicators](https://yaml.org/spec/1.2.2/#indicator-characters). However, as this greatly reduces readability, YAML [processors](https://yaml.org/spec/1.2.2/#processes-and-models) should [separate](https://yaml.org/spec/1.2.2/#separation-spaces) the [value](https://yaml.org/spec/1.2.2/#nodes) from the “`:`” on output, even in this case.

```
[148] c-ns-flow-map-json-key-entry(n,c) ::=
  c-flow-json-node(n,c)
  (
      (
        s-separate(n,c)?
        c-ns-flow-map-adjacent-value(n,c)
      )
    | e-node    # ""
  )
```
```
[149] c-ns-flow-map-adjacent-value(n,c) ::=
  c-mapping-value          # ':'
  (
      (
        s-separate(n,c)?
        ns-flow-node(n,c)
      )
    | e-node    # ""
  )
```

A more compact notation is usable inside [flow sequences](https://yaml.org/spec/1.2.2/#flow-sequences), if the [mapping](https://yaml.org/spec/1.2.2/#mapping) contains a *single key/value pair*. This notation does not require the surrounding “ `{` ” and “ `}` ” characters. Note that it is not possible to specify any [node properties](https://yaml.org/spec/1.2.2/#node-properties) for the [mapping](https://yaml.org/spec/1.2.2/#mapping) in this case.

**Example 7.19 Single Pair Flow Mappings**

| ``` [ foo: bar ] ``` | ```json [ { "foo": "bar" } ] ``` |
| --- | --- |

**Legend:**

- `[ns-flow-pair(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-pair)`

If the “`?`” indicator is explicitly specified, [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) is unambiguous and the syntax is identical to the general case.

```
[150] ns-flow-pair(n,c) ::=
    (
      c-mapping-key     # '?' (not followed by non-ws char)
      s-separate(n,c)
      ns-flow-map-explicit-entry(n,c)
    )
  | ns-flow-pair-entry(n,c)
```

**Example 7.20 Single Pair Explicit Entry**

| ``` [ ? foo  bar : baz ] ``` | ```json [ { "foo bar": "baz" } ] ``` |
| --- | --- |

**Legend:**

- `[ns-flow-map-explicit-entry(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-map-explicit-entry)`

If the “`?`” indicator is omitted, [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) needs to see past the *implicit key* to recognize it as such. To limit the amount of lookahead required, the “`:`” indicator must appear at most 1024 Unicode characters beyond the start of the [key](https://yaml.org/spec/1.2.2/#nodes). In addition, the [key](https://yaml.org/spec/1.2.2/#nodes) is restricted to a single line.

Note that YAML allows arbitrary [nodes](https://yaml.org/spec/1.2.2/#nodes) to be used as [keys](https://yaml.org/spec/1.2.2/#nodes). In particular, a [key](https://yaml.org/spec/1.2.2/#nodes) may be a [sequence](https://yaml.org/spec/1.2.2/#sequence) or a [mapping](https://yaml.org/spec/1.2.2/#mapping). Thus, without the above restrictions, practical one-pass [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) would have been impossible to implement.

```
[151] ns-flow-pair-entry(n,c) ::=
    ns-flow-pair-yaml-key-entry(n,c)
  | c-ns-flow-map-empty-key-entry(n,c)
  | c-ns-flow-pair-json-key-entry(n,c)
```
```
[152] ns-flow-pair-yaml-key-entry(n,c) ::=
  ns-s-implicit-yaml-key(FLOW-KEY)
  c-ns-flow-map-separate-value(n,c)
```
```
[153] c-ns-flow-pair-json-key-entry(n,c) ::=
  c-s-implicit-json-key(FLOW-KEY)
  c-ns-flow-map-adjacent-value(n,c)
```
```
[154] ns-s-implicit-yaml-key(c) ::=
  ns-flow-yaml-node(0,c)
  s-separate-in-line?
  /* At most 1024 characters altogether */
```
```
[155] c-s-implicit-json-key(c) ::=
  c-flow-json-node(0,c)
  s-separate-in-line?
  /* At most 1024 characters altogether */
```

**Example 7.21 Single Pair Implicit Entries**

| ``` - [ YAML·: separate ] - [ °: empty key entry ] - [ {JSON: like}:adjacent ] ``` | ```json [ [ { "YAML": "separate" } ],   [ { null: "empty key entry" } ],   [ { { "JSON": "like" }: "adjacent" } ] ] ``` |
| --- | --- |

**Legend:**

- `[ns-s-implicit-yaml-key](https://yaml.org/spec/1.2.2/#rule-ns-s-implicit-yaml-key)`
- `[e-node](https://yaml.org/spec/1.2.2/#rule-e-node)`
- `[c-s-implicit-json-key](https://yaml.org/spec/1.2.2/#rule-c-s-implicit-json-key)`
- `Value`

**Example 7.22 Invalid Implicit Keys**

| ``` [ foo  bar: invalid,  "foo_...>1K characters..._bar": invalid ] ``` | ``` ERROR: - The foo bar key spans multiple lines - The foo...bar key is too long ``` |
| --- | --- |

## 7.5. Flow Nodes

*JSON-like* [flow styles](https://yaml.org/spec/1.2.2/#flow-style-productions) all have explicit start and end [indicators](https://yaml.org/spec/1.2.2/#indicator-characters). The only [flow style](https://yaml.org/spec/1.2.2/#flow-style-productions) that does not have this property is the [plain scalar](https://yaml.org/spec/1.2.2/#plain-style). Note that none of the “JSON-like” styles is actually acceptable by JSON. Even the [double-quoted style](https://yaml.org/spec/1.2.2/#double-quoted-style) is a superset of the JSON string format.

```
[156] ns-flow-yaml-content(n,c) ::=
  ns-plain(n,c)
```
```
[157] c-flow-json-content(n,c) ::=
    c-flow-sequence(n,c)
  | c-flow-mapping(n,c)
  | c-single-quoted(n,c)
  | c-double-quoted(n,c)
```
```
[158] ns-flow-content(n,c) ::=
    ns-flow-yaml-content(n,c)
  | c-flow-json-content(n,c)
```

**Example 7.23 Flow Content**

| ``` - [ a, b ] - { a: b } - "a" - 'b' - c ``` | ```json [ [ "a", "b" ],   { "a": "b" },   "a",   "b",   "c" ] ``` |
| --- | --- |

**Legend:**

- `[c-flow-json-content(n,c)](https://yaml.org/spec/1.2.2/#rule-c-flow-json-content)`
- `[ns-flow-yaml-content(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-yaml-content)`

A complete [flow](https://yaml.org/spec/1.2.2/#flow-style-productions) [node](https://yaml.org/spec/1.2.2/#nodes) also has optional [node properties](https://yaml.org/spec/1.2.2/#node-properties), except for [alias nodes](https://yaml.org/spec/1.2.2/#alias-nodes) which refer to the [anchored](https://yaml.org/spec/1.2.2/#anchors-and-aliases) [node properties](https://yaml.org/spec/1.2.2/#node-properties).

```
[159] ns-flow-yaml-node(n,c) ::=
    c-ns-alias-node
  | ns-flow-yaml-content(n,c)
  | (
      c-ns-properties(n,c)
      (
          (
            s-separate(n,c)
            ns-flow-yaml-content(n,c)
          )
        | e-scalar
      )
    )
```
```
[160] c-flow-json-node(n,c) ::=
  (
    c-ns-properties(n,c)
    s-separate(n,c)
  )?
  c-flow-json-content(n,c)
```
```
[161] ns-flow-node(n,c) ::=
    c-ns-alias-node
  | ns-flow-content(n,c)
  | (
      c-ns-properties(n,c)
      (
        (
          s-separate(n,c)
          ns-flow-content(n,c)
        )
        | e-scalar
      )
    )
```

**Example 7.24 Flow Nodes**

| ``` - !!str "a" - 'b' - &anchor "c" - *anchor - !!str° ``` | ```json [ "a",   "b",   "c",   "c",   "" ] ``` |
| --- | --- |

**Legend:**

- `[c-flow-json-node(n,c)](https://yaml.org/spec/1.2.2/#rule-c-flow-json-node)`
- `[ns-flow-yaml-node(n,c)](https://yaml.org/spec/1.2.2/#rule-ns-flow-yaml-node)`

## Chapter 8. Block Style Productions

YAML’s *block styles* employ [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) rather than [indicators](https://yaml.org/spec/1.2.2/#indicator-characters) to denote structure. This results in a more human readable (though less compact) notation.

## 8.1. Block Scalar Styles

YAML provides two *block scalar styles*, [literal](https://yaml.org/spec/1.2.2/#literal-style) and [folded](https://yaml.org/spec/1.2.2/#line-folding). Each provides a different trade-off between readability and expressive power.

### 8.1.1. Block Scalar Headers

[Block scalars](https://yaml.org/spec/1.2.2/#block-scalar-styles) are controlled by a few [indicators](https://yaml.org/spec/1.2.2/#indicator-characters) given in a *header* preceding the [content](https://yaml.org/spec/1.2.2/#nodes) itself. This header is followed by a non-content [line break](https://yaml.org/spec/1.2.2/#line-break-characters) with an optional [comment](https://yaml.org/spec/1.2.2/#comments). This is the only case where a [comment](https://yaml.org/spec/1.2.2/#comments) must not be followed by additional [comment](https://yaml.org/spec/1.2.2/#comments) lines.

> Note: See [Production Parameters](https://yaml.org/spec/1.2.2/#production-parameters) for the definition of the `t` variable.

```
[162] c-b-block-header(t) ::=
  (
      (
        c-indentation-indicator
        c-chomping-indicator(t)
      )
    | (
        c-chomping-indicator(t)
        c-indentation-indicator
      )
  )
  s-b-comment
```

**Example 8.1 Block Scalar Header**

| ``` - \| # Empty header↓  literal - >1 # Indentation indicator↓  ·folded - \|+ # Chomping indicator↓  keep  - >1- # Both indicators↓  ·strip ``` | ```json [ "literal\n",   " folded\n",   "keep\n\n",   " strip" ] ``` |
| --- | --- | --- | --- |

#### 8.1.1.1. Block Indentation Indicator

Every block scalar has a *content indentation level*. The content of the block scalar excludes a number of leading [spaces](https://yaml.org/spec/1.2.2/#white-space-characters) on each line up to the content indentation level.

If a block scalar has an *indentation indicator*, then the content indentation level of the block scalar is equal to the indentation level of the block scalar plus the integer value of the indentation indicator character.

If no indentation indicator is given, then the content indentation level is equal to the number of leading [spaces](https://yaml.org/spec/1.2.2/#white-space-characters) on the first non- [empty line](https://yaml.org/spec/1.2.2/#empty-lines) of the contents. If there is no non- [empty line](https://yaml.org/spec/1.2.2/#empty-lines) then the content indentation level is equal to the number of spaces on the longest line.

It is an error if any non- [empty line](https://yaml.org/spec/1.2.2/#empty-lines) does not begin with a number of spaces greater than or equal to the content indentation level.

It is an error for any of the leading [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) to contain more [spaces](https://yaml.org/spec/1.2.2/#white-space-characters) than the first non- [empty line](https://yaml.org/spec/1.2.2/#empty-lines).

A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should only emit an explicit indentation indicator for cases where detection will fail.

```
[163] c-indentation-indicator ::=
  [x31-x39]    # 1-9
```

**Example 8.2 Block Indentation Indicator**

| ``` - \|° ·detected - >° · ·· ··# detected - \|1 ··explicit - >° ·→ ·detected ``` | ```json [ "detected\n",   "\n\n# detected\n",   " explicit\n",   "\t\ndetected\n" ] ``` |
| --- | --- | --- | --- |

**Legend:**

- `[c-indentation-indicator](https://yaml.org/spec/1.2.2/#rule-c-indentation-indicator)`
- `[s-indent(n)](https://yaml.org/spec/1.2.2/#rule-s-indent)`

**Example 8.3 Invalid Block Scalar Indentation Indicators**

| ``` - \| ·· ·text - > ··text ·text - \|2 ·text ``` | ``` ERROR: - A leading all-space line must   not have too many spaces. - A following text line must   not be less indented. - The text is less indented   than the indicated level. ``` |
| --- | --- | --- | --- |

#### 8.1.1.2. Block Chomping Indicator

*Chomping* controls how final [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) and trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are interpreted. YAML provides three chomping methods:

Strip

*Stripping* is specified by the “ `-` ” chomping indicator. In this case, the final [line break](https://yaml.org/spec/1.2.2/#line-break-characters) and any trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are excluded from the [scalar’s content](https://yaml.org/spec/1.2.2/#scalar).

Clip

*Clipping* is the default behavior used if no explicit chomping indicator is specified. In this case, the final [line break](https://yaml.org/spec/1.2.2/#line-break-characters) character is preserved in the [scalar’s content](https://yaml.org/spec/1.2.2/#scalar). However, any trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are excluded from the [scalar’s content](https://yaml.org/spec/1.2.2/#scalar).

Keep

*Keeping* is specified by the “ `+` ” chomping indicator. In this case, the final [line break](https://yaml.org/spec/1.2.2/#line-break-characters) and any trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are considered to be part of the [scalar’s content](https://yaml.org/spec/1.2.2/#scalar). These additional lines are not subject to [folding](https://yaml.org/spec/1.2.2/#line-folding).

The chomping method used is a [presentation detail](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) and must not be used to convey [content](https://yaml.org/spec/1.2.2/#nodes) information.

```
[164]
c-chomping-indicator(STRIP) ::= '-'
c-chomping-indicator(KEEP)  ::= '+'
c-chomping-indicator(CLIP)  ::= ""
```

The interpretation of the final [line break](https://yaml.org/spec/1.2.2/#line-break-characters) of a [block scalar](https://yaml.org/spec/1.2.2/#block-scalar-styles) is controlled by the chomping indicator specified in the [block scalar header](https://yaml.org/spec/1.2.2/#block-scalar-headers).

```
[165]
b-chomped-last(STRIP) ::= b-non-content  | <end-of-input>
b-chomped-last(CLIP)  ::= b-as-line-feed | <end-of-input>
b-chomped-last(KEEP)  ::= b-as-line-feed | <end-of-input>
```

**Example 8.4 Chomping Final Line Break**

| ``` strip: \|-   text↓ clip: \|   text↓ keep: \|+   text↓ ``` | ```json { "strip": "text",   "clip": "text\n",   "keep": "text\n" } ``` |
| --- | --- | --- | --- | --- |

**Legend:**

- `[b-non-content](https://yaml.org/spec/1.2.2/#rule-b-non-content)`
- `[b-as-line-feed](https://yaml.org/spec/1.2.2/#rule-b-as-line-feed)`

The interpretation of the trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) following a [block scalar](https://yaml.org/spec/1.2.2/#block-scalar-styles) is also controlled by the chomping indicator specified in the [block scalar header](https://yaml.org/spec/1.2.2/#block-scalar-headers).

```
[166]
l-chomped-empty(n,STRIP) ::= l-strip-empty(n)
l-chomped-empty(n,CLIP)  ::= l-strip-empty(n)
l-chomped-empty(n,KEEP)  ::= l-keep-empty(n)
```
```
[167] l-strip-empty(n) ::=
  (
    s-indent-less-or-equal(n)
    b-non-content
  )*
  l-trail-comments(n)?
```
```
[168] l-keep-empty(n) ::=
  l-empty(n,BLOCK-IN)*
  l-trail-comments(n)?
```

Explicit [comment](https://yaml.org/spec/1.2.2/#comments) lines may follow the trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines). To prevent ambiguity, the first such [comment](https://yaml.org/spec/1.2.2/#comments) line must be less [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) than the [block scalar content](https://yaml.org/spec/1.2.2/#block-scalar-styles). Additional [comment](https://yaml.org/spec/1.2.2/#comments) lines, if any, are not so restricted. This is the only case where the [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) of [comment](https://yaml.org/spec/1.2.2/#comments) lines is constrained.

```
[169] l-trail-comments(n) ::=
  s-indent-less-than(n)
  c-nb-comment-text
  b-comment
  l-comment*
```

**Example 8.5 Chomping Trailing Lines**

| ``` # Strip   # Comments: strip: \|-   # text↓ ··⇓ ·# Clip ··# comments: ↓ clip: \|   # text↓ ·↓ ·# Keep ··# comments: ↓ keep: \|+   # text↓ ↓ ·# Trail ··# comments. ``` | ```json { "strip": "# text",   "clip": "# text\n",   "keep": "# text\n\n" } ``` |
| --- | --- | --- | --- | --- |

If a [block scalar](https://yaml.org/spec/1.2.2/#block-scalar-styles) consists only of [empty lines](https://yaml.org/spec/1.2.2/#empty-lines), then these lines are considered as trailing lines and hence are affected by chomping.

**Example 8.6 Empty Scalar Chomping**

| ``` strip: >- ↓ clip: > ↓ keep: \|+ ↓ ``` | ```json { "strip": "",   "clip": "",   "keep": "\n" } ``` |
| --- | --- | --- |

**Legend:**

- `[l-strip-empty(n)](https://yaml.org/spec/1.2.2/#rule-l-strip-empty)`
- `[l-keep-empty(n)](https://yaml.org/spec/1.2.2/#rule-l-keep-empty)`

### 8.1.2. Literal Style

The *literal style* is denoted by the “ `|` ” indicator. It is the simplest, most restricted and most readable [scalar style](https://yaml.org/spec/1.2.2/#node-styles).

```
[170] c-l+literal(n) ::=
  c-literal                # '|'
  c-b-block-header(t)
  l-literal-content(n+m,t)
```

**Example 8.7 Literal Scalar**

| ``` \|↓ ·literal↓ ·→text↓ ↓ ``` | ```json "literal\n\ttext\n" ``` |
| --- | --- | --- |

**Legend:**

- `[c-l+literal(n)](https://yaml.org/spec/1.2.2/#rule-c-l+literal)`

Inside literal scalars, all ([indented](https://yaml.org/spec/1.2.2/#indentation-spaces)) characters are considered to be [content](https://yaml.org/spec/1.2.2/#nodes), including [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters. Note that all [line break](https://yaml.org/spec/1.2.2/#line-break-characters) characters are [normalized](https://yaml.org/spec/1.2.2/#line-break-characters). In addition, [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are not [folded](https://yaml.org/spec/1.2.2/#line-folding), though final [line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) and trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) are [chomped](https://yaml.org/spec/1.2.2/#block-chomping-indicator).

There is no way to escape characters inside literal scalars. This restricts them to [printable](https://yaml.org/spec/1.2.2/#character-set) characters. In addition, there is no way to break a long literal line.

```
[171] l-nb-literal-text(n) ::=
  l-empty(n,BLOCK-IN)*
  s-indent(n) nb-char+
```
```
[172] b-nb-literal-next(n) ::=
  b-as-line-feed
  l-nb-literal-text(n)
```
```
[173] l-literal-content(n,t) ::=
  (
    l-nb-literal-text(n)
    b-nb-literal-next(n)*
    b-chomped-last(t)
  )?
  l-chomped-empty(n,t)
```

**Example 8.8 Literal Content**

| ``` \| · ·· ··literal↓ ···↓ ·· ··text↓ ↓ ·# Comment ``` | ```json "\n\nliteral\n·\n\ntext\n" ``` |
| --- | --- | --- |

### 8.1.3. Folded Style

The *folded style* is denoted by the “ `>` ” indicator. It is similar to the [literal style](https://yaml.org/spec/1.2.2/#literal-style); however, folded scalars are subject to [line folding](https://yaml.org/spec/1.2.2/#line-folding).

```
[174] c-l+folded(n) ::=
  c-folded                 # '>'
  c-b-block-header(t)
  l-folded-content(n+m,t)
```

**Example 8.9 Folded Scalar**

| ``` >↓ ·folded↓ ·text↓ ↓ ``` | ```json "folded text\n" ``` |
| --- | --- |

**Legend:**

- `[c-l+folded(n)](https://yaml.org/spec/1.2.2/#rule-c-l+folded)`

[Folding](https://yaml.org/spec/1.2.2/#line-folding) allows long lines to be broken anywhere a single [space](https://yaml.org/spec/1.2.2/#white-space-characters) character separates two non- [space](https://yaml.org/spec/1.2.2/#white-space-characters) characters.

```
[175] s-nb-folded-text(n) ::=
  s-indent(n)
  ns-char
  nb-char*
```
```
[176] l-nb-folded-lines(n) ::=
  s-nb-folded-text(n)
  (
    b-l-folded(n,BLOCK-IN)
    s-nb-folded-text(n)
  )*
```

**Example 8.10 Folded Lines**

| ``` >  ·folded↓ ·line↓ ↓ ·next ·line↓    * bullet     * list    * lines  ·last↓ ·line↓  # Comment ``` | ```json "\nfolded line\nnext line\n  \ * bullet\n \n  * list\n  \ * lines\n\nlast line\n" ``` |
| --- | --- |

**Legend:**

- `[l-nb-folded-lines(n)](https://yaml.org/spec/1.2.2/#rule-l-nb-folded-lines)`
- `[s-nb-folded-text(n)](https://yaml.org/spec/1.2.2/#rule-s-nb-folded-text)`

(The following three examples duplicate this example, each highlighting different productions.)

Lines starting with [white space](https://yaml.org/spec/1.2.2/#white-space-characters) characters (*more-indented* lines) are not [folded](https://yaml.org/spec/1.2.2/#line-folding).

```
[177] s-nb-spaced-text(n) ::=
  s-indent(n)
  s-white
  nb-char*
```
```
[178] b-l-spaced(n) ::=
  b-as-line-feed
  l-empty(n,BLOCK-IN)*
```
```
[179] l-nb-spaced-lines(n) ::=
  s-nb-spaced-text(n)
  (
    b-l-spaced(n)
    s-nb-spaced-text(n)
  )*
```

[Line breaks](https://yaml.org/spec/1.2.2/#line-break-characters) and [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) separating folded and more-indented lines are also not [folded](https://yaml.org/spec/1.2.2/#line-folding).

```
[180] l-nb-same-lines(n) ::=
  l-empty(n,BLOCK-IN)*
  (
      l-nb-folded-lines(n)
    | l-nb-spaced-lines(n)
  )
```
```
[181] l-nb-diff-lines(n) ::=
  l-nb-same-lines(n)
  (
    b-as-line-feed
    l-nb-same-lines(n)
  )*
```

**Example 8.12 Empty Separation Lines**

| ``` > ↓  folded  line↓ ↓  next  line↓    * bullet     * list    * lines↓ ↓  last  line  # Comment ``` | ```json "\nfolded line\nnext line\n  \ * bullet\n \n  * list\n  \ * lines\n\nlast line\n" ``` |
| --- | --- |

**Legend:**

- `[b-as-line-feed](https://yaml.org/spec/1.2.2/#rule-b-as-line-feed)`
- `(separation) [l-empty(n,c)](https://yaml.org/spec/1.2.2/#rule-l-empty)`

The final [line break](https://yaml.org/spec/1.2.2/#line-break-characters) and trailing [empty lines](https://yaml.org/spec/1.2.2/#empty-lines) if any, are subject to [chomping](https://yaml.org/spec/1.2.2/#block-chomping-indicator) and are never [folded](https://yaml.org/spec/1.2.2/#line-folding).

```
[182] l-folded-content(n,t) ::=
  (
    l-nb-diff-lines(n)
    b-chomped-last(t)
  )?
  l-chomped-empty(n,t)
```

**Example 8.13 Final Empty Lines**

| ``` >   folded  line   next  line    * bullet     * list    * lines   last  line↓ ↓ # Comment ``` | ```json "\nfolded line\nnext line\n  \ * bullet\n \n  * list\n  \ * lines\n\nlast line\n" ``` |
| --- | --- |

**Legend:**

- `[b-chomped-last(t)](https://yaml.org/spec/1.2.2/#rule-b-chomped-last)`
- `[l-chomped-empty(n,t)](https://yaml.org/spec/1.2.2/#rule-l-chomped-empty)`

## 8.2. Block Collection Styles

For readability, *block collections styles* are not denoted by any [indicator](https://yaml.org/spec/1.2.2/#indicator-characters). Instead, YAML uses a lookahead method, where a block collection is distinguished from a [plain scalar](https://yaml.org/spec/1.2.2/#plain-style) only when a [key/value pair](https://yaml.org/spec/1.2.2/#mapping) or a [sequence entry](https://yaml.org/spec/1.2.2/#block-sequences) is seen.

### 8.2.1. Block Sequences

A *block sequence* is simply a series of [nodes](https://yaml.org/spec/1.2.2/#nodes), each denoted by a leading “ `-` ” indicator. The “ `-` ” indicator must be [separated](https://yaml.org/spec/1.2.2/#separation-spaces) from the [node](https://yaml.org/spec/1.2.2/#nodes) by [white space](https://yaml.org/spec/1.2.2/#white-space-characters). This allows “ `-` ” to be used as the first character in a [plain scalar](https://yaml.org/spec/1.2.2/#plain-style) if followed by a non-space character (e.g. “ `-42` ”).

```
[183] l+block-sequence(n) ::=
  (
    s-indent(n+1+m)
    c-l-block-seq-entry(n+1+m)
  )+
```
```
[184] c-l-block-seq-entry(n) ::=
  c-sequence-entry    # '-'
  [ lookahead ≠ ns-char ]
  s-l+block-indented(n,BLOCK-IN)
```

**Example 8.14 Block Sequence**

| ``` block sequence: ··- one↓   - two : three↓ ``` | ```json { "block sequence": [     "one",     { "two": "three" } ] } ``` |
| --- | --- |

**Legend:**

- `[c-l-block-seq-entry(n)](https://yaml.org/spec/1.2.2/#rule-c-l-block-seq-entry)`
- `auto-detected [s-indent(n)](https://yaml.org/spec/1.2.2/#rule-s-indent)`

The entry [node](https://yaml.org/spec/1.2.2/#nodes) may be either [completely empty](https://yaml.org/spec/1.2.2/#example-empty-content), be a nested [block node](https://yaml.org/spec/1.2.2/#block-nodes) or use a *compact in-line notation*. The compact notation may be used when the entry is itself a nested [block collection](https://yaml.org/spec/1.2.2/#block-collection-styles). In this case, both the “ `-` ” indicator and the following [spaces](https://yaml.org/spec/1.2.2/#white-space-characters) are considered to be part of the [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) of the nested [collection](https://yaml.org/spec/1.2.2/#collections). Note that it is not possible to specify [node properties](https://yaml.org/spec/1.2.2/#node-properties) for such a [collection](https://yaml.org/spec/1.2.2/#collections).

```
[185] s-l+block-indented(n,c) ::=
    (
      s-indent(m)
      (
          ns-l-compact-sequence(n+1+m)
        | ns-l-compact-mapping(n+1+m)
      )
    )
  | s-l+block-node(n,c)
  | (
      e-node    # ""
      s-l-comments
    )
```
```
[186] ns-l-compact-sequence(n) ::=
  c-l-block-seq-entry(n)
  (
    s-indent(n)
    c-l-block-seq-entry(n)
  )*
```

**Example 8.15 Block Sequence Entry Types**

| ``` -° # Empty - \|  block node -·- one # Compact ··- two # sequence - one: two # Compact mapping ``` | ```json [ null,   "block node\n",   [ "one", "two" ],   { "one": "two" } ] ``` |
| --- | --- | --- |

**Legend:**

- `Empty`
- `[s-l+block-node(n,c)](https://yaml.org/spec/1.2.2/#rule-s-l+block-node)`
- `[ns-l-compact-sequence(n)](https://yaml.org/spec/1.2.2/#rule-ns-l-compact-sequence)`
- `[ns-l-compact-mapping(n)](https://yaml.org/spec/1.2.2/#rule-ns-l-compact-mapping)`

### 8.2.2. Block Mappings

A *Block mapping* is a series of entries, each [presenting](https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree) a [key/value pair](https://yaml.org/spec/1.2.2/#mapping).

```
[187] l+block-mapping(n) ::=
  (
    s-indent(n+1+m)
    ns-l-block-map-entry(n+1+m)
  )+
```

**Example 8.16 Block Mappings**

| ``` block mapping: ·key: value↓ ``` | ```json { "block mapping": {     "key": "value" } } ``` |
| --- | --- |

**Legend:**

- `[ns-l-block-map-entry(n)](https://yaml.org/spec/1.2.2/#rule-ns-l-block-map-entry)`
- `auto-detected [s-indent(n)](https://yaml.org/spec/1.2.2/#rule-s-indent)`

If the “`?`” indicator is specified, the optional value node must be specified on a separate line, denoted by the “`:`” indicator. Note that YAML allows here the same [compact in-line notation](https://yaml.org/spec/1.2.2/#example-block-sequence) described above for [block sequence](https://yaml.org/spec/1.2.2/#block-sequences) entries.

```
[188] ns-l-block-map-entry(n) ::=
    c-l-block-map-explicit-entry(n)
  | ns-l-block-map-implicit-entry(n)
```
```
[189] c-l-block-map-explicit-entry(n) ::=
  c-l-block-map-explicit-key(n)
  (
      l-block-map-explicit-value(n)
    | e-node                        # ""
  )
```
```
[190] c-l-block-map-explicit-key(n) ::=
  c-mapping-key                     # '?' (not followed by non-ws char)
  s-l+block-indented(n,BLOCK-OUT)
```
```
[191] l-block-map-explicit-value(n) ::=
  s-indent(n)
  c-mapping-value                   # ':' (not followed by non-ws char)
  s-l+block-indented(n,BLOCK-OUT)
```

**Example 8.17 Explicit Block Mapping Entries**

| ``` ? explicit key # Empty value↓° ? \|   block key↓ :·- one # Explicit compact ··- two # block value↓ ``` | ```json { "explicit key": null,   "block key\n": [     "one",     "two" ] } ``` |
| --- | --- | --- |

**Legend:**

- `[c-l-block-map-explicit-key(n)](https://yaml.org/spec/1.2.2/#rule-c-l-block-map-explicit-key)`
- `[l-block-map-explicit-value(n)](https://yaml.org/spec/1.2.2/#rule-l-block-map-explicit-value)`
- `[e-node](https://yaml.org/spec/1.2.2/#rule-e-node)`

If the “`?`” indicator is omitted, [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) needs to see past the [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry), in the same way as in the [single key/value pair](https://yaml.org/spec/1.2.2/#mapping) [flow mapping](https://yaml.org/spec/1.2.2/#flow-mappings). Hence, such [keys](https://yaml.org/spec/1.2.2/#nodes) are subject to the same restrictions; they are limited to a single line and must not span more than 1024 Unicode characters.

```
[192] ns-l-block-map-implicit-entry(n) ::=
  (
      ns-s-block-map-implicit-key
    | e-node    # ""
  )
  c-l-block-map-implicit-value(n)
```
```
[193] ns-s-block-map-implicit-key ::=
    c-s-implicit-json-key(BLOCK-KEY)
  | ns-s-implicit-yaml-key(BLOCK-KEY)
```

In this case, the [value](https://yaml.org/spec/1.2.2/#nodes) may be specified on the same line as the [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry). Note however that in block mappings the [value](https://yaml.org/spec/1.2.2/#nodes) must never be adjacent to the “`:`”, as this greatly reduces readability and is not required for [JSON compatibility](https://yaml.org/spec/1.2.2/#yaml-directives) (unlike the case in [flow mappings](https://yaml.org/spec/1.2.2/#flow-mappings)).

There is no compact notation for in-line [values](https://yaml.org/spec/1.2.2/#nodes). Also, while both the [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry) and the [value](https://yaml.org/spec/1.2.2/#nodes) following it may be empty, the “`:`” indicator is mandatory. This prevents a potential ambiguity with multi-line [plain scalars](https://yaml.org/spec/1.2.2/#plain-style).

```
[194] c-l-block-map-implicit-value(n) ::=
  c-mapping-value           # ':' (not followed by non-ws char)
  (
      s-l+block-node(n,BLOCK-OUT)
    | (
        e-node    # ""
        s-l-comments
      )
  )
```

**Example 8.18 Implicit Block Mapping Entries**

| ``` plain key: in-line value °:° # Both empty "quoted key": - entry ``` | ```json { "plain key": "in-line value",   null: null,   "quoted key": [ "entry" ] } ``` |
| --- | --- |

**Legend:**

- `[ns-s-block-map-implicit-key](https://yaml.org/spec/1.2.2/#rule-ns-s-block-map-implicit-key)`
- `[c-l-block-map-implicit-value(n)](https://yaml.org/spec/1.2.2/#rule-c-l-block-map-implicit-value)`

A [compact in-line notation](https://yaml.org/spec/1.2.2/#example-block-sequence) is also available. This compact notation may be nested inside [block sequences](https://yaml.org/spec/1.2.2/#block-sequences) and explicit block mapping entries. Note that it is not possible to specify [node properties](https://yaml.org/spec/1.2.2/#node-properties) for such a nested mapping.

```
[195] ns-l-compact-mapping(n) ::=
  ns-l-block-map-entry(n)
  (
    s-indent(n)
    ns-l-block-map-entry(n)
  )*
```

**Example 8.19 Compact Block Mappings**

| ``` - sun: yellow↓ - ? earth: blue↓   : moon: white↓ ``` | ```json [ { "sun": "yellow" },   { { "earth": "blue" }:       { "moon": "white" } } ] ``` |
| --- | --- |

**Legend:**

- `[ns-l-compact-mapping(n)](https://yaml.org/spec/1.2.2/#rule-ns-l-compact-mapping)`

### 8.2.3. Block Nodes

YAML allows [flow nodes](https://yaml.org/spec/1.2.2/#flow-nodes) to be embedded inside [block collections](https://yaml.org/spec/1.2.2/#block-collection-styles) (but not vice-versa).[Flow nodes](https://yaml.org/spec/1.2.2/#flow-nodes) must be [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) by at least one more [space](https://yaml.org/spec/1.2.2/#white-space-characters) than the parent [block collection](https://yaml.org/spec/1.2.2/#block-collection-styles). Note that [flow nodes](https://yaml.org/spec/1.2.2/#flow-nodes) may begin on a following line.

It is at this point that [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) needs to distinguish between a [plain scalar](https://yaml.org/spec/1.2.2/#plain-style) and an [implicit key](https://yaml.org/spec/1.2.2/#example-single-pair-explicit-entry) starting a nested [block mapping](https://yaml.org/spec/1.2.2/#block-mappings).

```
[196] s-l+block-node(n,c) ::=
    s-l+block-in-block(n,c)
  | s-l+flow-in-block(n)
```
```
[197] s-l+flow-in-block(n) ::=
  s-separate(n+1,FLOW-OUT)
  ns-flow-node(n+1,FLOW-OUT)
  s-l-comments
```

**Example 8.20 Block Node Types**

| ``` -↓ ··"flow in block"↓ -·>  Block scalar↓ -·!!map # Block collection   foo : bar↓ ``` | ```json [ "flow in block",   "Block scalar\n",   { "foo": "bar" } ] ``` |
| --- | --- |

**Legend:**

- `[s-l+flow-in-block(n)](https://yaml.org/spec/1.2.2/#rule-s-l+flow-in-block)`
- `[s-l+block-in-block(n,c)](https://yaml.org/spec/1.2.2/#rule-s-l+block-in-block)`

The block [node’s properties](https://yaml.org/spec/1.2.2/#node-properties) may span across several lines. In this case, they must be [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) by at least one more [space](https://yaml.org/spec/1.2.2/#white-space-characters) than the [block collection](https://yaml.org/spec/1.2.2/#block-collection-styles), regardless of the [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces) of the [block collection](https://yaml.org/spec/1.2.2/#block-collection-styles) entries.

```
[198] s-l+block-in-block(n,c) ::=
    s-l+block-scalar(n,c)
  | s-l+block-collection(n,c)
```
```
[199] s-l+block-scalar(n,c) ::=
  s-separate(n+1,c)
  (
    c-ns-properties(n+1,c)
    s-separate(n+1,c)
  )?
  (
      c-l+literal(n)
    | c-l+folded(n)
  )
```

**Example 8.21 Block Scalar Nodes**

| ``` literal: \|2 ··value folded:↓ ···!foo ··>1 ·value ``` | ```json { "literal": "value",   "folded": !<!foo> "value" } ``` |
| --- | --- | --- |

**Legend:**

- `[c-l+literal(n)](https://yaml.org/spec/1.2.2/#rule-c-l+literal)`
- `[c-l+folded(n)](https://yaml.org/spec/1.2.2/#rule-c-l+folded)`

Since people perceive the “ `-` ” indicator as [indentation](https://yaml.org/spec/1.2.2/#indentation-spaces), nested [block sequences](https://yaml.org/spec/1.2.2/#block-sequences) may be [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) by one less [space](https://yaml.org/spec/1.2.2/#white-space-characters) to compensate, except, of course, if nested inside another [block sequence](https://yaml.org/spec/1.2.2/#block-sequences) (\[`BLOCK-OUT` context\] versus \[`BLOCK-IN` context\]).

```
[200] s-l+block-collection(n,c) ::=
  (
    s-separate(n+1,c)
    c-ns-properties(n+1,c)
  )?
  s-l-comments
  (
      seq-space(n,c)
    | l+block-mapping(n)
  )
```
```
[201] seq-space(n,BLOCK-OUT) ::= l+block-sequence(n-1)
    seq-space(n,BLOCK-IN)  ::= l+block-sequence(n)
```

**Example 8.22 Block Collection Nodes**

| ``` sequence: !!seq - entry - !!seq  - nested mapping: !!map  foo: bar ``` | ```json { "sequence": [     "entry",     [ "nested" ] ],   "mapping": { "foo": "bar" } } ``` |
| --- | --- |

**Legend:**

- `[s-l+block-collection(n,c)](https://yaml.org/spec/1.2.2/#rule-s-l+block-collection)`
- `[l+block-sequence(n)](https://yaml.org/spec/1.2.2/#rule-l+block-sequence)`
- `[l+block-mapping(n)](https://yaml.org/spec/1.2.2/#rule-l+block-mapping)`

## Chapter 9. Document Stream Productions

## 9.1. Documents

A YAML character [stream](https://yaml.org/spec/1.2.2/#streams) may contain several *documents*. Each document is completely independent from the rest.

### 9.1.1. Document Prefix

A document may be preceded by a *prefix* specifying the [character encoding](https://yaml.org/spec/1.2.2/#character-encodings) and optional [comment](https://yaml.org/spec/1.2.2/#comments) lines. Note that all [documents](https://yaml.org/spec/1.2.2/#documents) in a stream must use the same [character encoding](https://yaml.org/spec/1.2.2/#character-encodings). However it is valid to re-specify the [encoding](https://yaml.org/spec/1.2.2/#character-encodings) using a [byte order mark](https://yaml.org/spec/1.2.2/#character-encodings) for each [document](https://yaml.org/spec/1.2.2/#documents) in the stream.

The existence of the optional prefix does not necessarily indicate the existence of an actual [document](https://yaml.org/spec/1.2.2/#documents).

```
[202] l-document-prefix ::=
  c-byte-order-mark?
  l-comment*
```

**Example 9.1 Document Prefix**

| ``` ⇔# Comment # lines Document ``` | ```json "Document" ``` |
| --- | --- |

**Legend:**

- `[l-document-prefix](https://yaml.org/spec/1.2.2/#rule-l-document-prefix)`

### 9.1.2. Document Markers

Using [directives](https://yaml.org/spec/1.2.2/#directives) creates a potential ambiguity. It is valid to have a “ `%` ” character at the start of a line (e.g. as the first character of the second line of a [plain scalar](https://yaml.org/spec/1.2.2/#plain-style)). How, then, to distinguish between an actual [directive](https://yaml.org/spec/1.2.2/#directives) and a [content](https://yaml.org/spec/1.2.2/#nodes) line that happens to start with a “ `%` ” character?

The solution is the use of two special *marker* lines to control the processing of [directives](https://yaml.org/spec/1.2.2/#directives), one at the start of a [document](https://yaml.org/spec/1.2.2/#documents) and one at the end.

At the start of a [document](https://yaml.org/spec/1.2.2/#documents), lines beginning with a “ `%` ” character are assumed to be [directives](https://yaml.org/spec/1.2.2/#directives). The (possibly empty) list of [directives](https://yaml.org/spec/1.2.2/#directives) is terminated by a *directives end marker* line. Lines following this marker can safely use “ `%` ” as the first character.

At the end of a [document](https://yaml.org/spec/1.2.2/#documents), a *document end marker* line is used to signal the [parser](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) to begin scanning for [directives](https://yaml.org/spec/1.2.2/#directives) again.

The existence of this optional *document suffix* does not necessarily indicate the existence of an actual following [document](https://yaml.org/spec/1.2.2/#documents).

Obviously, the actual [content](https://yaml.org/spec/1.2.2/#nodes) lines are therefore forbidden to begin with either of these markers.

```
[203] c-directives-end ::= "---"
```
```
[204] c-document-end ::=
  "..."    # (not followed by non-ws char)
```
```
[205] l-document-suffix ::=
  c-document-end
  s-l-comments
```
```
[206] c-forbidden ::=
  <start-of-line>
  (
      c-directives-end
    | c-document-end
  )
  (
      b-char
    | s-white
    | <end-of-input>
  )
```

**Example 9.2 Document Markers**

| ``` %YAML 1.2 --- Document ... # Suffix ``` | ```json "Document" ``` |
| --- | --- |

**Legend:**

- `[c-directives-end](https://yaml.org/spec/1.2.2/#rule-c-directives-end)`
- `[l-document-suffix](https://yaml.org/spec/1.2.2/#rule-l-document-suffix)`
- `[c-document-end](https://yaml.org/spec/1.2.2/#rule-c-document-end)`

### 9.1.3. Bare Documents

A *bare document* does not begin with any [directives](https://yaml.org/spec/1.2.2/#directives) or [marker](https://yaml.org/spec/1.2.2/#document-markers) lines. Such documents are very “clean” as they contain nothing other than the [content](https://yaml.org/spec/1.2.2/#nodes). In this case, the first non-comment line may not start with a “ `%` ” first character.

Document [nodes](https://yaml.org/spec/1.2.2/#nodes) are [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) as if they have a parent [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) at -1 [spaces](https://yaml.org/spec/1.2.2/#white-space-characters). Since a [node](https://yaml.org/spec/1.2.2/#nodes) must be more [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) than its parent [node](https://yaml.org/spec/1.2.2/#nodes), this allows the document’s [node](https://yaml.org/spec/1.2.2/#nodes) to be [indented](https://yaml.org/spec/1.2.2/#indentation-spaces) at zero or more [spaces](https://yaml.org/spec/1.2.2/#white-space-characters).

```
[207] l-bare-document ::=
  s-l+block-node(-1,BLOCK-IN)
  /* Excluding c-forbidden content */
```

**Example 9.3 Bare Documents**

| ``` Bare document ... # No document ... \| %!PS-Adobe-2.0 # Not the first line ``` | ```json "Bare document" --- "%!PS-Adobe-2.0\n" ``` |
| --- | --- | --- |

**Legend:**

- `[l-bare-document](https://yaml.org/spec/1.2.2/#rule-l-bare-document)`

### 9.1.4. Explicit Documents

An *explicit document* begins with an explicit [directives end marker](https://yaml.org/spec/1.2.2/#document-markers) line but no [directives](https://yaml.org/spec/1.2.2/#directives). Since the existence of the [document](https://yaml.org/spec/1.2.2/#documents) is indicated by this [marker](https://yaml.org/spec/1.2.2/#document-markers), the [document](https://yaml.org/spec/1.2.2/#documents) itself may be [completely empty](https://yaml.org/spec/1.2.2/#example-empty-content).

```
[208] l-explicit-document ::=
  c-directives-end
  (
      l-bare-document
    | (
        e-node    # ""
        s-l-comments
      )
  )
```

**Example 9.4 Explicit Documents**

| ``` --- { matches % : 20 } ... --- # Empty ... ``` | ```json { "matches %": 20 } --- null ``` |
| --- | --- |

**Legend:**

- `[l-explicit-document](https://yaml.org/spec/1.2.2/#rule-l-explicit-document)`

### 9.1.5. Directives Documents

A *directives document* begins with some [directives](https://yaml.org/spec/1.2.2/#directives) followed by an explicit [directives end marker](https://yaml.org/spec/1.2.2/#document-markers) line.

```
[209] l-directive-document ::=
  l-directive+
  l-explicit-document
```

**Example 9.5 Directives Documents**

| ``` %YAML 1.2 --- \| %!PS-Adobe-2.0 ... %YAML 1.2 --- # Empty ... ``` | ```json "%!PS-Adobe-2.0\n" --- null ``` |
| --- | --- | --- |

**Legend:**

- `[l-explicit-document](https://yaml.org/spec/1.2.2/#rule-l-explicit-document)`

## 9.2. Streams

A YAML *stream* consists of zero or more [documents](https://yaml.org/spec/1.2.2/#documents). Subsequent [documents](https://yaml.org/spec/1.2.2/#documents) require some sort of separation [marker](https://yaml.org/spec/1.2.2/#document-markers) line. If a [document](https://yaml.org/spec/1.2.2/#documents) is not terminated by a [document end marker](https://yaml.org/spec/1.2.2/#document-markers) line, then the following [document](https://yaml.org/spec/1.2.2/#documents) must begin with a [directives end marker](https://yaml.org/spec/1.2.2/#document-markers) line.

```
[210] l-any-document ::=
    l-directive-document
  | l-explicit-document
  | l-bare-document
```
```
[211] l-yaml-stream ::=
  l-document-prefix*
  l-any-document?
  (
      (
        l-document-suffix+
        l-document-prefix*
        l-any-document?
      )
    | c-byte-order-mark
    | l-comment
    | l-explicit-document
  )*
```

**Example 9.6 Stream**

| ``` Document --- # Empty ... %YAML 1.2 --- matches %: 20 ``` | ```json "Document" --- null --- { "matches %": 20 } ``` |
| --- | --- |

**Legend:**

- `[l-any-document](https://yaml.org/spec/1.2.2/#rule-l-any-document)`
- `[l-document-suffix](https://yaml.org/spec/1.2.2/#rule-l-document-suffix)`
- `[l-explicit-document](https://yaml.org/spec/1.2.2/#rule-l-explicit-document)`

A sequence of bytes is a *well-formed stream* if, taken as a whole, it complies with the above `l-yaml-stream` production.

A YAML *schema* is a combination of a set of [tags](https://yaml.org/spec/1.2.2/#tags) and a mechanism for [resolving](https://yaml.org/spec/1.2.2/#resolved-tags) [non-specific tags](https://yaml.org/spec/1.2.2/#resolved-tags).

## 10.1. Failsafe Schema

The *failsafe schema* is guaranteed to work with any YAML [document](https://yaml.org/spec/1.2.2/#documents). It is therefore the recommended [schema](https://yaml.org/spec/1.2.2/#recommended-schemas) for generic YAML tools. A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should therefore support this [schema](https://yaml.org/spec/1.2.2/#recommended-schemas), at least as an option.

### 10.1.1. Tags

#### 10.1.1.1. Generic Mapping

URI

`tag:yaml.org,2002:map`

Kind

[Mapping](https://yaml.org/spec/1.2.2/#mapping).

Definition

[Represents](https://yaml.org/spec/1.2.2/#representation-graph) an associative container, where each [key](https://yaml.org/spec/1.2.2/#nodes) is unique in the association and mapped to exactly one [value](https://yaml.org/spec/1.2.2/#nodes). YAML places no restrictions on the type of [keys](https://yaml.org/spec/1.2.2/#nodes); in particular, they are not restricted to being [scalars](https://yaml.org/spec/1.2.2/#scalars). Example [bindings](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) to [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) types include Perl’s hash, Python’s dictionary and Java’s Hashtable.

**Example 10.1 `!!map` Examples**

```
Block style: !!map
  Clark : Evans
  Ingy  : döt Net
  Oren  : Ben-Kiki

Flow style: !!map { Clark: Evans, Ingy: döt Net, Oren: Ben-Kiki }
```

#### 10.1.1.2. Generic Sequence

URI

`tag:yaml.org,2002:seq`

Kind

[Sequence](https://yaml.org/spec/1.2.2/#sequence).

Definition

[Represents](https://yaml.org/spec/1.2.2/#representation-graph) a collection indexed by sequential integers starting with zero. Example [bindings](https://yaml.org/spec/1.2.2/#constructing-native-data-structures) to [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) types include Perl’s array, Python’s list or tuple and Java’s array or Vector.

**Example 10.2 `!!seq` Examples**

```
Block style: !!seq
- Clark Evans
- Ingy döt Net
- Oren Ben-Kiki

Flow style: !!seq [ Clark Evans, Ingy döt Net, Oren Ben-Kiki ]
```

#### 10.1.1.3. Generic String

URI

`tag:yaml.org,2002:str`

Kind

[Scalar](https://yaml.org/spec/1.2.2/#scalar).

Definition

[Represents](https://yaml.org/spec/1.2.2/#representation-graph) a Unicode string, a sequence of zero or more Unicode characters. This type is usually [bound](https://yaml.org/spec/1.2.2/#representing-native-data-structures) to the [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) language’s string type or, for languages lacking one (such as C), to a character array.

Canonical Form:

The obvious.

**Example 10.3 `!!str` Examples**

```
Block style: !!str |-
  String: just a theory.

Flow style: !!str "String: just a theory."
```

### 10.1.2. Tag Resolution

All [nodes](https://yaml.org/spec/1.2.2/#nodes) with the “`!`” non-specific tag are [resolved](https://yaml.org/spec/1.2.2/#resolved-tags), by the standard [convention](https://yaml.org/spec/1.2.2/#resolved-tags), to “ `tag:yaml.org,2002:seq` ”, “ `tag:yaml.org,2002:map` ” or “ `tag:yaml.org,2002:str` ”, according to their [kind](https://yaml.org/spec/1.2.2/#nodes).

All [nodes](https://yaml.org/spec/1.2.2/#nodes) with the “`?`” non-specific tag are left [unresolved](https://yaml.org/spec/1.2.2/#resolved-tags). This constrains the [application](https://yaml.org/spec/1.2.2/#processes-and-models) to deal with a [partial representation](https://yaml.org/spec/1.2.2/#loading-failure-points).

## 10.2. JSON Schema

The *JSON schema* is the lowest common denominator of most modern computer languages and allows [parsing](https://yaml.org/spec/1.2.2/#parsing-the-presentation-stream) JSON files. A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should therefore support this [schema](https://yaml.org/spec/1.2.2/#recommended-schemas), at least as an option. It is also strongly recommended that other [schemas](https://yaml.org/spec/1.2.2/#recommended-schemas) should be based on it.

### 10.2.1. Tags

The JSON [schema](https://yaml.org/spec/1.2.2/#recommended-schemas) uses the following [tags](https://yaml.org/spec/1.2.2/#tags) in addition to those defined by the [failsafe](https://yaml.org/spec/1.2.2/#failsafe-schema) schema:

#### 10.2.1.1. Null

URI

`tag:yaml.org,2002:null`

Kind

[Scalar](https://yaml.org/spec/1.2.2/#scalar).

Definition

[Represents](https://yaml.org/spec/1.2.2/#representation-graph) the lack of a value. This is typically [bound](https://yaml.org/spec/1.2.2/#representing-native-data-structures) to a [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) null-like value (e.g., `undef` in Perl,`None` in Python). Note that a null is different from an empty string. Also, a [mapping](https://yaml.org/spec/1.2.2/#mapping) entry with some [key](https://yaml.org/spec/1.2.2/#nodes) and a null [value](https://yaml.org/spec/1.2.2/#nodes) is valid and different from not having that [key](https://yaml.org/spec/1.2.2/#nodes) in the [mapping](https://yaml.org/spec/1.2.2/#mapping).

Canonical Form

`null`.

**Example 10.4 `!!null` Examples**

```
!!null null: value for null key
key with null value: !!null null
```

#### 10.2.1.2. Boolean

URI

`tag:yaml.org,2002:bool`

Kind

[Scalar](https://yaml.org/spec/1.2.2/#scalar).

Definition

[Represents](https://yaml.org/spec/1.2.2/#representation-graph) a true/false value. In languages without a [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) Boolean type (such as C), they are usually [bound](https://yaml.org/spec/1.2.2/#representing-native-data-structures) to a native integer type, using one for true and zero for false.

Canonical Form

Either `true` or `false`.

**Example 10.5 `!!bool` Examples**

```
YAML is a superset of JSON: !!bool true
Pluto is a planet: !!bool false
```

#### 10.2.1.3. Integer

URI

`tag:yaml.org,2002:int`

Kind

[Scalar](https://yaml.org/spec/1.2.2/#scalar).

Definition

[Represents](https://yaml.org/spec/1.2.2/#representation-graph) arbitrary sized finite mathematical integers. Scalars of this type should be [bound](https://yaml.org/spec/1.2.2/#representing-native-data-structures) to a [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) integer data type, if possible.

Some languages (such as Perl) provide only a “number” type that allows for both integer and floating-point values. A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) may use such a type for integers as long as they round-trip properly.

In some languages (such as C), an integer may overflow the [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) type’s storage capability. A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) may reject such a value as an error, truncate it with a warning or find some other manner to round-trip it. In general, integers representable using 32 binary digits should safely round-trip through most systems.

Canonical Form

Decimal integer notation, with a leading “ `-` ” character for negative values, matching the regular expression `0 | -? [1-9] [0-9]*`

**Example 10.6 `!!int` Examples**

```
negative: !!int -12
zero: !!int 0
positive: !!int 34
```

#### 10.2.1.4. Floating Point

URI

`tag:yaml.org,2002:float`

Kind

[Scalar](https://yaml.org/spec/1.2.2/#scalar).

Definition

[Represents](https://yaml.org/spec/1.2.2/#representation-graph) an approximation to real numbers, including three special values (positive and negative infinity and “not a number”).

Some languages (such as Perl) provide only a “number” type that allows for both integer and floating-point values. A YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) may use such a type for floating-point numbers, as long as they round-trip properly.

Not all floating-point values can be stored exactly in any given [native](https://yaml.org/spec/1.2.2/#representing-native-data-structures) type. Hence a float value may change by “a small amount” when round-tripped. The supported range and accuracy depends on the implementation, though 32 bit IEEE floats should be safe. Since YAML does not specify a particular accuracy, using floating-point [mapping keys](https://yaml.org/spec/1.2.2/#nodes) requires great care and is not recommended.

Canonical Form

Either `0`, `.inf`, `-.inf`, `.nan` or scientific notation matching the regular expression  
`-? [1-9] ( \. [0-9]* [1-9] )? ( e [-+] [1-9] [0-9]* )?`.

**Example 10.7 `!!float` Examples**

```
negative: !!float -1
zero: !!float 0
positive: !!float 2.3e4
infinity: !!float .inf
not a number: !!float .nan
```

### 10.2.2. Tag Resolution

The [JSON schema](https://yaml.org/spec/1.2.2/#json-schema) [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution) is an extension of the [failsafe schema](https://yaml.org/spec/1.2.2/#failsafe-schema) [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution).

All [nodes](https://yaml.org/spec/1.2.2/#nodes) with the “`!`” non-specific tag are [resolved](https://yaml.org/spec/1.2.2/#resolved-tags), by the standard [convention](https://yaml.org/spec/1.2.2/#resolved-tags), to “ `tag:yaml.org,2002:seq` ”, “ `tag:yaml.org,2002:map` ” or “ `tag:yaml.org,2002:str` ”, according to their [kind](https://yaml.org/spec/1.2.2/#nodes).

[Collections](https://yaml.org/spec/1.2.2/#collections) with the “`?`” non-specific tag (that is, [untagged](https://yaml.org/spec/1.2.2/#resolved-tags) [collections](https://yaml.org/spec/1.2.2/#collections)) are [resolved](https://yaml.org/spec/1.2.2/#resolved-tags) to “ `tag:yaml.org,2002:seq` ” or “ `tag:yaml.org,2002:map` ” according to their [kind](https://yaml.org/spec/1.2.2/#nodes).

[Scalars](https://yaml.org/spec/1.2.2/#scalars) with the “`?`” non-specific tag (that is, [plain scalars](https://yaml.org/spec/1.2.2/#plain-style)) are matched with a list of regular expressions (first match wins, e.g. `0` is resolved as `!!int`). In principle, JSON files should not contain any [scalars](https://yaml.org/spec/1.2.2/#scalars) that do not match at least one of these. Hence the YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should consider them to be an error.

| Regular expression | Resolved to tag |
| --- | --- |
| `null` | tag:yaml.org,2002:null |
| `true \| false` | tag:yaml.org,2002:bool |
| `-? ( 0 \| [1-9] [0-9]* )` | tag:yaml.org,2002:int |
| `-? ( 0 \| [1-9] [0-9]* ) ( \. [0-9]* )? ( [eE] [-+]? [0-9]+ )?` | tag:yaml.org,2002:float |
| `*` | Error |

> Note: The regular expression for `float` does not exactly match the one in the JSON specification, where at least one digit is required after the dot: `( \.  [0-9]+ )`. The YAML 1.2 specification intended to match JSON behavior, but this cannot be addressed in the 1.2.2 specification.

**Example 10.8 JSON Tag Resolution**

| ``` A null: null Booleans: [ true, false ] Integers: [ 0, -0, 3, -19 ] Floats: [ 0., -0.0, 12e03, -2E+05 ] Invalid: [ True, Null,   0o7, 0x3A, +12.3 ] ``` | ```json { "A null": null,   "Booleans": [ true, false ],   "Integers": [ 0, 0, 3, -19 ],   "Floats": [ 0.0, -0.0, 12000, -200000 ],   "Invalid": [ "True", "Null",     "0o7", "0x3A", "+12.3" ] } ``` |
| --- | --- |

## 10.3. Core Schema

The *Core schema* is an extension of the [JSON schema](https://yaml.org/spec/1.2.2/#json-schema), allowing for more human-readable [presentation](https://yaml.org/spec/1.2.2/#presentation-stream) of the same types. This is the recommended default [schema](https://yaml.org/spec/1.2.2/#recommended-schemas) that YAML [processor](https://yaml.org/spec/1.2.2/#processes-and-models) should use unless instructed otherwise. It is also strongly recommended that other [schemas](https://yaml.org/spec/1.2.2/#recommended-schemas) should be based on it.

### 10.3.1. Tags

The core [schema](https://yaml.org/spec/1.2.2/#recommended-schemas) uses the same [tags](https://yaml.org/spec/1.2.2/#tags) as the [JSON schema](https://yaml.org/spec/1.2.2/#json-schema).

### 10.3.2. Tag Resolution

The [core schema](https://yaml.org/spec/1.2.2/#core-schema) [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution) is an extension of the [JSON schema](https://yaml.org/spec/1.2.2/#json-schema) [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution).

All [nodes](https://yaml.org/spec/1.2.2/#nodes) with the “`!`” non-specific tag are [resolved](https://yaml.org/spec/1.2.2/#resolved-tags), by the standard [convention](https://yaml.org/spec/1.2.2/#resolved-tags), to “ `tag:yaml.org,2002:seq` ”, “ `tag:yaml.org,2002:map` ” or “ `tag:yaml.org,2002:str` ”, according to their [kind](https://yaml.org/spec/1.2.2/#nodes).

[Collections](https://yaml.org/spec/1.2.2/#collections) with the “`?`” non-specific tag (that is, [untagged](https://yaml.org/spec/1.2.2/#resolved-tags) [collections](https://yaml.org/spec/1.2.2/#collections)) are [resolved](https://yaml.org/spec/1.2.2/#resolved-tags) to “ `tag:yaml.org,2002:seq` ” or “ `tag:yaml.org,2002:map` ” according to their [kind](https://yaml.org/spec/1.2.2/#nodes).

[Scalars](https://yaml.org/spec/1.2.2/#scalars) with the “`?`” non-specific tag (that is, [plain scalars](https://yaml.org/spec/1.2.2/#plain-style)) are matched with an extended list of regular expressions. However, in this case, if none of the regular expressions matches, the [scalar](https://yaml.org/spec/1.2.2/#scalar) is [resolved](https://yaml.org/spec/1.2.2/#resolved-tags) to `tag:yaml.org,2002:str` (that is, considered to be a string).

| Regular expression | Resolved to tag |
| --- | --- |
| `null \| Null \| NULL \| ~` | tag:yaml.org,2002:null |
| `/* Empty */` | tag:yaml.org,2002:null |
| `true \| True \| TRUE \| false \| False \| FALSE` | tag:yaml.org,2002:bool |
| `[-+]? [0-9]+` | tag:yaml.org,2002:int (Base 10) |
| `0o [0-7]+` | tag:yaml.org,2002:int (Base 8) |
| `0x [0-9a-fA-F]+` | tag:yaml.org,2002:int (Base 16) |
| `[-+]? ( \. [0-9]+ \| [0-9]+ ( \. [0-9]* )? ) ( [eE] [-+]? [0-9]+ )?` | tag:yaml.org,2002:float (Number) |
| `[-+]? ( \.inf \| \.Inf \| \.INF )` | tag:yaml.org,2002:float (Infinity) |
| `\.nan \| \.NaN \| \.NAN` | tag:yaml.org,2002:float (Not a number) |
| `*` | tag:yaml.org,2002:str (Default) |

**Example 10.9 Core Tag Resolution**

| ``` A null: null Also a null: # Empty Not a null: "" Booleans: [ true, True, false, FALSE ] Integers: [ 0, 0o7, 0x3A, -19 ] Floats: [   0., -0.0, .5, +12e03, -2E+05 ] Also floats: [   .inf, -.Inf, +.INF, .NAN ] ``` | ```json { "A null": null,   "Also a null": null,   "Not a null": "",   "Booleans": [ true, true, false, false ],   "Integers": [ 0, 7, 58, -19 ],   "Floats": [     0.0, -0.0, 0.5, 12000, -200000 ],   "Also floats": [     Infinity, -Infinity, Infinity, NaN ] } ``` |
| --- | --- |

## 10.4. Other Schemas

None of the above recommended [schemas](https://yaml.org/spec/1.2.2/#recommended-schemas) preclude the use of arbitrary explicit [tags](https://yaml.org/spec/1.2.2/#tags). Hence YAML [processors](https://yaml.org/spec/1.2.2/#processes-and-models) for a particular programming language typically provide some form of [local tags](https://yaml.org/spec/1.2.2/#tags) that map directly to the language’s [native data structures](https://yaml.org/spec/1.2.2/#representing-native-data-structures) (e.g., `!ruby/object:Set`).

While such [local tags](https://yaml.org/spec/1.2.2/#tags) are useful for ad hoc [applications](https://yaml.org/spec/1.2.2/#processes-and-models), they do not suffice for stable, interoperable cross- [application](https://yaml.org/spec/1.2.2/#processes-and-models) or cross-platform data exchange.

Interoperable [schemas](https://yaml.org/spec/1.2.2/#recommended-schemas) make use of [global tags](https://yaml.org/spec/1.2.2/#tags) (URIs) that [represent](https://yaml.org/spec/1.2.2/#representation-graph) the same data across different programming languages. In addition, an interoperable [schema](https://yaml.org/spec/1.2.2/#recommended-schemas) may provide additional [tag resolution](https://yaml.org/spec/1.2.2/#tag-resolution) rules. Such rules may provide additional regular expressions, as well as consider the path to the [node](https://yaml.org/spec/1.2.2/#nodes). This allows interoperable [schemas](https://yaml.org/spec/1.2.2/#recommended-schemas) to use [untagged](https://yaml.org/spec/1.2.2/#resolved-tags) [nodes](https://yaml.org/spec/1.2.2/#nodes).

It is strongly recommended that such [schemas](https://yaml.org/spec/1.2.2/#recommended-schemas) be based on the [core schema](https://yaml.org/spec/1.2.2/#core-schema) defined above.

## Reference Links

[^1]: [YAML Language Development Team](https://yaml.org/spec/1.2.2/ext/team)

[^2]: [YAML Specification on GitHub](https://github.com/yaml/yaml-spec)

[^3]: [YAML Ain’t Markup Language (YAML™) version 1.2](https://yaml.org/spec/1.2/)

[^4]: [Unicode – The World Standard for Text and Emoji](https://home.unicode.org/)

[^5]: [YAML Core Mailing List](https://sourceforge.net/projects/yaml/lists/yaml-core)

[^6]: [SML-DEV Mailing List Archive](https://github.com/yaml/sml-dev-archive)

[^7]: [Data::Denter - An (deprecated) alternative to Data::Dumper and Storable](https://metacpan.org/dist/Data-Denter/view/Denter.pod)

[^8]: [YAML Ain’t Markup Language (YAML™) version 1.1](https://yaml.org/spec/1.1/)

[^9]: [The JSON data interchange syntax](https://www.ecma-international.org/publications-and-standards/standards/ecma-404/)

[^10]: [PyYAML - YAML parser and emitter for Python](https://github.com/yaml/pyyaml)

[^11]: [LibYAML - A C library for parsing and emitting YAML](https://github.com/yaml/libyaml)

[^12]: [Request for Comments Summary](https://datatracker.ietf.org/doc/html/rfc2119)

[^13]: [directed graph](https://xlinux.nist.gov/dads/HTML/directedGraph.html)

[^14]: [The ‘tag’ URI Scheme](https://datatracker.ietf.org/doc/html/rfc4151)

[^15]: [Wikipedia - C0 and C1 control codes](https://en.wikipedia.org/wiki/C0_and_C1_control_codes)

[^16]: [Wikipedia - Universal Character Set characters #Surrogates](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates)

[^17]: [UTF-8, UTF-16, UTF-32 & BOM](https://www.unicode.org/faq/utf_bom.html)

[^18]: [Uniform Resource Identifiers (URI)](https://datatracker.ietf.org/doc/html/rfc3986)
