# NIPE

Nipe is a vipe clone in rust because why not. Stands for "not vipe" because "viper" and other good names are already taken.

Nipe allows you to run your editor in the middle of a unix pipeline and edit the data that is being piped between programs. Your editor will have the full
data being piped from command1 loaded into it, and when you close it, that data will be piped into command2.

Only works for text editors which use the first argument as the file to open which should be always the case.

## Synapsis

```sh
command1 | nipe | command2
```

Will open your editor with the data from command1, and when you close it, the data will be piped into command2.
You can control which editor to use by setting the `EDITOR` environment variable.

## Suffix

If you want to use a suffix other than `.txt` for the temporary file, you can use the `--suffix` option.

```sh
command1 | nipe --suffix .html | command2
```
That will tell your editor to use the `.html` suffix instead of the default `.txt`.
