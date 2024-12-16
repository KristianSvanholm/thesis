{
    description = "Dependencies";
inputs = {
	nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
	flake-utils.url = "github:numtide/flake-utils";
	old-ocaml.url = "github:nixos/nixpkgs/54c1e44240d8a527a8f4892608c4bce5440c3ecb";
	old-dart.url = "github:nixos/nixpkgs/733b33a07eac62a01f738f4bf15aa46b4c84168b";
	#hhvm.url = "github:nixos/nixpkgs/db8a4a4ef5644652bba98243805323eb7bf10404";
    };

    outputs = { self, nixpkgs, flake-utils, ... }@inputs: 
	flake-utils.lib.eachDefaultSystem
	    (system:
		let 
		    pkgs = import nixpkgs {
			inherit system;
		    };
		in
		with pkgs;
		{
		    devShells.default = mkShell {
			buildInputs = [
			    python312 
			    libgcc # C/C++
			    rustup # Rust
			    jdk # Java
			    ghc # Haskell
			    llvmPackages_15.libllvm # Haskell
			    erlang 
			    go
			    fsharp
			    gfortran # Fortran [ifx would not work for apple]
			    jruby
			    ruby
			    nodejs_23 # JavaScript
			    sbcl # Lisp
			    lua
			    fpc # Pascal
			    perl
			    racket
			    ruby
			    swift
			    dotnet-sdk
			    typescript
	    		    #inputs.hhvm.legacyPackages.${system}.hhvm
			    #hhvm # Hack. Cant get this to work yet.
			    #gnu-smalltalk # Smalltalk
			    #graalvm-ce # Java Graal


			    # php
			    (php.buildEnv {
				extensions = ({enabled, all}: enabled ++ (with all; [
				    sysvmsg
				    shmop
				]));
			    })

			    # Pidigits
			    gmp
			    python312Packages.gmpy2
			    perl540Packages.MathGMPz

			    #libApr
			    apr

			    #pcre
			    pcre

			    #khash
			    htslib

			    #Regex
			    boost


			    ### Is not packaged for apple silicon currently, or at required legacy
			    #julia
			    #gnat # ada
			    #inputs.old-dart.legacyPackages.${system}.dart # Dart 2.7.2
			    #inputs.old-ocaml.legacyPackages.${system}.ocaml # Ocaml 4.14.1
			];
		    };
		}
	    );
}
