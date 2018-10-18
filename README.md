# refren: refactor/rename

The `refren` command facilitates refactoring/renaming in cases when you want to rename a concept that appears in your
code rather than a specific variable or class. So, if you decide that "JS example" should be renamed to
"javascript dummy", `refren` will do it for you, aware of different code styles.

## Simple example

`
$ cat src/lib.js
`

```js
const JS_EXAMPLE_STATIC_CONST = 44

function jsExampleFunction(firstJsExampleArgument, secondJsExampleArgument) {
}

module.exports = {
    jsExampleFunction
}
```

`
$ cat src/lib.js | refren 'JS example' 'javascript dummy'
`

```js
const JAVASCRIPT_DUMMY_STATIC_CONST = null

function javascriptDummyFunction(firstJavascriptDummyArgument, secondJavascriptDummyArgument) {
}

module.exports = {
    javascriptDummyFunction
}
```

## Using together with the `find` command

`refren` can be easily used together with the `find` command:

`
$ find src -name "*.js" -exec refren "JS example" "javascript dummy" --file {} \;
`
