# rr: refactor/rename

The `rr` command facilitates refactoring/renaming in cases when you want to rename a concept that appears in your code
rather than a specific variable or class. So, if you decide that "JS example" should be renamed to "javascript dummy",
`rr` will do it for you, aware of different code styles.

## Simple example
```
$ cat src/lib.js
const JS_EXAMPLE_STATIC_CONST = 44

function jsExampleFunction(firstJsExampleArgument, secondJsExampleArgument) {
}

module.exports = {
    jsExampleFunction
}

$ cat src/lib.js | rr 'JS example' 'javascript dummy'
const JAVASCRIPT_DUMMY_STATIC_CONST = null

function javascriptDummyFunction(firstJavascriptDummyArgument, secondJavascriptDummyArgument) {
}

module.exports = {
    javascriptDummyFunction
}
```

## Using together with the `find` command

```
$ find src -name "*.js" -exec rr "JS example" "javascript dummy" --file {} \;

$ git diff
diff --git a/src/cli.js b/src/cli.js
--- a/src/cli.js
+++ b/src/cli.js
@@ -1,3 +1,3 @@
 const lib = require('./lib.js')

-var jsExampleVariable = lib.jsExampleFunction()
+var javascriptDummyVariable = lib.javascriptDummyFunction()
diff --git a/src/lib.js b/src/lib.js
--- a/src/lib.js
+++ b/src/lib.js
@@ -1,8 +1,8 @@
-const JS_EXAMPLE_STATIC_CONST = null
+const JAVASCRIPT_DUMMY_STATIC_CONST = null

-function jsExampleFunction(firstJsExampleArgument, secondJsExampleArgument) {
+function javascriptDummyFunction(firstJavascriptDummyArgument, secondJavascriptDummyArgument) {
 }

 module.exports = {
-    jsExampleFunction
+    javascriptDummyFunction
 }
```