// SPDX-License-Identifier: MIT OR Apache-2.0

package lang

import (
	"list"
	"strings"
)

// Schema

Language :: {
	name: string // display name
	globs: [...string]
	interpreters: [...string]

	// require at least one identifying filetype marker
	markers :: [globs, interpreters]
	//marker_is_defined: true & or([ len(m) > 0 for m in markers ])
}

languages :: [Key=string]: Language & {
	name: *Key | string // name's value defaults to the map key.
}

// Input

languages :: {
	Abap: {
		name: "ABAP"
		globs: ["*.abap"]
	}
	ActionScript: globs: ["*.as"]
	Ada: globs: ["*.ada", "*.adb", "*.ads", "*.pad"]
	Agda: globs: ["*.agda"]
	Alex: globs: ["*.x"]
	Arduino: {
		name: "Arduino C++"
		globs: ["*.ino"]
	}
	AsciiDoc: globs: ["*.adoc", "*.asc", "*.asciidoc"]
	Asn1: {
		name: "ASN.1"
		globs: ["*.asn1"]
	}
	Asp: {
		name: "ASP"
		globs: ["*.asa", "*.asp"]
	}
	AspNet: {
		name: "ASP.NET"
		globs: ["*.asax", "*.ascx", "*.asmx", "*.aspx", "*.master", "*.sitemap", "*.webinfo"]
	}
	Assembly: globs: ["*.asm"]
	AssemblyGAS: {
		name: "GNU Style Assembly"
		globs: ["*.s"]
	}
	Autoconf: globs: ["*.in"]
	AutoHotKey: globs: ["*.ahk"]
	Automake: globs: ["*.am"]
	Bash: {
		name: "BASH"
		globs: ["*.bash"]
		interpreters: ["bash"]
	}
	Batch: globs: ["*.bat", "*.btm", "*.cmd"]
	BrightScript: globs: ["*.brs"]
	C: globs: ["*.c", "*.ec", "*.pgc"]
	Cabal: globs: ["*.cabal"]
	Cassius: globs: ["*.cassius"]
	Ceylon: globs: ["*.ceylon"]
	CHeader: {
		name: "C Header"
		globs: ["*.h"]
	}
	Clojure: globs: ["*.clj"]
	ClojureC: globs: ["*.cljc"]
	ClojureScript: globs: ["*.cljs"]
	CMake: {
		globs: ["*.cmake", "cmakelists.txt"]
	}
	Cobol: {
		name: "COBOL"
		globs: ["*.cob", "*.cbl", "*.ccp", "*.cobol", "*.cpy"]
	}
	CoffeeScript: globs: ["*.coffee", "*.cjsx"]
	Cogent: globs: ["*.cogent"]
	ColdFusion: globs: ["*.cfm"]
	ColdFusionScript: {
		name: "ColdFusion CFScript"
		globs: ["*.cfc"]
	}
	Coq: globs: ["*.v"]
	Cpp: {
		name: "C++"
		globs: ["*.cc", "*.cpp", "*.cxx", "*.c++", "*.pcc", "*.tpp"]
	}
	CppHeader: {
		name: "C++ Header"
		globs: ["*.hh", "*.hpp", "*.hxx", "*.inl", "*.ipp"]
	}
	Crystal: globs: ["*.cr"]
	CSharp: {
		name: "C#"
		globs: ["*.cs"]
	}
	CShell: {
		name: "C Shell"
		globs: ["*.csh"]
		interpreters: ["csh"]
	}
	Css: {
		name: "CSS"
		globs: ["*.css"]
	}
	Cue: {
		name: "CUE"
		globs: ["*.cue"]
	}
	D: globs: ["*.d"]
	Dart: globs: ["*.dart"]
	DeviceTree: {
		name: "Device Tree"
		globs: ["*.dts", "*.dtsi"]
	}
	Dhall: globs: ["*.dhall"]
	Dockerfile: {
		globs: ["*.dockerfile", "*.dockerignore", "dockerfile"]
	}
	DotNetResource: {
		name: ".NET Resource"
		globs: ["*.resx"]
	}
	DreamMaker: {
		name: "Dream Maker"
		globs: ["*.dm", "*.dme"]
	}
	Dust: {
		name: "Dust.js"
		globs: ["*.dust"]
	}
	Edn: globs: ["*.edn"]
	Elisp: {
		name: "Emacs Lisp"
		globs: ["*.el"]
	}
	Elixir: globs: ["*.ex", "*.exs"]
	Elm: globs: ["*.elm"]
	Elvish: {
		globs: ["*.elv"]
		interpreters: ["elvish"]
	}
	EmacsDevEnv: {
		name: "Emacs Dev Env"
		globs: ["*.ede"]
	}
	Emojicode: globs: ["*.emojic", "🍇"]
	Erlang: globs: ["*.erl", "*.hrl"]
	FEN: {
		name: "FEN"
		globs: ["*.fen"]
	}
	Fish: {
		globs: ["*.fish"]
		interpreters: ["fish"]
	}
	FlatBuffers: {
		name: "FlatBuffers Schema"
		globs: ["*.fbs"]
	}
	Forth: globs: ["*.4th", "*.forth", "*.fr", "*.frt", "*.fth", "*.f83", "*.fb", "*.fpm", "*.e4", "*.rx", "*.ft"]
	FortranLegacy: {
		name: "FORTRAN Legacy"
		globs: ["*.f", "*.for", "*.ftn", "*.f77", "*.pfo"]
	}
	FortranModern: {
		name: "FORTRAN Modern"
		globs: ["*.f03", "*.f08", "*.f90", "*.f95"]
	}
	FreeMarker: globs: ["*.ftl", "*.ftlh", "*.ftlx"]
	FSharp: {
		name: "F#"
		globs: ["*.fs", "*.fsi", "*.fsx", "*.fsscript"]
	}
	Fstar: {
		name: "F*"
		globs: ["*.fst"]
	}
	GDB: {
		name: "GDB Script"
		globs: ["*.gdb"]
	}
	GdScript: {
		name: "GDScript"
		globs: ["*.gd"]
	}
	Gherkin: {
		name: "Gherkin (Cucumber)"
		globs: ["*.feature"]
	}
	Glsl: {
		name: "GLSL"
		globs: ["*.vert", "*.tesc", "*.tese", "*.geom", "*.frag", "*.comp", "*.glsl"]
	}
	Go: globs: ["*.go"]
	Graphql: {
		name: "GraphQL"
		globs: ["*.gql", "*.graphql"]
	}
	Groovy: globs: ["*.groovy", "*.grt", "*.gtpl", "*.gvy"]
	Hamlet: globs: ["*.hamlet"]
	Handlebars: globs: ["*.hbs", "*.handlebars"]
	Happy: globs: ["*.y", "*.ly"]
	Haskell: globs: ["*.hs"]
	Haxe: globs: ["*.hx"]
	Hcl: {
		name: "HCL"
		globs: ["*.hcl", "*.tf", "*.tfvars"]
	}
	Hex: {
		name: "HEX"
		globs: ["*.hex"]
	}
	Hlsl: {
		name: "HLSL"
		globs: ["*.hlsl"]
	}
	HolyC: globs: ["*.HC", "*.hc"]
	Html: {
		name: "HTML"
		globs: ["*.html", "*.htm"]
	}
	Idris: globs: ["*.idr", "*.lidr"]
	Ini: {
		name: "INI"
		globs: ["*.ini"]
	}
	IntelHex: {
		name: "Intel HEX"
		globs: ["*.ihex"]
	}
	Isabelle: globs: ["*.thy"]
	Jai: {
		name: "JAI"
		globs: ["*.jai"]
	}
	Java: globs: ["*.java"]
	JavaScript: globs: ["*.js", "*.mjs"]
	Json: {
		name: "JSON"
		globs: ["*.json"]
	}
	Jsx: {
		name: "JSX"
		globs: ["*.jsx"]
	}
	Julia: globs: ["*.jl"]
	Julius: globs: ["*.julius"]
	KakouneScript: {
		name: "Kakoune script"
		globs: ["*.kak"]
	}
	Kotlin: globs: ["*.kt", "*.kts"]
	Lean: globs: ["*.lean", "*.hlean"]
	Less: {
		name: "LESS"
		globs: ["*.less"]
	}
	LinkerScript: {
		name: "LD Script"
		globs: ["*.lds"]
	}
	Liquid: {
		name: "Liquid"
		globs: ["*.liquid"]
	}
	Lisp: globs: ["*.lisp", "*.lsp"]
	LLVM: globs: ["*.ll"]
	Logtalk: globs: ["*.lgt", "*.logtalk"]
	Lua: globs: ["*.lua"]
	Lucius: globs: ["*.lucius"]
	Madlang: globs: ["*.mad"]
	Makefile: {
		globs: ["*.makefile", "*.mak", "*.mk", "makefile"]
	}
	Markdown: globs: ["*.md", "*.markdown"]
	Meson: globs: ["meson.build", "meson_options.txt"]
	Mint: globs: ["*.mint"]
	ModuleDef: {
		name: "Module-Definition"
		globs: ["*.def"]
	}
	MoonScript: globs: ["*.moon"]
	MsBuild: {
		name: "MSBuild"
		globs: ["*.csproj", "*.vbproj", "*.fsproj", "*.props", "*.targets"]
	}
	Mustache: globs: ["*.mustache"]
	Nim: globs: ["*.nim"]
	Nix: globs: ["*.nix"]
	NotQuitePerl: {
		name: "Not Quite Perl"
		globs: ["*.nqp"]
	}
	ObjectiveC: {
		name: "Objective-C"
		globs: ["*.m"]
	}
	ObjectiveCpp: {
		name: "Objective-C++"
		globs: ["*.mm"]
	}
	OCaml: globs: ["*.ml", "*.mli", "*.re", "*.rei"]
	Odin: globs: ["*.odin"]
	Org: globs: ["*.org"]
	Oz: globs: ["*.oz"]
	Pan: globs: ["*.pan", "*.tpl"]
	Pascal: globs: ["*.pas", "*.pp"]
	Perl: {
		globs: ["*.pl", "*.pm"]
		interpreters: ["perl"]
	}
	Perl6: globs: ["*.pl6", "*.pm6"]
	Pest: globs: ["*.pest"]
	Php: {
		name: "PHP"
		globs: ["*.php"]
	}
	Polly: globs: ["*.polly"]
	Pony: globs: ["*.pony"]
	PostCss: {
		name: "PostCSS"
		globs: ["*.pcss", "*.sss"]
	}
	PowerShell: globs: ["*.ps1", "*.psm1", "*.psd1", "*.ps1xml", "*.cdxml", "*.pssc", "*.psc1"]
	Processing: globs: ["*.pde"]
	Prolog: globs: ["*.p", "*.pro"]
	Protobuf: {
		name: "Protocol Buffers"
		globs: ["*.proto"]
	}
	PSL: {
		name: "PSL Assertion"
		globs: ["*.psl"]
	}
	PureScript: globs: ["*.purs"]
	Python: {
		globs: ["*.py", "*.pyw"]
		interpreters: ["python", "python2", "python3"]
	}
	Qcl: {
		name: "QCL"
		globs: ["*.qcl"]
	}
	Qml: {
		name: "QML"
		globs: ["*.qml"]
	}
	R: globs: ["*.r"]
	Racket: globs: ["*.rkt"]
	Rakefile: {
		globs: ["*.rake", "rakefile"]
	}
	Razor: globs: ["*.cshtml"]
	Renpy: {
		name: "Ren'Py"
		globs: ["*.rpy"]
	}
	ReStructuredText: globs: ["*.rst"]
	RON: {
		name: "Rusty Object Notation"
		globs: ["*.ron"]
	}
	RPMSpecfile: {
		name: "RPM Specfile"
		globs: ["*.spec"]
	}
	Ruby: {
		globs: ["*.rb"]
		interpreters: ["ruby"]
	}
	RubyHtml: {
		name: "Ruby HTML"
		globs: ["*.rhtml"]
	}
	Rust: globs: ["*.rs"]
	Sass: globs: ["*.sass", "*.scss"]
	Scala: globs: ["*.sc", "*.scala"]
	Scheme: globs: ["*.scm", "*.ss"]
	Scons: globs: ["sconscript", "sconstruct"]
	Sh: {
		name: "Shell"
		globs: ["*.sh"]
		interpreters: ["sh"]
	}
	Sml: {
		name: "Standard ML (SML)"
		globs: ["*.sml"]
	}
	Solidity: {
		name: "Solidity"
		globs: ["*.sol"]
	}
	SpecmanE: {
		name: "Specman e"
		globs: ["*.e"]
	}
	Spice: {
		name: "Spice Netlist"
		globs: ["*.ckt"]
	}
	Sql: {
		name: "SQL"
		globs: ["*.sql"]
	}
	SRecode: {
		name: "SRecode Template"
		globs: ["*.srt"]
	}
	Stratego: {
		name: "Stratego/XT"
		globs: ["*.str"]
	}
	Svg: {
		name: "SVG"
		globs: ["*.svg"]
	}
	Swift: globs: ["*.swift"]
	Swig: {
		name: "SWIG"
		globs: ["*.swg", "*.i"]
	}
	SystemVerilog: globs: ["*.sv", "*.svh"]
	Tcl: {
		name: "TCL"
		globs: ["*.tcl"]
	}
	Tex: {
		name: "TeX"
		globs: ["*.tex", "*.sty"]
	}
	Text: {
		name: "Plain Text"
		globs: ["*.text", "*.txt"]
	}
	Thrift: globs: ["*.thrift"]
	Toml: {
		name: "TOML"
		globs: ["*.toml"]
	}
	Twig: {
		name: "Twig"
		globs: ["*.twig"]
	}
	TypeScript: globs: ["*.ts", "*.tsx"]
	UnrealDeveloperMarkdown: {
		name: "Unreal Markdown"
		globs: ["*.udn"]
	}
	UnrealPlugin: {
		name: "Unreal Plugin"
		globs: ["*.uplugin"]
	}
	UnrealProject: {
		name: "Unreal Project"
		globs: ["*.uproject"]
	}
	UnrealScript: {
		name: "Unreal Script"
		globs: ["*.uc", "*.uci", "*.upkg"]
	}
	UnrealShader: {
		name: "Unreal Shader"
		globs: ["*.usf"]
	}
	UnrealShaderHeader: {
		name: "Unreal Shader Header"
		globs: ["*.ush"]
	}
	UrWeb: {
		name: "Ur/Web"
		globs: ["*.ur", "*.urs"]
	}
	UrWebProject: {
		name: "Ur/Web Project"
		globs: ["*.urp"]
	}
	Vala: globs: ["*.vala"]
	VB6: {
		name: "VB6"
		globs: ["*.frm", "*.bas", "*.cls"]
	}
	VBScript: {
		name: "VBScript"
		globs: ["*.vbs"]
	}
	Velocity: {
		name: "Apache Velocity"
		globs: ["*.vm"]
	}
	Verilog: globs: ["*.vg", "*.vh"]
	VerilogArgsFile: {
		name: "Verilog Args File"
		globs: ["*.irunargs", "*.xrunargs"]
	}
	Vhdl: {
		name: "VHDL"
		globs: ["*.vhd", "*.vhdl"]
	}
	VimScript: {
		name: "Vim Script"
		globs: ["*.vim"]
	}
	VisualBasic: {
		name: "Visual Basic"
		globs: ["*.vb"]
	}
	VisualStudioProject: {
		name: "Visual Studio Project"
		globs: ["*.vcproj", "*.vcxproj"]
	}
	VisualStudioSolution: {
		name: "Visual Studio Solution"
		globs: ["*.sln"]
	}
	Vue: {
		name: "Vue"
		globs: ["*.vue"]
	}
	WebAssembly: globs: ["*.wat", "*.wast"]
	Wolfram: globs: ["*.nb", "*.wl"]
	Xaml: {
		name: "XAML"
		globs: ["*.xaml"]
	}
	XcodeConfig: {
		name: "Xcode Config"
		globs: ["*.xcconfig"]
	}
	Xml: {
		name: "XML"
		globs: ["*.xml"]
	}
	XSL: {
		name: "XSL"
		globs: ["*.xsl", "*.xslt"]
	}
	Xtend: globs: ["*.xtend"]
	Yaml: {
		name: "YAML"
		globs: ["*.yaml", "*.yml"]
	}
	Zig: globs: ["*.zig"]
	Zsh: {
		globs: ["*.zsh"]
		interpreters: ["zsh"]
	}
}

// Constraints

keys ::         [ strings.ToLower(k) for k, v in languages ]
globs ::        list.FlattenN([ v.globs for k, v in languages ], -1)
interpreters :: list.FlattenN([ v.interpreters for k, v in languages ], -1)

all_keys_sorted:         true & list.SortStrings(keys) == keys
all_globs_unique:        true & list.UniqueItems(globs)
all_interpreters_unique: true & list.UniqueItems(interpreters)

// Output

output: "languages": {
	for k, v in languages {
		"\(strings.ToLower(k))": v
	}
}
