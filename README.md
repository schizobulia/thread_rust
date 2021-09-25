# thread_rust

**thread_rust:** Create threads using rust

```
$ https://www.rust-lang.org/zh-CN/tools/install

$ npm install -g cargo-cp-artifact

$ npm i thread_rust

```

```javascript
const thread_rust = require('thread_rust')

function main() {
  thread_rust.start(()=> {
    callback()
  })
  console.log('main')
}

function callback() {
  let s = new Date().getTime()
  while ((new Date().getTime() - s) < 2000) {
  }
  console.log('thread end')
}

main()
```