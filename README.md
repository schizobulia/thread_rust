# thread_rust

**thread_rust:** Create threads using rust

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