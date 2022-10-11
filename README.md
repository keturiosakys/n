# newfile (or simply n) - create files and folders quickly

Create files and folders quickly with relative paths. It's like [Sublime's AdvancedNewFile](https://github.com/SublimeText/AdvancedNewFile) plugin but for your terminal.

## Usage

```bash
n [path to file or folder]
```

### Example

Simply type the path along with the desired file name and extension. If the specified directories do not exist, `n` will create them automatically.

```bash
n the_spy/who_came_in_from_the_cold.txt
```

```
the_spy/
└── who_came_in_from_the_cold.txt
```

You can also chain multiple folders or files by simply separating them with spaces.

```bash
n project_final.txt project_final_final.txt project_really_final.txt
```

```
project_final.txt
project_final_final.txt
project_really_final.txt
```
