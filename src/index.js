import loadLib from './lib.rs'

loadLib().then(result => {
  const { add } = result.instance.exports
  console.log('return value was', add(2, 3))
})
