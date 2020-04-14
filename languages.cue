// SPDX-License-Identifier: MIT OR Apache-2.0

package lang

import (
	"list"
	"strings"
)

// Schema

Language :: {
	name: string // display name
	extensions: [...string]
	filenames: [...string]
	interpreters: [...string]

	// require at least one identifying filetype marker
	markers :: [extensions, filenames, interpreters]
	//marker_is_defined: true & or([ len(m) > 0 for m in markers ])
}

languages :: [Key=string]: Language & {
	name: *Key | string // name's value defaults to the map key.
}

// Input

languages :: {
	Abap: {
		name: "ABAP"
		extensions: ["abap"]
	}
	ActionScript: extensions: ["as"]
	Ada: extensions: ["ada", "adb", "ads", "pad"]
	Agda: extensions: ["agda"]
	Alex: extensions: ["x"]
	Arduino: {
		name: "Arduino C++"
		extensions: ["ino"]
	}
	AsciiDoc: extensions: ["adoc", "asc", "asciidoc"]
	Asn1: {
		name: "ASN.1"
		extensions: ["asn1"]
	}
	Asp: {
		name: "ASP"
		extensions: ["asa", "asp"]
	}
	AspNet: {
		name: "ASP.NET"
		extensions: ["asax", "ascx", "asmx", "aspx", "master", "sitemap", "webinfo"]
	}
	Assembly: extensions: ["asm"]
	AssemblyGAS: {
		name: "GNU Style Assembly"
		extensions: ["s"]
	}
	Autoconf: extensions: ["in"]
	AutoHotKey: extensions: ["ahk"]
	Automake: extensions: ["am"]
	Bash: {
		name: "BASH"
		extensions: ["bash"]
		interpreters: ["bash"]
	}
	Batch: extensions: ["bat", "btm", "cmd"]
	BrightScript: extensions: ["brs"]
	C: extensions: ["c", "ec", "pgc"]
	Cabal: extensions: ["cabal"]
	Cassius: extensions: ["cassius"]
	Ceylon: extensions: ["ceylon"]
	CHeader: {
		name: "C Header"
		extensions: ["h"]
	}
	Clojure: extensions: ["clj"]
	ClojureC: extensions: ["cljc"]
	ClojureScript: extensions: ["cljs"]
	CMake: {
		extensions: ["cmake"]
		filenames: ["cmakelists.txt"]
	}
	Cobol: {
		name: "COBOL"
		extensions: ["cob", "cbl", "ccp", "cobol", "cpy"]
	}
	CoffeeScript: extensions: ["coffee", "cjsx"]
	Cogent: extensions: ["cogent"]
	ColdFusion: extensions: ["cfm"]
	ColdFusionScript: {
		name: "ColdFusion CFScript"
		extensions: ["cfc"]
	}
	Coq: extensions: ["v"]
	Cpp: {
		name: "C++"
		extensions: ["cc", "cpp", "cxx", "c++", "pcc", "tpp"]
	}
	CppHeader: {
		name: "C++ Header"
		extensions: ["hh", "hpp", "hxx", "inl", "ipp"]
	}
	Crystal: extensions: ["cr"]
	CSharp: {
		name: "C#"
		extensions: ["cs"]
	}
	CShell: {
		name: "C Shell"
		extensions: ["csh"]
		interpreters: ["csh"]
	}
	Css: {
		name: "CSS"
		extensions: ["css"]
	}
	Cue: {
		name: "CUE"
		extensions: ["cue"]
	}
	D: extensions: ["d"]
	Dart: extensions: ["dart"]
	DeviceTree: {
		name: "Device Tree"
		extensions: ["dts", "dtsi"]
	}
	Dhall: extensions: ["dhall"]
	Dockerfile: {
		extensions: ["dockerfile", "dockerignore"]
		filenames: ["dockerfile"]
	}
	DotNetResource: {
		name: ".NET Resource"
		extensions: ["resx"]
	}
	DreamMaker: {
		name: "Dream Maker"
		extensions: ["dm", "dme"]
	}
	Dust: {
		name: "Dust.js"
		extensions: ["dust"]
	}
	Edn: extensions: ["edn"]
	Elisp: {
		name: "Emacs Lisp"
		extensions: ["el"]
	}
	Elixir: extensions: ["ex", "exs"]
	Elm: extensions: ["elm"]
	Elvish: {
		extensions: ["elv"]
		interpreters: ["elvish"]
	}
	EmacsDevEnv: {
		name: "Emacs Dev Env"
		extensions: ["ede"]
	}
	Emojicode: extensions: ["emojic", "🍇"]
	Erlang: extensions: ["erl", "hrl"]
	FEN: {
		name: "FEN"
		extensions: ["fen"]
	}
	Fish: {
		extensions: ["fish"]
		interpreters: ["fish"]
	}
	FlatBuffers: {
		name: "FlatBuffers Schema"
		extensions: ["fbs"]
	}
	Forth: extensions: ["4th", "forth", "fr", "frt", "fth", "f83", "fb", "fpm", "e4", "rx", "ft"]
	FortranLegacy: {
		name: "FORTRAN Legacy"
		extensions: ["f", "for", "ftn", "f77", "pfo"]
	}
	FortranModern: {
		name: "FORTRAN Modern"
		extensions: ["f03", "f08", "f90", "f95"]
	}
	FreeMarker: extensions: ["ftl", "ftlh", "ftlx"]
	FSharp: {
		name: "F#"
		extensions: ["fs", "fsi", "fsx", "fsscript"]
	}
	Fstar: {
		name: "F*"
		extensions: ["fst"]
	}
	GDB: {
		name: "GDB Script"
		extensions: ["gdb"]
	}
	GdScript: {
		name: "GDScript"
		extensions: ["gd"]
	}
	Gherkin: {
		name: "Gherkin (Cucumber)"
		extensions: ["feature"]
	}
	Glsl: {
		name: "GLSL"
		extensions: ["vert", "tesc", "tese", "geom", "frag", "comp", "glsl"]
	}
	Go: extensions: ["go"]
	Graphql: {
		name: "GraphQL"
		extensions: ["gql", "graphql"]
	}
	Groovy: extensions: ["groovy", "grt", "gtpl", "gvy"]
	Hamlet: extensions: ["hamlet"]
	Handlebars: extensions: ["hbs", "handlebars"]
	Happy: extensions: ["y", "ly"]
	Haskell: extensions: ["hs"]
	Haxe: extensions: ["hx"]
	Hcl: {
		name: "HCL"
		extensions: ["hcl", "tf", "tfvars"]
	}
	Hex: {
		name: "HEX"
		extensions: ["hex"]
	}
	Hlsl: {
		name: "HLSL"
		extensions: ["hlsl"]
	}
	HolyC: extensions: ["HC", "hc"]
	Html: {
		name: "HTML"
		extensions: ["html", "htm"]
	}
	Idris: extensions: ["idr", "lidr"]
	Ini: {
		name: "INI"
		extensions: ["ini"]
	}
	IntelHex: {
		name: "Intel HEX"
		extensions: ["ihex"]
	}
	Isabelle: extensions: ["thy"]
	Jai: {
		name: "JAI"
		extensions: ["jai"]
	}
	Java: extensions: ["java"]
	JavaScript: extensions: ["js", "mjs"]
	Json: {
		name: "JSON"
		extensions: ["json"]
	}
	Jsx: {
		name: "JSX"
		extensions: ["jsx"]
	}
	Julia: extensions: ["jl"]
	Julius: extensions: ["julius"]
	KakouneScript: {
		name: "Kakoune script"
		extensions: ["kak"]
	}
	Kotlin: extensions: ["kt", "kts"]
	Lean: extensions: ["lean", "hlean"]
	Less: {
		name: "LESS"
		extensions: ["less"]
	}
	LinkerScript: {
		name: "LD Script"
		extensions: ["lds"]
	}
	Liquid: {
		name: "Liquid"
		extensions: ["liquid"]
	}
	Lisp: extensions: ["lisp", "lsp"]
	LLVM: extensions: ["ll"]
	Logtalk: extensions: ["lgt", "logtalk"]
	Lua: extensions: ["lua"]
	Lucius: extensions: ["lucius"]
	Madlang: extensions: ["mad"]
	Makefile: {
		extensions: ["makefile", "mak", "mk"]
		filenames: ["makefile"]
	}
	Markdown: extensions: ["md", "markdown"]
	Meson: filenames: ["meson.build", "meson_options.txt"]
	Mint: extensions: ["mint"]
	ModuleDef: {
		name: "Module-Definition"
		extensions: ["def"]
	}
	MoonScript: extensions: ["moon"]
	MsBuild: {
		name: "MSBuild"
		extensions: ["csproj", "vbproj", "fsproj", "props", "targets"]
	}
	Mustache: extensions: ["mustache"]
	Nim: extensions: ["nim"]
	Nix: extensions: ["nix"]
	NotQuitePerl: {
		name: "Not Quite Perl"
		extensions: ["nqp"]
	}
	ObjectiveC: {
		name: "Objective-C"
		extensions: ["m"]
	}
	ObjectiveCpp: {
		name: "Objective-C++"
		extensions: ["mm"]
	}
	OCaml: extensions: ["ml", "mli", "re", "rei"]
	Odin: extensions: ["odin"]
	Org: extensions: ["org"]
	Oz: extensions: ["oz"]
	Pan: extensions: ["pan", "tpl"]
	Pascal: extensions: ["pas", "pp"]
	Perl: {
		extensions: ["pl", "pm"]
		interpreters: ["perl"]
	}
	Perl6: extensions: ["pl6", "pm6"]
	Pest: extensions: ["pest"]
	Php: {
		name: "PHP"
		extensions: ["php"]
	}
	Polly: extensions: ["polly"]
	Pony: extensions: ["pony"]
	PostCss: {
		name: "PostCSS"
		extensions: ["pcss", "sss"]
	}
	PowerShell: extensions: ["ps1", "psm1", "psd1", "ps1xml", "cdxml", "pssc", "psc1"]
	Processing: extensions: ["pde"]
	Prolog: extensions: ["p", "pro"]
	Protobuf: {
		name: "Protocol Buffers"
		extensions: ["proto"]
	}
	PSL: {
		name: "PSL Assertion"
		extensions: ["psl"]
	}
	PureScript: extensions: ["purs"]
	Python: {
		extensions: ["py", "pyw"]
		interpreters: ["python", "python2", "python3"]
	}
	Qcl: {
		name: "QCL"
		extensions: ["qcl"]
	}
	Qml: {
		name: "QML"
		extensions: ["qml"]
	}
	R: extensions: ["r"]
	Racket: extensions: ["rkt"]
	Rakefile: {
		extensions: ["rake"]
		filenames: ["rakefile"]
	}
	Razor: extensions: ["cshtml"]
	Renpy: {
		name: "Ren'Py"
		extensions: ["rpy"]
	}
	ReStructuredText: extensions: ["rst"]
	RON: {
		name: "Rusty Object Notation"
		extensions: ["ron"]
	}
	RPMSpecfile: {
		name: "RPM Specfile"
		extensions: ["spec"]
	}
	Ruby: {
		extensions: ["rb"]
		interpreters: ["ruby"]
	}
	RubyHtml: {
		name: "Ruby HTML"
		extensions: ["rhtml"]
	}
	Rust: extensions: ["rs"]
	Sass: extensions: ["sass", "scss"]
	Scala: extensions: ["sc", "scala"]
	Scheme: extensions: ["scm", "ss"]
	Scons: filenames: ["sconstruct", "sconscript"]
	Sh: {
		name: "Shell"
		extensions: ["sh"]
		interpreters: ["sh"]
	}
	Sml: {
		name: "Standard ML (SML)"
		extensions: ["sml"]
	}
	Solidity: {
		name: "Solidity"
		extensions: ["sol"]
	}
	SpecmanE: {
		name: "Specman e"
		extensions: ["e"]
	}
	Spice: {
		name: "Spice Netlist"
		extensions: ["ckt"]
	}
	Sql: {
		name: "SQL"
		extensions: ["sql"]
	}
	SRecode: {
		name: "SRecode Template"
		extensions: ["srt"]
	}
	Stratego: {
		name: "Stratego/XT"
		extensions: ["str"]
	}
	Svg: {
		name: "SVG"
		extensions: ["svg"]
	}
	Swift: extensions: ["swift"]
	Swig: {
		name: "SWIG"
		extensions: ["swg", "i"]
	}
	SystemVerilog: extensions: ["sv", "svh"]
	Tcl: {
		name: "TCL"
		extensions: ["tcl"]
	}
	Tex: {
		name: "TeX"
		extensions: ["tex", "sty"]
	}
	Text: {
		name: "Plain Text"
		extensions: ["text", "txt"]
	}
	Thrift: extensions: ["thrift"]
	Toml: {
		name: "TOML"
		extensions: ["toml"]
	}
	Twig: {
		name: "Twig"
		extensions: ["twig"]
	}
	TypeScript: extensions: ["ts", "tsx"]
	UnrealDeveloperMarkdown: {
		name: "Unreal Markdown"
		extensions: ["udn"]
	}
	UnrealPlugin: {
		name: "Unreal Plugin"
		extensions: ["uplugin"]
	}
	UnrealProject: {
		name: "Unreal Project"
		extensions: ["uproject"]
	}
	UnrealScript: {
		name: "Unreal Script"
		extensions: ["uc", "uci", "upkg"]
	}
	UnrealShader: {
		name: "Unreal Shader"
		extensions: ["usf"]
	}
	UnrealShaderHeader: {
		name: "Unreal Shader Header"
		extensions: ["ush"]
	}
	UrWeb: {
		name: "Ur/Web"
		extensions: ["ur", "urs"]
	}
	UrWebProject: {
		name: "Ur/Web Project"
		extensions: ["urp"]
	}
	Vala: extensions: ["vala"]
	VB6: {
		name: "VB6"
		extensions: ["frm", "bas", "cls"]
	}
	VBScript: {
		name: "VBScript"
		extensions: ["vbs"]
	}
	Velocity: {
		name: "Apache Velocity"
		extensions: ["vm"]
	}
	Verilog: extensions: ["vg", "vh"]
	VerilogArgsFile: {
		name: "Verilog Args File"
		extensions: ["irunargs", "xrunargs"]
	}
	Vhdl: {
		name: "VHDL"
		extensions: ["vhd", "vhdl"]
	}
	VimScript: {
		name: "Vim Script"
		extensions: ["vim"]
	}
	VisualBasic: {
		name: "Visual Basic"
		extensions: ["vb"]
	}
	VisualStudioProject: {
		name: "Visual Studio Project"
		extensions: ["vcproj", "vcxproj"]
	}
	VisualStudioSolution: {
		name: "Visual Studio Solution"
		extensions: ["sln"]
	}
	Vue: {
		name: "Vue"
		extensions: ["vue"]
	}
	WebAssembly: extensions: ["wat", "wast"]
	Wolfram: extensions: ["nb", "wl"]
	Xaml: {
		name: "XAML"
		extensions: ["xaml"]
	}
	XcodeConfig: {
		name: "Xcode Config"
		extensions: ["xcconfig"]
	}
	Xml: {
		name: "XML"
		extensions: ["xml"]
	}
	XSL: {
		name: "XSL"
		extensions: ["xsl", "xslt"]
	}
	Xtend: extensions: ["xtend"]
	Yaml: {
		name: "YAML"
		extensions: ["yaml", "yml"]
	}
	Zig: extensions: ["zig"]
	Zsh: {
		extensions: ["zsh"]
		interpreters: ["zsh"]
	}
}

// Constraints

keys ::         [ strings.ToLower(k) for k, v in languages ]
extensions ::   list.FlattenN([ v.extensions for k, v in languages ], -1)
filenames ::    list.FlattenN([ v.filenames for k, v in languages ], -1)
interpreters :: list.FlattenN([ v.interpreters for k, v in languages ], -1)

all_keys_sorted:         true & list.SortStrings(keys) == keys
all_extensions_unique:   true & list.UniqueItems(extensions)
all_filenames_unique:    true & list.UniqueItems(filenames)
all_interpreters_unique: true & list.UniqueItems(interpreters)

// Output

output: "languages": {
	for k, v in languages {
		"\(strings.ToLower(k))": v
	}
}
