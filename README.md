# Paranagram Clap
A command-line application to find anagrams.

## Get it

If you want to compile it, you can just clone this repo and so compile it.

If you want to use Paranagram Clap, you can search in the [release](https://github.com/Tatounee/Paranagram_Clap/tree/main/release) directory for your platform.

You will need a dictionary file as a base to generate anagrams, you can find one in the [release](https://github.com/Tatounee/Paranagram_Clap/tree/main/release) directory.

## Setup

Copy the `.env` file in the [release](https://github.com/Tatounee/Paranagram_Clap/tree/main/release) directory and path it in the parent directory of your application.

Setup your `.env` file to set the default paths for input and output file. It recommended to use absolute path.

**Exemple :**
```
── C:
   ├── dir1
   │   ├── paranagram_clap.exe
   │   ├── input.txt
   │   └── input.out
   └── .env
```
```
$ cat .env
INPUT_PATH = C:/dir1/input.txt
OUTPUT_PATH = C:/dir1/output.txt
```

## Usage

Simply open a terminal in the same directory of your application and run `$ paranagram_clap <some_sentence>`

### Flag and Option

 - `--input=<path>` and `-i=<path>` => Overwrite the path setup in the `.env` file to choose which file will be used as a dictionary.
 - `--output=<path>` and `-o=<path>` => Overwrite the path setup in the `.env` file to choose which file will be used to write anagrams.
 - `--justwords` and `-j` => Use if you only want the words which can be created from your sentence.
 - `--debug` and `-d` => Prints advancement of the generation of anagrams

# :warning: Little PC

If you try to generate full sentences of anagrams, you will need a lot of RAM. In my PC with 16 Go I can't search for sentence more than 13 characters.
