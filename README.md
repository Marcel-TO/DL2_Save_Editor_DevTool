# Dying Light 2 Save Editor DevTool
This repository builds upon the open source [DL2 Save Editor](https://github.com/Marcel-TO/DL2_Save_Editor) and focuses only on the save analysing.

## What does the DevTool do?
Loading a save requires to scrape through thousands of hex values in order to find specific values that need to be changed. Since every save is different, moving through a save file can not be done with hardcoding the offsets or starting values alone. Thats where this devtool comes into play. The goal is to validate saves consistently and find errors easily.

## How to interact with the DevTool
In contrast to the Editor itself, the devtool will have a terminal like interface. So all interactions will be done with command prompts.
Here a quick overview of the commands that will be supported:
- `analyse-save`
- `edit-values`
- `hex-showcase`
- `clear`

> __**Help Function**__ 💡
>
> Like in a terminal you can append a `--help` or `-h` to a command to get all informations and explanations regarding the arguments

## Command Prompts
### Analyse Save | `analyse-save`

| Long Arg | Short Arg | Description | Required? | Type |
| --- | --- | --- | --- | --- |
| `--path` | `-p` | The full path of the save you want to analyse. | ✔️ | String |
| `--debug` | `-d` | Indicates whether each step will be manually continued by pressing enter or not. |  | Boolean |
| `--help` | `-h` | Returns the information for the command with all arguments and explanations. |  | Nothing |

**Example:**
```bash
analyse-save --path "C://User/Documents/SaveFile.sav" --debug True 
```
> Demonstrates how to start analysing a savefile with the debugging feature.

```bash
analyse-save --help 
```
> Returns the information for the command with all arguments and explanations.
