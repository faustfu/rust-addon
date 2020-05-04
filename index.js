let addon = require('./index.node')
console.log(addon.hello)
console.log(addon.myFunc1())
console.log(addon.myFunc2(1,2))
console.log(addon.myFunc3('ok?'))

// console.time('10000-elements)1')
// for (let i = 0; i < 10000; i++) {
//   addon.myFunc()
// }
// console.timeEnd('10000-elements)1')

// console.time('10000-elements)2')
// for (let i = 0; i < 10000; i++) {
//   myFunc()
// }
// console.timeEnd('10000-elements)2')

function myFunc() {
  return 'Hello from JS'
}
