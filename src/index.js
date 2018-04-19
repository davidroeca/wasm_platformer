import createImportObject from './js/createImportObject'
import gameLoop from './js/gameLoop'
import loadLib from './lib.rs'

loadLib({env: createImportObject()}).then(result => {
  const rustLib = result.instance.exports
  gameLoop(
    rustLib.update,
    rustLib.render
  )
})
