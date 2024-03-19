# Dying Light 2 Save Editor DevTool
This repository builds upon the open source [DL2 Save Editor](https://github.com/Marcel-TO/DL2_Save_Editor) and focuses only on the save analysing.

## What does the DevTool do?
Loading a save requires to scrape through thousands of hex values in order to find specific values that need to be changed. Since every save is different, moving through a save file can not be done with hardcoding the offsets or starting values alone. Thats where this devtool comes into play. The goal is to validate saves consistently and find errors easily.

## How to interact with the DevTool
In contrast to the Editor itself, the devtool will have a terminal like interface. So all interactions will be done with command prompts.
Here a quick overview of the commands that will be supported:
- `analyse-save`
- `find-item`
- `swap-id`
- `edit-values`
- `show-ids`
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

### Find item | `find-item`

| Long Arg | Short Arg | Description | Required? | Type |
| --- | --- | --- | --- | --- |
| `--path` | `-p` | The full path of the save you want to analyse. | ✔️ | String |
| `--id` | `-i` | The ID you are looking for inside your save. | ✔️ (only required if you are not looking for category) | String |
| `--category` | `-c` | The ID you are looking for inside your save. | ✔️ (only required if you are not looking for id) | String |
| `--area` | `-a` | Defines where you want to search for the item. (Inventory, Skills, etc...) |  | String |
| `--debug` | `-d` | Indicates whether each step will be manually continued by pressing enter or not. |  | Boolean |
| `--help` | `-h` | Returns the information for the command with all arguments and explanations. |  | Nothing |

**Example:**
```bash
find-item --path "C://User/Documents/SaveFile.sav" --id Token_MutationSamples --area Inventory
```
> Demonstrates how to look for the `Token_MutationSamples` inside the Inventory.

```bash
find-item --path "C://User/Documents/SaveFile.sav" --category Firearm --area Inventory 
```
> Demonstrates how to see if the inventory contains firearms.


### Swap ID | `swap-id`

| Long Arg | Short Arg | Description | Required? | Type |
| --- | --- | --- | --- | --- |
| `--path` | `-p` | The full path of the save you want to analyse. | ✔️ | String |
| `--source` | `-s` | The ID you want to replace. | ✔️ | String |
| `--target` | `-t` | The new ID you want to replace the old one with. | ✔️ | String |
| `--debug` | `-d` | Indicates whether each step will be manually continued by pressing enter or not. |  | Boolean |
| `--help` | `-h` | Returns the information for the command with all arguments and explanations. |  | Nothing |

**Example:**
```bash
swap-id --path "C://User/Documents/SaveFile.sav" --source wpn_15hs_v_gameover_Artifact --target wpn_1hs_mach_03_army --debug True
```
> Demonstrates how to swap an ID with the debugging feature.

```bash
swap-id -p "C://User/Documents/SaveFile.sav" -s wpn_15hs_v_gameover_Artifact -t wpn_1hs_mach_03_army -d True
```
> Demonstrates the same command with short args.

### Edit Values | `edit-values`

| Long Arg | Short Arg | Description | Required? | Type |
| --- | --- | --- | --- | --- |
| `--path` | `-p` | The full path of the save you want to analyse. | ✔️ | String |
| `--offset` | `-o` | The offset of the current item you want to edit. | ✔️ | String |
| `--level` | `-l` | The new level you want to change. |  | Number (u16 -> max 65535) |
| `--seed` | `-s` | The new seed you want to change. |  | Number (u16 -> max 65535) |
| `--amount` | `-a` | The new amount you want to change. To avoid issues please stick to max 9999 |  | Number (u16 -> max 4294967295) |
| `--durability` | `-dl` | The new durability you want to change. To avoid issues please stick to max 9999 |  | Number (u16 -> max 4294967295) |
| `--debug` | `-d` | Indicates whether each step will be manually continued by pressing enter or not. |  | Boolean |
| `--help` | `-h` | Returns the information for the command with all arguments and explanations. |  | Nothing |

**Example:**
```bash
edit-values --path "C://User/Documents/SaveFile.sav" --offset 1195661 --level 6543 --seed 22352
```
> Demonstrates how to edit values of an item.

### Show IDs | `show-ids`

| Long Arg | Short Arg | Description | Required? | Type |
| --- | --- | --- | --- | --- |
| `--category` | `-c` | A specific category of the IDs. (For example melee, consumables, etc) |  | String |
| `--find` | `-f` | Find a specific ID. |  | String |
| `--help` | `-h` | Returns the information for the command with all arguments and explanations. |  | Nothing |

**Example:**
```bash
show-ids --category Melee 
```
> Demonstrates how to display all IDs from the melee category.

```bash
show-ids -f wpn_15hs_v_gameover_Artifact
```
> Demonstrates if the `wpn_15hs_v_gameover_Artifact` is insde the IDs folder.

## Command Prompts
### Hex Showcase | `hex-showcase`

| Long Arg | Short Arg | Description | Required? | Type |
| --- | --- | --- | --- | --- |
| `--path` | `-p` | The full path of the save you want to analyse. | ✔️ | String |
| `--from` | `-f` | The starting offset from where the hex values should be displayed. |  | String |
| `--to` | `-t` | The end offset to where the hex values should be displayed. |  | String |
| `--help` | `-h` | Returns the information for the command with all arguments and explanations. |  | Nothing |

**Example:**
```bash
hex-showcase --path "C://User/Documents/SaveFile.sav" 
```
> Demonstrates how to view the hex values.

```bash
hex-showcase --path "C://User/Documents/SaveFile.sav" --from 119556 --to 119560 
```
> Demonstrates how to view specific byte sections
>
> ## Command Prompts
### Clear | `clear`
This command removes previous prompts and clears your terminal.
