# pluralizer-cli

Command line wrapper for the [pluralizer](https://lib.rs/crates/pluralizer) crate lib.

## usage

### single word
```shell
echo "box" | pluralizer-cli
```
will result in:
```shell
boxes
```

### multiple words
```shell
pluralizer-cli << EOF
box
fox
EOF
``` 
will result in:
```shell
boxes
foxes
```

### file

Given a file with the following contents:

```shell
food
house
fizz
way
baby
```
The command:

```shell
cat test | pluralizer-cli
```

will result in:
```shell
foods
houses
fizzes
ways
babies
```



### file append

Given a file with one word per line, the command:
```shell
cat test | pluralizer-cli | sponge -a test
```

will append the original file, test, with the pluralized words added.